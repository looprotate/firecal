use super::{AggregateError, Aggregator};
use crate::{CsvHeader, Hotspot, ParseError};
use firecal_types::{AggregateView, CellIndex, MonthIndex, Satellite};

fn hotspot() -> Hotspot {
    Hotspot {
        latitude: -33.5,
        longitude: -70.25,
        month: MonthIndex::from_index(288),
        day: 1,
        satellite: Satellite::Terra,
        frp_megawatts: 1.5,
        daytime: true,
    }
}

fn group() -> Aggregator {
    Aggregator::new(Satellite::Terra, MonthIndex::from_index(288))
}

#[test]
fn stored_energy_rounds_to_f32_and_rejects_nonfinite_or_out_of_range_values() {
    assert_eq!(
        super::stored_energy(1.5).unwrap().to_bits(),
        1.5_f32.to_bits()
    );
    assert_eq!(
        super::stored_energy(f64::from(f32::MAX)).unwrap().to_bits(),
        f32::MAX.to_bits()
    );
    for value in [-1.0, f64::INFINITY, f64::NAN, f64::MAX] {
        assert_eq!(
            super::stored_energy(value),
            Err(AggregateError::EnergyOverflow)
        );
    }
}

fn bytes(aggregator: &Aggregator) -> Vec<u8> {
    let records = aggregator.records().unwrap();
    let mut output = vec![0; firecal_types::aggregate_len(records.len()).unwrap()];
    assert_eq!(aggregator.write(&mut output).unwrap(), output.len());
    output
}

#[test]
fn empty_group_writes_a_header_with_no_records() {
    let output = bytes(&group());
    assert_eq!(output.len(), 12);
    let view = AggregateView::parse(&output).unwrap();
    assert!(view.is_empty());
    assert_eq!(view.satellite(), Satellite::Terra);
    assert_eq!(view.month().index(), 288);
}

#[test]
fn same_cell_adds_counts_frp_and_day_detections() {
    let mut aggregator = group();
    aggregator.add(&hotspot()).unwrap();
    let mut night = hotspot();
    night.daytime = false;
    night.frp_megawatts = 2.5;
    aggregator.add(&night).unwrap();
    let records = aggregator.records().unwrap();
    assert_eq!(records.len(), 1);
    let record = &records[0];
    // (90 + 33.5)*4 = row 494; (-70.25 + 180)*4 = col 439.
    assert_eq!(record.cell, CellIndex::new(494, 439).unwrap());
    assert_eq!((record.count, record.day_count), (2, 1));
    assert_eq!(record.frp_megawatts.to_bits(), 4.0_f32.to_bits());
    let output = bytes(&aggregator);
    assert_eq!(
        AggregateView::parse(&output)
            .unwrap()
            .records()
            .collect::<Vec<_>>(),
        records
    );
}

#[test]
fn cells_are_sorted_in_output_independent_of_insertion_order() {
    let mut aggregator = group();
    for (latitude, longitude) in [(-90.0, 180.0), (90.0, -180.0), (0.0, 0.0)] {
        aggregator
            .add(&Hotspot {
                latitude,
                longitude,
                ..hotspot()
            })
            .unwrap();
    }
    let records = aggregator.records().unwrap();
    assert_eq!(
        records.iter().map(|r| r.cell.index()).collect::<Vec<_>>(),
        [0, 519_120, 1_036_799]
    );
    assert_eq!(AggregateView::parse(&bytes(&aggregator)).unwrap().len(), 3);
}

#[test]
fn foreign_month_or_satellite_does_not_modify_the_group() {
    let mut aggregator = group();
    let before = bytes(&aggregator);
    for record in [
        Hotspot {
            satellite: Satellite::Aqua,
            ..hotspot()
        },
        Hotspot {
            month: MonthIndex::from_index(289),
            ..hotspot()
        },
    ] {
        assert_eq!(
            aggregator.add(&record),
            Err(AggregateError::MismatchedGroup)
        );
        assert_eq!(bytes(&aggregator), before);
    }
}

#[test]
fn invalid_coordinates_and_energy_do_not_modify_the_group() {
    let mut aggregator = group();
    aggregator.add(&hotspot()).unwrap();
    let before = bytes(&aggregator);
    for latitude in [91.0, -91.0, f64::NAN, f64::INFINITY] {
        assert_eq!(
            aggregator.add(&Hotspot {
                latitude,
                ..hotspot()
            }),
            Err(AggregateError::InvalidCoordinate)
        );
    }
    for longitude in [181.0, -181.0, f64::NAN] {
        assert_eq!(
            aggregator.add(&Hotspot {
                longitude,
                ..hotspot()
            }),
            Err(AggregateError::InvalidCoordinate)
        );
    }
    for frp_megawatts in [-1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(
            aggregator.add(&Hotspot {
                frp_megawatts,
                ..hotspot()
            }),
            Err(AggregateError::InvalidEnergy)
        );
    }
    assert_eq!(bytes(&aggregator), before);
}

