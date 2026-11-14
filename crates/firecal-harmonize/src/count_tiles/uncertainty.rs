//! Bind residual uncertainty to the existing count fit, then encode estimated sigma.

use super::encoding::SigmaPlane;
use super::{AggregateIndex, CountPlan, CountTile, TileBuildError, apply, fit_aggregates};
use crate::{OverlapCount, SigmaModel, fit_sigma};
use firecal_types::{AggregateView, CellIndex, MonthIndex, TILE_SIDE};

/// Fits the count mapping and residual model on identical complete training tile-months.
///
/// The cutoff, coverage assertions and fixed source pair have the same meaning as
/// [`fit_aggregates`]. Only paired training months contribute residuals.
///
/// # Errors
/// [`TileBuildError`] for invalid inputs or invalid uncertainty configuration.
pub fn fit_sigma_aggregates(
    plan: &CountPlan,
    aggregates: &[AggregateView<'_>],
    train_through: MonthIndex,
    floor_counts: u16,
) -> Result<SigmaModel, TileBuildError> {
    let calibration = fit_aggregates(plan, aggregates, train_through)?;
    let index = AggregateIndex::new(plan, aggregates)?;
    let mut samples = Vec::new();
    for month in index
        .paired_months(plan)
        .into_iter()
        .filter(|month| *month <= train_through)
    {
        let modis = index
            .counts(plan, month, plan.modis)
            .ok_or(TileBuildError::MissingMonth(month))?;
        let viirs = index
            .counts(plan, month, plan.viirs)
            .ok_or(TileBuildError::MissingMonth(month))?;
        for (offset, (modis_count, viirs_count)) in (0_u32..).zip(modis.into_iter().zip(viirs)) {
            let cell = CellIndex::new(
                plan.tile.row() * TILE_SIDE + offset / TILE_SIDE,
                plan.tile.col() * TILE_SIDE + offset % TILE_SIDE,
            )
            .map_err(TileBuildError::Types)?;
            samples.push(OverlapCount {
                month,
                cell,
                modis_count,
                viirs_count,
            });
        }
    }
    fit_sigma(&calibration, &samples, floor_counts).map_err(TileBuildError::Sigma)
}

/// Applies a count mapping with estimated sigma bound to that same mapping.
///
/// MODIS cells use their measurement proxy; mapped VIIRS cells use residual RMS
/// and temporal/range widening. Counts are identical to [`apply`]. Sigma rounds
/// upward, saturating at 65535; a clipped count always carries saturated sigma.
///
/// # Errors
/// [`TileBuildError`] for invalid inputs, mismatched tile or unsupported VIIRS counts.
pub fn apply_with_sigma(
    model: &SigmaModel,
    plan: &CountPlan,
    aggregates: &[AggregateView<'_>],
) -> Result<CountTile, TileBuildError> {
    let mut tile = apply(model.calibration(), plan, aggregates)?;
    let index = AggregateIndex::new(plan, aggregates)?;
    let mut counts = tile.counts.iter();
    let mut sigmas = Vec::with_capacity(tile.counts.len());
    for offset in 0..plan.months {
        let month = MonthIndex::from_index(plan.first.index() + offset);
        let (raw, is_modis) = if let Some(values) = index.counts(plan, month, plan.modis) {
            (values, true)
        } else {
            (
                index
                    .counts(plan, month, plan.viirs)
                    .ok_or(TileBuildError::MissingMonth(month))?,
                false,
            )
        };
        for value in raw {
            let encoded = counts.next().ok_or(TileBuildError::Types(
                firecal_types::Error::PlaneLengthMismatch,
            ))?;
            let sigma = if is_modis {
                model.modis_sigma(value)
            } else {
                model
                    .viirs_sigma(month, value)
                    .map_err(TileBuildError::Sigma)?
            };
            sigmas.push(rounded_sigma(sigma, *encoded));
        }
    }
    tile.sigma = Some(SigmaPlane {
        values: sigmas,
        model: model.clone(),
    });
    Ok(tile)
}

pub(super) fn rounded_sigma(value: f64, count: u16) -> u16 {
    if count == u16::MAX {
        return u16::MAX;
    }
    let mut low = 0_u16;
    let mut high = u16::MAX;
    while low < high {
        let middle = low + (high - low) / 2;
        if value <= f64::from(middle) {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    low
}
