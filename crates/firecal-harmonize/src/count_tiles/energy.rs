//! Fit and apply the independent FRP plane while preserving count/sigma output.

use super::encoding::EnergyPlane;
use super::{
    AggregateIndex, CountPlan, CountTile, TileBuildError, apply_with_sigma, rounded_plane_value,
};
use crate::{EnergyCalibration, SigmaModel, fit_energy};
use firecal_types::{AggregateView, MonthIndex};

/// Fits independent summed-FRP quantiles on complete standard overlap tile-months.
///
/// The source pair, tile coverage and training cutoff match the count pipeline;
/// only the FRP values enter this calibration. No count scale factor is reused.
///
/// # Errors
/// [`TileBuildError`] for invalid files, cutoff, absent paired overlap or invalid FRP.
pub fn fit_energy_aggregates(
    plan: &CountPlan,
    aggregates: &[AggregateView<'_>],
    train_through: MonthIndex,
) -> Result<EnergyCalibration, TileBuildError> {
    plan.check_cutoff(train_through)?;
    let index = AggregateIndex::new(plan, aggregates)?;
    let mut modis = Vec::new();
    let mut viirs = Vec::new();
    for month in index
        .paired_months(plan)
        .into_iter()
        .filter(|month| *month <= train_through)
    {
        modis.extend(
            index
                .frp(plan, month, plan.modis)
                .ok_or(TileBuildError::MissingMonth(month))?,
        );
        viirs.extend(
            index
                .frp(plan, month, plan.viirs)
                .ok_or(TileBuildError::MissingMonth(month))?,
        );
    }
    if modis.is_empty() {
        return Err(TileBuildError::NoTrainingOverlap);
    }
    fit_energy(plan.tile, &modis, &viirs).map_err(TileBuildError::Energy)
}

/// Applies counts, estimated count sigma and independently calibrated FRP.
///
/// Existing MODIS files remain authoritative, including zero FRP; only absent
/// MODIS files use mapped VIIRS. Energy stores rounded, saturated summed FRP in
/// megawatts. The sigma plane remains exclusively the uncertainty of count.
///
/// # Errors
/// [`TileBuildError`] for source errors, tile mismatch or unsupported positive FRP.
pub fn apply_with_energy(
    model: &SigmaModel,
    energy: &EnergyCalibration,
    plan: &CountPlan,
    aggregates: &[AggregateView<'_>],
) -> Result<CountTile, TileBuildError> {
    if energy.tile() != plan.tile {
        return Err(TileBuildError::WrongCalibrationTile);
    }
    let mut tile = apply_with_sigma(model, plan, aggregates)?;
    let index = AggregateIndex::new(plan, aggregates)?;
    let mut values = Vec::with_capacity(tile.counts.len());
    let mut clipped = 0_u32;
    for offset in 0..plan.months {
        let month = MonthIndex::from_index(plan.first.index() + offset);
        let (raw, is_modis) = if let Some(values) = index.frp(plan, month, plan.modis) {
            (values, true)
        } else {
            (
                index
                    .frp(plan, month, plan.viirs)
                    .ok_or(TileBuildError::MissingMonth(month))?,
                false,
            )
        };
        for raw in raw {
            let value = if is_modis {
                f64::from(raw)
            } else {
                energy.map_frp(raw).map_err(TileBuildError::Energy)?
            };
            clipped = clipped.saturating_add(u32::from(value > f64::from(u16::MAX)));
            values.push(rounded_plane_value(value));
        }
    }
    tile.energy = Some(EnergyPlane {
        values,
        calibration: energy.clone(),
        clipped,
    });
    Ok(tile)
}
