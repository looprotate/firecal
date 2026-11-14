//! Hand-computed FRP quantiles, validation and deterministic monotonicity.

use super::*;
use firecal_types::TileId;

fn tile() -> TileId {
    TileId::new(0, 0).unwrap()
}
fn close(got: f64, want: f64) {
    assert!((got - want).abs() <= 1e-12, "{got} != {want}");
}

#[test]
fn energy_hand_fixture_uses_own_scale_and_preserves_fractional_input() {
    let calibration = fit_energy(tile(), &[0.0, 20.0, 40.0], &[0.0, 5.0, 10.0]).unwrap();
    assert_eq!(calibration.tile(), tile());
    assert_eq!(calibration.sample_count(), 3);
    assert_eq!(calibration.knots().len(), 3);
    for (x, y) in [
        (0.0, 0.0),
        (1.25, 5.0),
        (2.5, 10.0),
        (5.0, 20.0),
        (7.5, 30.0),
        (10.0, 40.0),
        (20.0, 40.0),
    ] {
        close(calibration.map_frp(x).unwrap(), y);
    }
    close(calibration.knots()[1].viirs_megawatts(), 5.0);
    close(calibration.knots()[1].modis_megawatts(), 20.0);
}

#[test]
fn energy_ties_average_reference_quantiles_and_zero_is_structural() {
    let calibration = fit_energy(
        tile(),
        &[0.0, 2.0, 4.0, 9.0, 12.0],
        &[0.0, 1.0, 1.0, 1.0, 2.0],
    )
    .unwrap();
    close(calibration.map_frp(1.0).unwrap(), 5.0);
    close(calibration.map_frp(0.5).unwrap(), 2.5);
    let zero = fit_energy(tile(), &[0.0, 2.0, 6.0], &[-0.0, 0.0, 4.0]).unwrap();
    close(zero.map_frp(-0.0).unwrap(), 0.0);
    close(zero.map_frp(2.0).unwrap(), 3.0);
}

#[test]
fn energy_rejects_invalid_inputs_empty_overlap_or_unmatched_lengths() {
    assert_eq!(
        fit_energy(tile(), &[], &[]).unwrap_err(),
        EnergyError::EmptyOverlap
    );
    assert_eq!(
        fit_energy(tile(), &[1.0], &[]).unwrap_err(),
        EnergyError::LengthMismatch
    );
    for invalid in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(
            fit_energy(tile(), &[invalid], &[1.0]).unwrap_err(),
            EnergyError::InvalidMegawatts
        );
        assert_eq!(
            fit_energy(tile(), &[1.0], &[invalid]).unwrap_err(),
            EnergyError::InvalidMegawatts
        );
        let calibration = fit_energy(tile(), &[1.0], &[1.0]).unwrap();
        assert_eq!(
            calibration.map_frp(invalid),
            Err(EnergyError::InvalidMegawatts)
        );
    }
}

#[test]
fn energy_zero_only_support_and_single_sample_are_explicit() {
    let zero = fit_energy(tile(), &[0.0], &[0.0]).unwrap();
    assert_eq!(
        zero.map_frp(1.0),
        Err(EnergyError::UnsupportedPositiveEnergy)
    );
    close(zero.map_frp(0.0).unwrap(), 0.0);
    let single = fit_energy(tile(), &[3.0], &[6.0]).unwrap();
    close(single.map_frp(1.0).unwrap(), 0.5);
    close(single.map_frp(12.0).unwrap(), 3.0);
}

#[test]
fn energy_maximum_fca_values_stay_finite_before_tile_saturation() {
    let calibration = fit_energy(tile(), &[f32::MAX, f32::MAX], &[1.0, 1.0]).unwrap();
    close(calibration.map_frp(1.0).unwrap(), f64::from(f32::MAX));
    assert!(calibration.map_frp(f32::MAX).unwrap().is_finite());
    let tiny = fit_energy(tile(), &[f32::MIN_POSITIVE], &[f32::from_bits(1)]).unwrap();
    assert_eq!(
        tiny.map_frp(f32::from_bits(1)).unwrap().to_bits(),
        f64::from(f32::MIN_POSITIVE).to_bits()
    );
}

#[test]
fn energy_seeded_mappings_are_monotonic_and_input_order_independent() {
    use firecal_testkit::SeededRandom;
    const SEED: u64 = 0xC0BE_0033;
    let mut random = SeededRandom::new(SEED);
    for case in 0..1000 {
        let mut modis = vec![0.0];
        let mut viirs = vec![0.0];
        for _ in 0..32 {
            modis.push(f32::from(u16::try_from(random.next_below(1024)).unwrap()) * 0.25);
            viirs.push(f32::from(u16::try_from(random.next_below(64) + 1).unwrap()) * 0.25);
        }
        let first = fit_energy(tile(), &modis, &viirs).unwrap();
        modis.reverse();
        viirs.reverse();
        let second = fit_energy(tile(), &modis, &viirs).unwrap();
        let mut previous = 0.0;
        for i in 0..160_u16 {
            let x = f32::from(i) * 0.125;
            let value = first.map_frp(x).unwrap();
            assert!(
                value.is_finite() && value >= previous,
                "seed={SEED} case={case}"
            );
            assert_eq!(
                value.to_bits(),
                second.map_frp(x).unwrap().to_bits(),
                "seed={SEED} case={case}"
            );
            previous = value;
        }
    }
}
