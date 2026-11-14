//! Hand-computed examples and boundary checks for count calibration.

use super::*;

fn tile() -> TileId {
    TileId::new(0, 0).unwrap()
}

// Only +, -, *, / on small counts: this bounds floating-point roundoff, not sensor error.
const TOLERANCE: f64 = 1e-12;

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= TOLERANCE,
        "{actual} != {expected}"
    );
}

#[test]
fn fit_hand_fixture_matches_knots_application_and_monthly_totals() {
    let rows: Vec<Vec<u32>> = include_str!("../../tests/fixtures/count-overlap.csv")
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();
    let modis: Vec<u32> = rows.iter().map(|r| r[2]).collect();
    let viirs: Vec<u32> = rows.iter().map(|r| r[3]).collect();
    let calibration = fit(tile(), &modis, &viirs).unwrap();
    assert_eq!(calibration.tile(), tile());
    assert_eq!(calibration.sample_count(), 4);
    assert_eq!(calibration.knots().len(), 4);
    for (knot, (x, y)) in calibration
        .knots()
        .iter()
        .zip([(0, 0.0), (4, 2.0), (8, 4.0), (16, 8.0)])
    {
        assert_eq!(knot.viirs_count(), x);
        close(knot.modis_count(), y);
    }
    for row in rows {
        close(calibration.map_count(row[3]).unwrap(), f64::from(row[4]));
    }
    for line in include_str!("../../tests/fixtures/count-application.csv")
        .lines()
        .skip(1)
    {
        let row: Vec<u32> = line.split(',').map(|v| v.parse().unwrap()).collect();
        close(calibration.map_count(row[0]).unwrap(), f64::from(row[1]));
    }
}

#[test]
fn fit_tied_viirs_averages_all_targets_in_each_run() {
    let calibration = fit(tile(), &[0, 2, 4, 9, 12], &[0, 4, 4, 4, 8]).unwrap();
    close(calibration.map_count(4).unwrap(), 5.0);
    close(calibration.map_count(2).unwrap(), 2.5);
    close(calibration.map_count(6).unwrap(), 8.5);
}

#[test]
fn fit_zero_ties_preserve_zero_and_do_not_hide_positive_modis_residuals() {
    let calibration = fit(tile(), &[0, 2, 6], &[0, 0, 4]).unwrap();
    close(calibration.map_count(0).unwrap(), 0.0);
    close(calibration.map_count(2).unwrap(), 3.0);
    close(calibration.map_count(4).unwrap(), 6.0);
}

#[test]
fn fit_empty_or_unmatched_samples_are_errors() {
    assert_eq!(
        fit(tile(), &[], &[]).unwrap_err(),
        CalibrationError::EmptyOverlap
    );
    assert_eq!(
        fit(tile(), &[1], &[]).unwrap_err(),
        CalibrationError::LengthMismatch
    );
}

#[test]
fn fit_one_positive_sample_inserts_zero_and_clamps_tail() {
    let calibration = fit(tile(), &[3], &[6]).unwrap();
    assert_eq!(calibration.sample_count(), 1);
    close(calibration.map_count(0).unwrap(), 0.0);
    close(calibration.map_count(3).unwrap(), 1.5);
    close(calibration.map_count(u32::MAX).unwrap(), 3.0);
    assert_eq!(calibration.max_viirs_count(), 6);
}

#[test]
fn fit_zero_only_viirs_rejects_positive_application() {
    let calibration = fit(tile(), &[0, 2], &[0, 0]).unwrap();
    close(calibration.map_count(0).unwrap(), 0.0);
    assert_eq!(
        calibration.map_count(1),
        Err(CalibrationError::UnsupportedPositiveCount)
    );
}

#[test]
fn fit_zero_only_modis_maps_positive_viirs_to_zero() {
    let calibration = fit(tile(), &[0, 0], &[2, 8]).unwrap();
    close(calibration.map_count(u32::MAX).unwrap(), 0.0);
}

