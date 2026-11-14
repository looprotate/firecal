//! Encoded sigma, quadrature queries and saturation through the real count pipeline.

use super::*;
use crate::{apply_with_sigma, fit_sigma_aggregates};

#[test]
fn sigma_pipeline_encodes_floors_and_temporal_widening_without_changing_counts() {
    let p = CountPlan::new(
        plan().tile(),
        MonthIndex::from_index(276),
        2,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let files = [
        aggregate(Satellite::Terra, 276, &[(0, 0, 2), (0, 1, 4)]),
        aggregate(Satellite::SuomiNpp, 276, &[(0, 0, 4), (0, 1, 8)]),
        aggregate(Satellite::SuomiNpp, 277, &[(0, 0, 3), (0, 1, 6)]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|b| AggregateView::parse(b).unwrap())
        .collect();
    let model = fit_sigma_aggregates(&p, &views, MonthIndex::from_index(276), 1).unwrap();
    let output = apply_with_sigma(&model, &p, &views).unwrap();
    let bytes = output.encode().unwrap();
    let view = TileView::parse(&bytes).unwrap();
    // Half rounding remains count [2,3]; one month outside a one-month
    // training window doubles its base sigma: ceil(2*sqrt(1.5))=3, ceil(2*sqrt(3))=4.
    assert_eq!(
        view.value(Plane::Count, MonthIndex::from_index(277), 0, 0)
            .unwrap(),
        2
    );
    assert_eq!(
        view.value(Plane::Sigma, MonthIndex::from_index(276), 0, 0)
            .unwrap(),
        2
    );
    assert_eq!(
        view.value(Plane::Sigma, MonthIndex::from_index(277), 0, 0)
            .unwrap(),
        3
    );
    assert_eq!(
        view.value(Plane::Sigma, MonthIndex::from_index(277), 0, 1)
            .unwrap(),
        4
    );
    let area = Rectangle::new(89.75, 90.0, -180.0, -179.5).unwrap();
    let mut values = [0.0; 2];
    let mut sigma = [0.0; 2];
    series(
        &[(p.tile(), view)],
        &area,
        p.first_month(),
        Plane::Count,
        SeriesOut {
            value: &mut values,
            sigma: Some(&mut sigma),
        },
    )
    .unwrap();
    assert_eq!(values.map(f64::to_bits), [6.0_f64, 5.0].map(f64::to_bits));
    assert!((sigma[0] - 8.0_f64.sqrt()).abs() < 1e-12);
    assert!((sigma[1] - 5.0).abs() < 1e-12);
    let index = output
        .index_json(
            model.calibration(),
            MonthIndex::from_index(276),
            "2026-11-14",
        )
        .unwrap();
    assert!(index.contains("\"planes\":[\"count\",\"sigma\"]"));
    assert!(index.contains("\"real_holdout_coverage\":\"not_validated\""));
    // The count-plane bytes are unchanged from the previously checked count-only encoder.
    let previous = apply(model.calibration(), &p, &views)
        .unwrap()
        .encode()
        .unwrap();
    let count_bytes = usize::from(2_u16) * 1600 * 2;
    assert_eq!(
        &bytes[16..16 + count_bytes],
        &previous[16..16 + count_bytes]
    );
}

#[test]
fn sigma_pipeline_keeps_zero_count_tiles_when_viirs_activity_has_nonzero_uncertainty() {
    let p = CountPlan::new(
        plan().tile(),
        MonthIndex::from_index(276),
        2,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let files = [
        aggregate(Satellite::Terra, 276, &[]),
        aggregate(Satellite::SuomiNpp, 276, &[(0, 0, 4)]),
        aggregate(Satellite::SuomiNpp, 277, &[(0, 0, 4)]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|b| AggregateView::parse(b).unwrap())
        .collect();
    let model = fit_sigma_aggregates(&p, &views, p.first_month(), 1).unwrap();
    let output = apply_with_sigma(&model, &p, &views).unwrap();
    assert!(!output.is_empty());
    let bytes = output.encode().unwrap();
    let view = TileView::parse(&bytes).unwrap();
    assert_eq!(
        view.value(Plane::Count, MonthIndex::from_index(277), 0, 0)
            .unwrap(),
        0
    );
    assert_eq!(
        view.value(Plane::Sigma, MonthIndex::from_index(277), 0, 0)
            .unwrap(),
        2
    );
    assert_eq!(
        view.value(Plane::Sigma, MonthIndex::from_index(277), 0, 1)
            .unwrap(),
        0
    );
}

#[test]
fn sigma_pipeline_preserves_truly_empty_tiles_as_zero_without_creating_uncertainty() {
    let p = CountPlan::new(
        plan().tile(),
        MonthIndex::from_index(276),
        1,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let files = [
        aggregate(Satellite::Terra, 276, &[]),
        aggregate(Satellite::SuomiNpp, 276, &[]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|b| AggregateView::parse(b).unwrap())
        .collect();
    let model = fit_sigma_aggregates(&p, &views, p.first_month(), 1).unwrap();
    let output = apply_with_sigma(&model, &p, &views).unwrap();
    assert!(output.is_empty());
    let bytes = output.encode().unwrap();
    let view = TileView::parse(&bytes).unwrap();
    assert_eq!(view.value(Plane::Sigma, p.first_month(), 0, 0).unwrap(), 0);
}

#[test]
fn sigma_rounding_ceil_and_clipped_counts_saturate_without_wrap() {
    use crate::count_tiles::uncertainty::rounded_sigma;
    for (value, count, expected) in [
        (0.0, 0, 0),
        (0.01, 0, 1),
        (1.0, 0, 1),
        (1.01, 0, 2),
        (65534.01, 0, 65535),
        (1e20, 0, 65535),
        (1.0, 65535, 65535),
    ] {
        assert_eq!(rounded_sigma(value, count), expected);
    }
    let p = CountPlan::new(
        plan().tile(),
        MonthIndex::from_index(276),
        1,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let files = [
        aggregate(Satellite::Terra, 276, &[(0, 0, u32::MAX)]),
        aggregate(Satellite::SuomiNpp, 276, &[(0, 0, u32::MAX)]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|b| AggregateView::parse(b).unwrap())
        .collect();
    let model = fit_sigma_aggregates(&p, &views, p.first_month(), 1).unwrap();
    let bytes = apply_with_sigma(&model, &p, &views)
        .unwrap()
        .encode()
        .unwrap();
    assert_eq!(
        TileView::parse(&bytes)
            .unwrap()
            .value(Plane::Sigma, p.first_month(), 0, 0)
            .unwrap(),
        65535
    );
}

#[test]
fn sigma_pipeline_reports_invalid_floor_and_preserves_model_tile_binding() {
    let p = CountPlan::new(
        plan().tile(),
        MonthIndex::from_index(276),
        1,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let files = [
        aggregate(Satellite::Terra, 276, &[]),
        aggregate(Satellite::SuomiNpp, 276, &[]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|b| AggregateView::parse(b).unwrap())
        .collect();
    assert!(matches!(
        fit_sigma_aggregates(&p, &views, p.first_month(), 0),
        Err(TileBuildError::Sigma(
            crate::SigmaError::ZeroMeasurementFloor
        ))
    ));
    let model = fit_sigma_aggregates(&p, &views, p.first_month(), 1).unwrap();
    let wrong = CountPlan::new(
        TileId::new(1, 0).unwrap(),
        p.first_month(),
        1,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    assert!(matches!(
        apply_with_sigma(&model, &wrong, &views),
        Err(TileBuildError::WrongCalibrationTile)
    ));
}
