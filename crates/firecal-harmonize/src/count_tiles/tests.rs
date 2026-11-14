//! Synthetic complete-tile observations with hand-computed count expectations.

use super::*;
use firecal_query::{Rectangle, SeriesOut, series};
use firecal_types::{AggregateRecord, CellIndex, Plane, TileView, aggregate_len, write_aggregate};

fn aggregate(satellite: Satellite, month: u16, values: &[(u32, u32, u32)]) -> Vec<u8> {
    let records: Vec<_> = values
        .iter()
        .map(|&(row, col, count)| AggregateRecord {
            cell: CellIndex::new(row, col).unwrap(),
            count,
            frp_megawatts: 0.0,
            day_count: 0,
        })
        .collect();
    let mut bytes = vec![0; aggregate_len(records.len()).unwrap()];
    write_aggregate(
        satellite,
        MonthIndex::from_index(month),
        &records,
        &mut bytes,
    )
    .unwrap();
    bytes
}

fn plan() -> CountPlan {
    CountPlan::new(
        TileId::new(0, 0).unwrap(),
        MonthIndex::from_index(276),
        3,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap()
}

#[test]
fn apply_round_trip_query_preserves_modis_zero_and_uses_viirs_only_for_missing_modis() {
    let files = [
        aggregate(Satellite::Terra, 276, &[(0, 0, 2), (0, 1, 4)]),
        aggregate(Satellite::SuomiNpp, 276, &[(0, 0, 4), (0, 1, 8)]),
        aggregate(Satellite::Terra, 277, &[]),
        aggregate(Satellite::SuomiNpp, 277, &[(0, 0, 8)]),
        aggregate(Satellite::SuomiNpp, 278, &[(0, 0, 3), (0, 1, 6)]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|bytes| AggregateView::parse(bytes).unwrap())
        .collect();
    let calibration = fit_aggregates(&plan(), &views, MonthIndex::from_index(276)).unwrap();
    let output = apply(&calibration, &plan(), &views).unwrap();
    let bytes = output.encode().unwrap();
    let view = TileView::parse(&bytes).unwrap();
    assert_eq!(view.first_month(), MonthIndex::from_index(276));
    assert_eq!(view.month_count(), 3);
    assert_eq!(
        view.value(Plane::Count, MonthIndex::from_index(278), 0, 0)
            .unwrap(),
        2
    );
    assert_eq!(
        view.value(Plane::Count, MonthIndex::from_index(278), 0, 1)
            .unwrap(),
        3
    );
    let area = Rectangle::new(80.0, 90.0, -180.0, -170.0).unwrap();
    let mut values = [0.0; 3];
    series(
        &[(plan().tile(), view)],
        &area,
        MonthIndex::from_index(276),
        Plane::Count,
        SeriesOut {
            value: &mut values,
            sigma: None,
        },
    )
    .unwrap();
    assert_eq!(
        values.map(f64::to_bits),
        [6.0_f64, 0.0, 5.0].map(f64::to_bits)
    );
    assert_eq!(bytes, output.encode().unwrap());
    let index = output
        .index_json(&calibration, MonthIndex::from_index(276), "2026-11-14")
        .unwrap();
    assert!(index.contains("\"development_only\":true"));
    assert!(index.contains("\"planes\":[\"count\"]"));
    assert!(index.contains("\"t_0_0.bin\""));
}

#[test]
fn apply_missing_month_and_duplicate_sources_are_errors() {
    let p = plan();
    let calibration = fit(p.tile(), &[0, 2], &[0, 4]).unwrap();
    assert!(matches!(
        apply(&calibration, &p, &[]),
        Err(TileBuildError::MissingMonth(_))
    ));
    let bytes = aggregate(Satellite::Terra, 276, &[]);
    let view = AggregateView::parse(&bytes).unwrap();
    assert!(matches!(
        apply(&calibration, &p, &[view, view]),
        Err(TileBuildError::DuplicateAggregate)
    ));
}

#[test]
fn apply_saturates_and_rounds_counts_without_u16_wrapping() {
    for (value, expected) in [
        (0.0, 0),
        (0.49, 0),
        (0.5, 1),
        (1.5, 2),
        (65534.49, 65534),
        (65534.5, 65535),
        (65535.0, 65535),
        (f64::from(u32::MAX), 65535),
    ] {
        assert_eq!(rounded_count(value), expected);
    }
    let p = CountPlan::new(
        plan().tile(),
        MonthIndex::from_index(276),
        1,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let bytes = aggregate(Satellite::Terra, 276, &[(0, 0, u32::MAX)]);
    let calibration = fit(p.tile(), &[0, 1], &[0, 2]).unwrap();
    let output = apply(&calibration, &p, &[AggregateView::parse(&bytes).unwrap()]).unwrap();
    assert_eq!(
        TileView::parse(&output.encode().unwrap())
            .unwrap()
            .value(Plane::Count, p.first_month(), 0, 0)
            .unwrap(),
        65535
    );
}

#[test]
fn plan_rejects_empty_overflowing_months_and_wrong_sensor_pairs() {
    let tile = plan().tile();
    for (first, n) in [(276, 0), (u16::MAX, 2)] {
        assert!(
            CountPlan::new(
                tile,
                MonthIndex::from_index(first),
                n,
                Satellite::Terra,
                Satellite::SuomiNpp
            )
            .is_err()
        );
    }
    assert!(
        CountPlan::new(
            tile,
            MonthIndex::from_index(276),
            1,
            Satellite::SuomiNpp,
            Satellite::Terra
        )
        .is_err()
    );
}

#[test]
fn apply_checks_calibration_tile_satellites_and_month_range() {
    let p = plan();
    let wrong_tile = fit(TileId::new(1, 0).unwrap(), &[1], &[2]).unwrap();
    assert!(matches!(
        apply(&wrong_tile, &p, &[]),
        Err(TileBuildError::WrongCalibrationTile)
    ));
    let calibration = fit(p.tile(), &[1], &[2]).unwrap();
    for (satellite, month) in [
        (Satellite::Aqua, 276),
        (Satellite::Terra, 275),
        (Satellite::Terra, 279),
    ] {
        let bytes = aggregate(satellite, month, &[]);
        assert!(apply(&calibration, &p, &[AggregateView::parse(&bytes).unwrap()]).is_err());
    }
}

#[test]
fn fit_aggregates_rejects_absent_overlap_and_excludes_pre2012_months() {
    assert!(matches!(
        fit_aggregates(&plan(), &[], MonthIndex::from_index(276)),
        Err(TileBuildError::NoTrainingOverlap)
    ));
    let p = CountPlan::new(
        plan().tile(),
        MonthIndex::from_index(143),
        1,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let files = [
        aggregate(Satellite::Terra, 143, &[(0, 0, 1)]),
        aggregate(Satellite::SuomiNpp, 143, &[(0, 0, 2)]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|b| AggregateView::parse(b).unwrap())
        .collect();
    assert!(fit_aggregates(&p, &views, MonthIndex::from_index(143)).is_err());
}

mod boundaries;