#[test]
fn fit_maximum_counts_are_not_truncated_to_tile_u16() {
    let calibration = fit(tile(), &[0, u32::MAX], &[0, u32::MAX]).unwrap();
    close(
        calibration.map_count(u32::MAX).unwrap(),
        f64::from(u32::MAX),
    );
    close(calibration.map_count(65536).unwrap(), 65536.0);
}

#[test]
fn fit_input_permutation_preserves_calibration_and_input_buffers() {
    let modis = [8, 0, 4, 2];
    let viirs = [16, 0, 8, 4];
    let first = fit(tile(), &modis, &viirs).unwrap();
    let second = fit(tile(), &[0, 2, 4, 8], &[0, 4, 8, 16]).unwrap();
    for value in 0..=32 {
        assert_eq!(
            first.map_count(value).unwrap().to_bits(),
            second.map_count(value).unwrap().to_bits()
        );
    }
    assert_eq!(modis, [8, 0, 4, 2]);
    assert_eq!(viirs, [16, 0, 8, 4]);
}

#[test]
fn fit_seeded_distributions_are_monotonic_reproducible_and_preserve_training_totals_without_ties() {
    use firecal_testkit::SeededRandom;
    const SEED: u64 = 0xC0BE_0030;
    let mut random = SeededRandom::new(SEED);
    for case in 0..2000 {
        let n = u32::try_from(random.next_below(64) + 1).unwrap();
        let mut viirs = vec![0];
        let mut modis = vec![0];
        let mut x = 0;
        for _ in 0..n {
            x += u32::try_from(random.next_below(8) + 1).unwrap();
            viirs.push(x);
            modis.push(u32::try_from(random.next_below(1024)).unwrap());
        }
        let first = fit(tile(), &modis, &viirs).unwrap();
        let expected_total: f64 = modis.iter().map(|v| f64::from(*v)).sum();
        // Positive ranks are unique and MODIS includes zero: matching quantiles
        // preserves the marginal sum exactly here, not arbitrary monthly totals.
        let mapped_total: f64 = viirs.iter().map(|v| first.map_count(*v).unwrap()).sum();
        assert!(
            (mapped_total - expected_total).abs() <= TOLERANCE,
            "seed={SEED} case={case}"
        );
        modis.reverse();
        viirs.reverse();
        let second = fit(tile(), &modis, &viirs).unwrap();
        let mut previous = 0.0;
        for count in 0..=x + 10 {
            let value = first.map_count(count).unwrap();
            assert!(value >= previous, "seed={SEED} case={case} count={count}");
            assert_eq!(
                value.to_bits(),
                second.map_count(count).unwrap().to_bits(),
                "seed={SEED} case={case}"
            );
            previous = value;
        }
        // Duplicate VIIRS counts exercise the tie policy on separate distributions.
        let tied: Vec<u32> = viirs.iter().map(|v| v / 8).collect();
        let calibration = fit(tile(), &modis, &tied).unwrap();
        let mut previous = 0.0;
        for count in 0..=x {
            if let Ok(value) = calibration.map_count(count) {
                assert!(value >= previous, "seed={SEED} tied case={case}");
                previous = value;
            }
        }
    }
}

#[test]
fn fit_held_out_months_match_hand_computed_tile_totals() {
    // Training is January-April; held-out May and June have two cells each.
    // The known synthetic sensor relationship is VIIRS = 2 * MODIS.
    let calibration = fit(tile(), &[0, 2, 4, 8], &[0, 4, 8, 16]).unwrap();
    for (month, viirs, expected_modis_total) in [(280, [2, 6], 4.0), (281, [10, 14], 12.0)] {
        let total = viirs
            .into_iter()
            .map(|value| calibration.map_count(value).unwrap())
            .sum::<f64>();
        assert!(
            (total - expected_modis_total).abs() <= TOLERANCE,
            "held-out month={month}"
        );
    }
}
