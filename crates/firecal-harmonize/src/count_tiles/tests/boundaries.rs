//! Tile edges, missing support and empty output metadata.

use super::*;

#[test]
fn apply_empty_tile_is_omitted_and_index_rejects_wrong_date_or_calibration() {
    let p = CountPlan::new(
        plan().tile(),
        MonthIndex::from_index(276),
        1,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let bytes = aggregate(Satellite::Terra, 276, &[]);
    let calibration = fit(p.tile(), &[0], &[0]).unwrap();
    let output = apply(&calibration, &p, &[AggregateView::parse(&bytes).unwrap()]).unwrap();
    assert!(output.is_empty());
    let index = output
        .index_json(&calibration, p.first_month(), "2026-11-14")
        .unwrap();
    assert!(index.contains("\"tiles\":[]"));
    assert!(
        output
            .index_json(&calibration, p.first_month(), "2026-02-30")
            .is_err()
    );
    let wrong = fit(TileId::new(1, 0).unwrap(), &[0], &[0]).unwrap();
    assert!(matches!(
        output.index_json(&wrong, p.first_month(), "2026-11-14"),
        Err(TileBuildError::WrongCalibrationTile)
    ));
}

#[test]
fn apply_polar_antimeridian_tile_maps_last_cell_and_ignores_other_tiles() {
    let p = CountPlan::new(
        TileId::new(17, 35).unwrap(),
        MonthIndex::from_index(u16::MAX),
        1,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let bytes = aggregate(
        Satellite::Terra,
        u16::MAX,
        &[(0, 0, 99), (680, 1400, 3), (719, 1439, 7)],
    );
    let calibration = fit(p.tile(), &[1], &[2]).unwrap();
    let output = apply(&calibration, &p, &[AggregateView::parse(&bytes).unwrap()]).unwrap();
    let bytes = output.encode().unwrap();
    let view = TileView::parse(&bytes).unwrap();
    assert_eq!(view.value(Plane::Count, p.first_month(), 0, 0).unwrap(), 3);
    assert_eq!(
        view.value(Plane::Count, p.first_month(), 39, 39).unwrap(),
        7
    );
    assert_eq!(view.value(Plane::Count, p.first_month(), 0, 1).unwrap(), 0);
}

#[test]
fn apply_reports_unsupported_positive_viirs_and_training_cutoff_outside_range() {
    let p = CountPlan::new(
        plan().tile(),
        MonthIndex::from_index(276),
        1,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let bytes = aggregate(Satellite::SuomiNpp, 276, &[(0, 0, 1)]);
    let calibration = fit(p.tile(), &[0], &[0]).unwrap();
    assert!(matches!(
        apply(&calibration, &p, &[AggregateView::parse(&bytes).unwrap()]),
        Err(TileBuildError::Calibration(
            CalibrationError::UnsupportedPositiveCount
        ))
    ));
    for cutoff in [275, 277] {
        assert!(matches!(
            fit_aggregates(&p, &[], MonthIndex::from_index(cutoff)),
            Err(TileBuildError::TrainingCutoffOutsideRange)
        ));
    }
}
