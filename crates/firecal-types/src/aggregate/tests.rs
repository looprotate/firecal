use super::{AGGREGATE_MAGIC, AggregateRecord, AggregateView, aggregate_len, write_aggregate};
use crate::{CellIndex, Error, MonthIndex, Satellite};
use firecal_testkit::SeededRandom;

fn record(cell: u32, count: u32, frp_megawatts: f32, day_count: u32) -> AggregateRecord {
    AggregateRecord {
        cell: CellIndex::new(cell / 1440, cell % 1440).unwrap(),
        count,
        frp_megawatts,
        day_count,
    }
}

fn december_2000() -> MonthIndex {
    MonthIndex::new(2000, 12).unwrap()
}

fn encode(records: &[AggregateRecord]) -> Vec<u8> {
    let mut out = vec![0; aggregate_len(records.len()).unwrap()];
    let written = write_aggregate(Satellite::Aqua, december_2000(), records, &mut out).unwrap();
    assert_eq!(written, out.len());
    out
}

#[test]
fn one_record_has_exactly_the_bytes_written_by_hand() {
    // Aqua = 1; December 2000 = index 11; cell (1, 1) = 1441 = 0x05a1; 12.5 = 0x41480000.
    let expected: [u8; 28] = [
        0x46, 0x43, 0x41, 0x31, // FCA1
        0x01, 0x00, // satellite, reserved
        0x0b, 0x00, // month 11
        0x01, 0x00, 0x00, 0x00, // one record
        0xa1, 0x05, 0x00, 0x00, // cell 1441
        0x03, 0x00, 0x00, 0x00, // count 3
        0x00, 0x00, 0x48, 0x41, // 12.5 MW
        0x02, 0x00, 0x00, 0x00, // 2 in daylight
    ];
    assert_eq!(encode(&[record(1441, 3, 12.5, 2)]), expected);
    assert_eq!(&expected[..4], &AGGREGATE_MAGIC);
}

#[test]
fn an_empty_aggregate_is_just_the_header() {
    let bytes = encode(&[]);
    assert_eq!(bytes.len(), 12);
    let view = AggregateView::parse(&bytes).unwrap();
    assert!(view.is_empty());
    assert_eq!(view.len(), 0);
    assert_eq!(view.records().count(), 0);
}

#[test]
fn records_round_trip_with_their_satellite_and_month() {
    let records = [
        record(0, 1, 0.0, 0),
        record(1441, 3, 12.5, 2),
        record(720 * 1440 - 1, u32::MAX, 1.0e6, u32::MAX),
    ];
    let bytes = encode(&records);
    let view = AggregateView::parse(&bytes).unwrap();
    assert_eq!(view.satellite(), Satellite::Aqua);
    assert_eq!(view.month(), december_2000());
    assert_eq!(view.len(), 3);
    assert_eq!(view.records().collect::<Vec<_>>(), records);
}

#[test]
fn random_sorted_records_round_trip() {
    let seed = 20_261_113;
    let mut random = SeededRandom::new(seed);
    for _ in 0..200 {
        let mut cell = 0_u64;
        let mut records = Vec::new();
        for _ in 0..random.next_below(60) {
            cell += random.next_below(5_000) + 1;
            if cell >= 720 * 1440 {
                break;
            }
            let count = u32::try_from(random.next_below(1_000)).unwrap();
            let day_count = u32::try_from(random.next_below(u64::from(count) + 1)).unwrap();
            let megawatts = f32::from(u16::try_from(random.next_below(60_000)).unwrap()) / 8.0;
            records.push(record(
                u32::try_from(cell).unwrap(),
                count,
                megawatts,
                day_count,
            ));
        }
        let bytes = encode(&records);
        let view = AggregateView::parse(&bytes).unwrap();
        assert_eq!(view.records().collect::<Vec<_>>(), records, "seed {seed}");
    }
}