#[test]
fn count_above_u16_max_is_preserved_without_wrapping() {
    let mut aggregator = group();
    for _ in 0..65_536 {
        aggregator.add(&hotspot()).unwrap();
    }
    let records = aggregator.records().unwrap();
    assert_eq!((records[0].count, records[0].day_count), (65_536, 65_536));
    assert_eq!(records[0].frp_megawatts.to_bits(), 98_304.0_f32.to_bits());
}

#[test]
fn count_and_energy_overflow_are_errors_and_leave_previous_values_intact() {
    let mut aggregator = group();
    aggregator.add(&hotspot()).unwrap();
    let cell = CellIndex::new(494, 439).unwrap();
    aggregator.cells.get_mut(&cell).unwrap().count = u32::MAX;
    let before = bytes(&aggregator);
    assert_eq!(
        aggregator.add(&hotspot()),
        Err(AggregateError::CountOverflow)
    );
    assert_eq!(bytes(&aggregator), before);

    let mut large = group();
    let record = Hotspot {
        frp_megawatts: f64::from(f32::MAX),
        ..hotspot()
    };
    large.add(&record).unwrap();
    let before = bytes(&large);
    assert_eq!(large.add(&record), Err(AggregateError::EnergyOverflow));
    assert_eq!(bytes(&large), before);
    assert_eq!(
        group().add(&Hotspot {
            frp_megawatts: f64::MAX,
            ..hotspot()
        }),
        Err(AggregateError::EnergyOverflow)
    );
}

#[test]
fn small_write_buffer_returns_the_shared_format_error() {
    assert_eq!(
        group().write(&mut []),
        Err(AggregateError::Format(firecal_types::Error::BufferTooSmall))
    );
}

#[test]
fn frp_is_accumulated_in_f64_before_rounding_once_to_the_file_format() {
    let mut aggregator = group();
    for frp_megawatts in [16_777_216.0, 1.0, 1.0] {
        aggregator
            .add(&Hotspot {
                frp_megawatts,
                ..hotspot()
            })
            .unwrap();
    }
    assert_eq!(
        aggregator.records().unwrap()[0].frp_megawatts.to_bits(),
        16_777_218.0_f32.to_bits()
    );
}

#[test]
fn ten_row_fixture_has_hand_checked_counts_and_frp_for_each_satellite() {
    let fixture = include_str!("../../tests/fixtures/ten-hotspots.csv");
    let mut lines = fixture.lines();
    let header = CsvHeader::parse(lines.next().unwrap().as_bytes()).unwrap();
    let mut groups: Vec<_> = [
        Satellite::Terra,
        Satellite::Aqua,
        Satellite::SuomiNpp,
        Satellite::Noaa20,
        Satellite::Noaa21,
    ]
    .into_iter()
    .map(|satellite| Aggregator::new(satellite, MonthIndex::from_index(288)))
    .collect();
    let mut filtered = 0;
    for line in lines {
        match header.parse_record(line.as_bytes()) {
            Ok(record) => groups
                .iter_mut()
                .find(|g| g.satellite == record.satellite)
                .unwrap()
                .add(&record)
                .unwrap(),
            Err(ParseError::LowConfidence | ParseError::NonVegetation) => filtered += 1,
            Err(error) => panic!("Unexpected fixture error: {error:?}"),
        }
    }
    assert_eq!(filtered, 4);
    for (aggregator, (count, day_count, frp)) in groups.iter_mut().zip([
        (2, 2, 9.0_f32),
        (1, 0, 2.0),
        (1, 1, 4.0),
        (1, 0, 8.0),
        (1, 1, 16.0),
    ]) {
        let output = bytes(aggregator);
        let view = AggregateView::parse(&output).unwrap();
        assert_eq!(view.satellite(), aggregator.satellite);
        assert_eq!(view.month().index(), 288);
        let records = view.records().collect::<Vec<_>>();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].cell, CellIndex::new(494, 439).unwrap());
        assert_eq!((records[0].count, records[0].day_count), (count, day_count));
        assert_eq!(records[0].frp_megawatts.to_bits(), frp.to_bits());
        assert_eq!(records, aggregator.records().unwrap());
    }
}
