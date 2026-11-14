//! Hand-calculated residuals and whole-month held-out reference coverage.

use super::*;
use firecal_types::{CellIndex, MonthIndex, TileId};

fn samples() -> Vec<OverlapCount> {
    [276, 278]
        .into_iter()
        .flat_map(|month| {
            [0, 4]
                .into_iter()
                .zip(0_u32..)
                .map(move |(modis_count, col)| OverlapCount {
                    month: MonthIndex::from_index(month),
                    cell: CellIndex::new(0, col).unwrap(),
                    modis_count,
                    viirs_count: 4,
                })
        })
        .collect()
}
fn model() -> SigmaModel {
    let calibration = crate::fit(TileId::new(0, 0).unwrap(), &[0, 4, 0, 4], &[4, 4, 4, 4]).unwrap();
    fit_sigma(&calibration, &samples(), 1).unwrap()
}
fn close(got: f64, want: f64) {
    assert!((got - want).abs() <= 1e-12, "{got} != {want}");
}

#[test]
fn sigma_hand_residual_rms_and_temporal_extrapolation_match_arithmetic() {
    let model = model();
    // The fitted target is 2. Residuals +2,-2,+2,-2 give RMS sqrt(16/4)=2.
    close(model.residual_rms(), 2.0);
    assert_eq!(model.first_month(), MonthIndex::from_index(276));
    assert_eq!(model.last_month(), MonthIndex::from_index(278));
    assert_eq!(model.measurement_floor(), 1);
    close(
        model.viirs_sigma(MonthIndex::from_index(277), 4).unwrap(),
        2.0,
    );
    // Training span is three calendar months; distance 3 doubles sigma.
    close(
        model.viirs_sigma(MonthIndex::from_index(281), 4).unwrap(),
        4.0,
    );
    close(
        model.viirs_sigma(MonthIndex::from_index(273), 4).unwrap(),
        4.0,
    );
    // At VIIRS 8 the clamped count stays 2 but magnitude widening doubles sigma.
    close(
        model.viirs_sigma(MonthIndex::from_index(277), 8).unwrap(),
        4.0,
    );
}

#[test]
fn sigma_held_out_whole_months_cover_19_of_20_modis_references_at_two_sigma() {
    let model = model();
    // These months are absent from training; the values are declared before fitting.
    // One deliberate outlier (12) must remain outside the estimated band.
    let held_out = [
        (277, [0_u32, 1, 2, 3, 4, 0, 1, 2, 3, 4]),
        (279, [0, 1, 2, 3, 4, 0, 1, 2, 3, 12]),
    ];
    let mut covered = 0;
    for (month, truths) in held_out {
        for truth in truths {
            let predicted = model.calibration().map_count(4).unwrap();
            let sigma = model.viirs_sigma(MonthIndex::from_index(month), 4).unwrap();
            covered += u32::from((predicted - f64::from(truth)).abs() <= 2.0 * sigma);
        }
    }
    assert_eq!(
        covered, 19,
        "synthetic held-out 2-sigma reference coverage must be 19/20 (95%)"
    );
}

#[test]
fn sigma_modis_uses_counting_proxy_with_nonzero_floor() {
    let model = model();
    close(model.modis_sigma(0), 0.0);
    close(model.modis_sigma(9), 3.0);
    let calibration = crate::fit(TileId::new(0, 0).unwrap(), &[0, 4, 0, 4], &[4, 4, 4, 4]).unwrap();
    let raised = fit_sigma(&calibration, &samples(), 5).unwrap();
    close(raised.modis_sigma(9), 5.0);
    close(
        raised.viirs_sigma(MonthIndex::from_index(277), 4).unwrap(),
        5.0,
    );
}

#[test]
fn sigma_rejects_empty_mismatched_unsorted_or_nonoverlap_training() {
    let calibration = crate::fit(TileId::new(0, 0).unwrap(), &[0, 4, 0, 4], &[4, 4, 4, 4]).unwrap();
    assert!(matches!(
        fit_sigma(&calibration, &samples(), 0),
        Err(SigmaError::ZeroMeasurementFloor)
    ));
    assert!(matches!(
        fit_sigma(&calibration, &[], 1),
        Err(SigmaError::EmptyOverlap)
    ));
    assert!(matches!(
        fit_sigma(&calibration, &samples()[..1], 1),
        Err(SigmaError::SampleCountMismatch)
    ));
    let mut bad = samples();
    bad.swap(0, 1);
    assert!(matches!(
        fit_sigma(&calibration, &bad, 1),
        Err(SigmaError::SamplesNotOrdered)
    ));
    let mut bad = samples();
    bad[0].cell = CellIndex::new(40, 0).unwrap();
    assert!(matches!(
        fit_sigma(&calibration, &bad, 1),
        Err(SigmaError::CellOutsideCalibrationTile)
    ));
    let mut bad = samples();
    bad[0].month = MonthIndex::from_index(143);
    assert!(matches!(
        fit_sigma(&calibration, &bad, 1),
        Err(SigmaError::BeforeOverlap)
    ));
}