#[test]
fn unsorted_or_repeated_cells_are_refused_on_write() {
    let mut out = [0; 64];
    for records in [
        [record(5, 1, 1.0, 0), record(4, 1, 1.0, 0)],
        [record(5, 1, 1.0, 0), record(5, 1, 1.0, 0)],
    ] {
        let result = write_aggregate(Satellite::Terra, december_2000(), &records, &mut out);
        assert_eq!(result, Err(Error::RecordsNotSorted));
    }
}

#[test]
fn impossible_records_are_refused_on_write() {
    let mut out = [0; 64];
    for bad in [
        record(1, 1, -1.0, 0),
        record(1, 1, f32::NAN, 0),
        record(1, 1, f32::INFINITY, 0),
        record(1, 1, 1.0, 2),
    ] {
        let result = write_aggregate(Satellite::Terra, december_2000(), &[bad], &mut out);
        assert_eq!(result, Err(Error::RecordInvalid));
    }
}

#[test]
fn a_buffer_that_is_too_small_is_refused() {
    let mut out = [0; 27];
    let result = write_aggregate(
        Satellite::Aqua,
        december_2000(),
        &[record(1441, 3, 12.5, 2)],
        &mut out,
    );
    assert_eq!(result, Err(Error::BufferTooSmall));
}

#[test]
fn aggregate_len_is_twelve_plus_sixteen_per_record() {
    assert_eq!(aggregate_len(0), Some(12));
    assert_eq!(aggregate_len(3), Some(60));
    assert_eq!(aggregate_len(usize::MAX), None);
}

#[test]
fn a_wrong_magic_is_refused() {
    let mut bytes = encode(&[]);
    bytes[0] = b'X';
    assert_eq!(AggregateView::parse(&bytes).unwrap_err(), Error::BadMagic);
}

#[test]
fn a_short_header_or_body_is_truncated() {
    let bytes = encode(&[record(1441, 3, 12.5, 2)]);
    assert_eq!(
        AggregateView::parse(&bytes[..11]).unwrap_err(),
        Error::Truncated
    );
    assert_eq!(
        AggregateView::parse(&bytes[..bytes.len() - 1]).unwrap_err(),
        Error::Truncated
    );
}

#[test]
fn bytes_after_the_last_record_are_refused() {
    let mut bytes = encode(&[record(1441, 3, 12.5, 2)]);
    bytes.push(0);
    assert_eq!(
        AggregateView::parse(&bytes).unwrap_err(),
        Error::TrailingBytes
    );
}

#[test]
fn an_unknown_satellite_or_a_dirty_reserved_byte_is_refused() {
    let mut bytes = encode(&[]);
    bytes[4] = 9;
    assert_eq!(
        AggregateView::parse(&bytes).unwrap_err(),
        Error::UnknownSatellite
    );
    let mut bytes = encode(&[]);
    bytes[5] = 1;
    assert_eq!(
        AggregateView::parse(&bytes).unwrap_err(),
        Error::ReservedNotZero
    );
}

#[test]
fn a_cell_beyond_the_grid_is_refused_on_read() {
    let mut bytes = encode(&[record(1441, 3, 12.5, 2)]);
    bytes[12..16].copy_from_slice(&(720_u32 * 1440).to_le_bytes());
    assert_eq!(
        AggregateView::parse(&bytes).unwrap_err(),
        Error::CellOutsideGrid
    );
}

#[test]
fn unsorted_cells_and_bad_numbers_are_refused_on_read() {
    let mut bytes = encode(&[record(5, 1, 1.0, 0), record(6, 1, 1.0, 0)]);
    bytes[12..16].copy_from_slice(&9_u32.to_le_bytes());
    assert_eq!(
        AggregateView::parse(&bytes).unwrap_err(),
        Error::RecordsNotSorted
    );
    let mut bytes = encode(&[record(5, 1, 1.0, 0)]);
    bytes[20..24].copy_from_slice(&f32::NAN.to_le_bytes());
    assert_eq!(
        AggregateView::parse(&bytes).unwrap_err(),
        Error::RecordInvalid
    );
}

#[test]
fn a_count_that_promises_more_records_than_the_file_has_is_truncated() {
    let mut bytes = encode(&[]);
    bytes[8..12].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(AggregateView::parse(&bytes).unwrap_err(), Error::Truncated);
}
