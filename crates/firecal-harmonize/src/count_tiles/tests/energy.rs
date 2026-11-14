//! Independent count/FRP mappings and complete binary/query readback.

use super::*;
use crate::{
    EnergyError, apply_with_energy, apply_with_sigma, fit_energy_aggregates, fit_sigma_aggregates,
};

fn power_file(satellite: Satellite, month: u16, counts: &[u32], frp: &[f32]) -> Vec<u8> {
    assert_eq!(counts.len(), frp.len());
    let records: Vec<_> = counts
        .iter()
        .zip(frp)
        .zip(0_u32..)
        .map(|((&count, &power), col)| AggregateRecord {
            cell: CellIndex::new(0, col).unwrap(),
            count,
            frp_megawatts: power,
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

#[test]
fn energy_pipeline_fits_own_scale_and_preserves_count_sigma_bytes() {
    let p = plan();
    let files = [
        power_file(Satellite::Terra, 276, &[2, 4], &[20.0, 40.0]),
        power_file(Satellite::SuomiNpp, 276, &[4, 8], &[5.0, 10.0]),
        power_file(Satellite::Terra, 277, &[], &[]),
        power_file(Satellite::SuomiNpp, 277, &[8], &[99.0]),
        power_file(Satellite::SuomiNpp, 278, &[3, 6], &[1.25, 2.5]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|b| AggregateView::parse(b).unwrap())
        .collect();
    let sigma = fit_sigma_aggregates(&p, &views, p.first_month(), 1).unwrap();
    let energy = fit_energy_aggregates(&p, &views, p.first_month()).unwrap();
    assert_eq!(energy.sample_count(), 1600);
    let tile = apply_with_energy(&sigma, &energy, &p, &views).unwrap();
    let bytes = tile.encode().unwrap();
    let view = TileView::parse(&bytes).unwrap();
    assert_eq!(
        view.value(Plane::Energy, MonthIndex::from_index(278), 0, 0)
            .unwrap(),
        5
    );
    assert_eq!(
        view.value(Plane::Energy, MonthIndex::from_index(278), 0, 1)
            .unwrap(),
        10
    );
    let area = Rectangle::new(80.0, 90.0, -180.0, -170.0).unwrap();
    let mut values = [0.0; 3];
    series(
        &[(p.tile(), view)],
        &area,
        p.first_month(),
        Plane::Energy,
        SeriesOut {
            value: &mut values,
            sigma: None,
        },
    )
    .unwrap();
    assert_eq!(
        values.map(f64::to_bits),
        [60.0_f64, 0.0, 15.0].map(f64::to_bits)
    );
    let previous = apply_with_sigma(&sigma, &p, &views)
        .unwrap()
        .encode()
        .unwrap();
    let prefix = 16 + 2 * 3 * 1600 * 2;
    assert_eq!(&bytes[..prefix], &previous[..prefix]);
    let index = tile
        .index_json(sigma.calibration(), p.first_month(), "2026-11-14")
        .unwrap();
    assert!(index.contains("\"planes\":[\"count\",\"sigma\",\"energy\"]"));
    assert!(index.contains("\"energy_units\":\"megawatts_frp_sum\""));
    assert!(index.contains("\"sigma_for\":\"count\""));
    assert!(index.contains("\"frp-quantile-v1\""));
}

#[test]
fn energy_pipeline_rounds_saturates_and_keeps_energy_only_tiles() {
    for (power, expected) in [
        (0.25, 0),
        (0.5, 1),
        (1.5, 2),
        (65536.0, 65535),
        (f32::MAX, 65535),
    ] {
        let p = CountPlan::new(
            plan().tile(),
            MonthIndex::from_index(276),
            1,
            Satellite::Terra,
            Satellite::SuomiNpp,
        )
        .unwrap();
        // FCA1 fields are independent: preserve valid positive FRP even when count is zero.
        let files = [
            power_file(Satellite::Terra, 276, &[0], &[power]),
            power_file(Satellite::SuomiNpp, 276, &[0], &[power]),
        ];
        let views: Vec<_> = files
            .iter()
            .map(|b| AggregateView::parse(b).unwrap())
            .collect();
        let sigma = fit_sigma_aggregates(&p, &views, p.first_month(), 1).unwrap();
        let energy = fit_energy_aggregates(&p, &views, p.first_month()).unwrap();
        let tile = apply_with_energy(&sigma, &energy, &p, &views).unwrap();
        assert_eq!(tile.is_empty(), expected == 0);
        let bytes = tile.encode().unwrap();
        let view = TileView::parse(&bytes).unwrap();
        assert_eq!(
            view.value(Plane::Energy, p.first_month(), 0, 0).unwrap(),
            expected
        );
        assert_eq!(view.value(Plane::Count, p.first_month(), 0, 0).unwrap(), 0);
        assert_eq!(view.value(Plane::Sigma, p.first_month(), 0, 0).unwrap(), 0);
        if power > 65535.0 {
            assert!(
                tile.index_json(sigma.calibration(), p.first_month(), "2026-11-14")
                    .unwrap()
                    .contains("\"clipped_cell_months\":1")
            );
        }
    }
}

#[test]
fn energy_pipeline_rejects_wrong_tile_and_unsupported_positive_frp() {
    let p = CountPlan::new(
        plan().tile(),
        MonthIndex::from_index(276),
        2,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap();
    let files = [
        power_file(Satellite::Terra, 276, &[1], &[0.0]),
        power_file(Satellite::SuomiNpp, 276, &[2], &[0.0]),
        power_file(Satellite::SuomiNpp, 277, &[1], &[1.0]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|b| AggregateView::parse(b).unwrap())
        .collect();
    let sigma = fit_sigma_aggregates(&p, &views, p.first_month(), 1).unwrap();
    let energy = fit_energy_aggregates(&p, &views, p.first_month()).unwrap();
    assert!(matches!(
        apply_with_energy(&sigma, &energy, &p, &views),
        Err(TileBuildError::Energy(
            EnergyError::UnsupportedPositiveEnergy
        ))
    ));
    let wrong = crate::fit_energy(TileId::new(1, 0).unwrap(), &[1.0], &[1.0]).unwrap();
    assert!(matches!(
        apply_with_energy(&sigma, &wrong, &p, &views),
        Err(TileBuildError::WrongCalibrationTile)
    ));
}

#[test]
fn energy_training_requires_overlap_and_an_in_range_cutoff() {
    let p = plan();
    assert!(matches!(
        fit_energy_aggregates(&p, &[], p.first_month()),
        Err(TileBuildError::NoTrainingOverlap)
    ));
    assert!(matches!(
        fit_energy_aggregates(&p, &[], MonthIndex::from_index(275)),
        Err(TileBuildError::TrainingCutoffOutsideRange)
    ));
}