#[test]
fn sigma_zero_only_training_does_not_fabricate_positive_viirs_support() {
    let calibration = crate::fit(TileId::new(0, 0).unwrap(), &[0], &[0]).unwrap();
    let sample = OverlapCount {
        month: MonthIndex::from_index(276),
        cell: CellIndex::new(0, 0).unwrap(),
        modis_count: 0,
        viirs_count: 0,
    };
    let model = fit_sigma(&calibration, &[sample], 1).unwrap();
    assert!(matches!(
        model.viirs_sigma(MonthIndex::from_index(277), 1),
        Err(SigmaError::Calibration(
            CalibrationError::UnsupportedPositiveCount
        ))
    ));
    close(
        model.viirs_sigma(MonthIndex::from_index(277), 0).unwrap(),
        0.0,
    );
}

#[test]
fn sigma_rms_retains_systematic_zero_mapping_bias() {
    let calibration = crate::fit(TileId::new(0, 0).unwrap(), &[2], &[0]).unwrap();
    let sample = OverlapCount {
        month: MonthIndex::from_index(276),
        cell: CellIndex::new(0, 0).unwrap(),
        modis_count: 2,
        viirs_count: 0,
    };
    let model = fit_sigma(&calibration, &[sample], 1).unwrap();
    // Subtracting the mean residual would erase this error entirely.
    close(model.residual_rms(), 2.0);
    close(
        model.viirs_sigma(MonthIndex::from_index(276), 0).unwrap(),
        2.0,
    );
}

#[test]
fn sigma_seeded_models_are_finite_and_widen_monotonically_outside_training() {
    use firecal_testkit::SeededRandom;
    const SEED: u64 = 0xC0BE_0032;
    let mut random = SeededRandom::new(SEED);
    for case in 0..1000 {
        let mut observations = Vec::new();
        for month in [276, 278] {
            for col in 0..8 {
                observations.push(OverlapCount {
                    month: MonthIndex::from_index(month),
                    cell: CellIndex::new(0, col).unwrap(),
                    modis_count: u32::try_from(random.next_below(10000)).unwrap(),
                    viirs_count: u32::try_from(random.next_below(10000) + 1).unwrap(),
                });
            }
        }
        let modis: Vec<_> = observations.iter().map(|s| s.modis_count).collect();
        let viirs: Vec<_> = observations.iter().map(|s| s.viirs_count).collect();
        let calibration = crate::fit(TileId::new(0, 0).unwrap(), &modis, &viirs).unwrap();
        let model = fit_sigma(&calibration, &observations, 1).unwrap();
        let maximum = calibration.max_viirs_count();
        let mut previous = 0.0;
        for month in 278..290 {
            let value = model
                .viirs_sigma(MonthIndex::from_index(month), maximum)
                .unwrap();
            assert!(
                value.is_finite() && value >= previous,
                "seed={SEED} case={case}"
            );
            previous = value;
        }
        let base = model
            .viirs_sigma(MonthIndex::from_index(278), maximum)
            .unwrap();
        let extreme = model
            .viirs_sigma(MonthIndex::from_index(278), u32::MAX)
            .unwrap();
        assert!(
            extreme.is_finite() && extreme >= base,
            "seed={SEED} case={case}"
        );
    }
}

#[test]
fn sigma_residual_fitting_rejects_unsupported_positive_observations() {
    let calibration = crate::fit(TileId::new(0, 0).unwrap(), &[0], &[0]).unwrap();
    let sample = OverlapCount {
        month: MonthIndex::from_index(276),
        cell: CellIndex::new(0, 0).unwrap(),
        modis_count: 0,
        viirs_count: 1,
    };
    assert!(matches!(
        fit_sigma(&calibration, &[sample], 1),
        Err(SigmaError::Calibration(
            CalibrationError::UnsupportedPositiveCount
        ))
    ));
}
