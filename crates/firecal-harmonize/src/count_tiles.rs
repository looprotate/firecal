//! Development count tiles with optional estimated-sigma and FRP planes. Coverage of every input tile-month must be known complete.

use crate::{Calibration, CalibrationError, fit};
use firecal_types::{AggregateView, MonthIndex, Satellite, TILE_CELLS, TileId};

mod aggregate_index;
use aggregate_index::AggregateIndex;
mod uncertainty;
pub use uncertainty::{apply_with_sigma, fit_sigma_aggregates};
mod energy;
pub use energy::{apply_with_energy, fit_energy_aggregates};
mod encoding;
mod metadata;
pub use encoding::CountTile;

/// Why a tile build cannot proceed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileBuildError {
    /// A shared file-format or month-range validation failed.
    Types(firecal_types::Error),
    /// The calibration cannot fit or map a count.
    Calibration(CalibrationError),
    /// Residual uncertainty fitting or application failed.
    Sigma(crate::SigmaError),
    /// FRP calibration fitting or application failed.
    Energy(crate::EnergyError),
    /// Sources must be a MODIS satellite followed by a VIIRS satellite.
    InvalidSourcePair,
    /// Multiple files describe the same satellite-month.
    DuplicateAggregate,
    /// An aggregate is not from the selected fixed pair.
    UnexpectedSatellite,
    /// An input month lies outside the requested output range.
    MonthOutsideRange,
    /// Neither selected source is available in this month; absence is not zero.
    MissingMonth(MonthIndex),
    /// The supplied calibration belongs to another tile.
    WrongCalibrationTile,
    /// No complete paired overlap month in 2012 onward meets the training cutoff.
    NoTrainingOverlap,
    /// The cutoff lies outside the requested input/output range.
    TrainingCutoffOutsideRange,
    /// The build date must be a valid Gregorian YYYY-MM-DD.
    InvalidBuildDate,
}

/// Output extent and fixed satellite pair for one completely covered 10° tile.
///
/// This is an assertion of external coverage, not proof inferred from FCA1 records.
/// Country-clipped inputs cannot be used to assert complete intersecting tiles.
#[derive(Debug, Clone, Copy)]
pub struct CountPlan {
    tile: TileId,
    first: MonthIndex,
    months: u16,
    modis: Satellite,
    viirs: Satellite,
}

impl CountPlan {
    /// Validates a nonempty representable month range and a MODIS/VIIRS source pair.
    ///
    /// # Errors
    /// [`TileBuildError::InvalidSourcePair`] or [`TileBuildError::Types`] for a bad range.
    pub fn new(
        tile: TileId,
        first: MonthIndex,
        months: u16,
        modis: Satellite,
        viirs: Satellite,
    ) -> Result<Self, TileBuildError> {
        if modis.is_viirs() || !viirs.is_viirs() {
            return Err(TileBuildError::InvalidSourcePair);
        }
        let last = months
            .checked_sub(1)
            .ok_or(TileBuildError::Types(firecal_types::Error::EmptyMonthRange))?;
        first
            .index()
            .checked_add(last)
            .ok_or(TileBuildError::Types(
                firecal_types::Error::MonthRangeTooLong,
            ))?;
        Ok(Self {
            tile,
            first,
            months,
            modis,
            viirs,
        })
    }
    fn check_cutoff(&self, cutoff: MonthIndex) -> Result<(), TileBuildError> {
        if cutoff
            .index()
            .checked_sub(self.first.index())
            .is_none_or(|offset| offset >= self.months)
        {
            return Err(TileBuildError::TrainingCutoffOutsideRange);
        }
        Ok(())
    }
    /// The tile covered by the output.
    #[must_use]
    pub const fn tile(self) -> TileId {
        self.tile
    }
    /// The first output month.
    #[must_use]
    pub const fn first_month(self) -> MonthIndex {
        self.first
    }
}

/// Fits on paired complete tile-months from 2012 onward through `train_through`, inclusive.
///
/// No missing file is converted to zeros. An explicit empty FCA1 file is a known-zero month.
/// The caller must hold out other months for validation and assert science-quality coverage.
///
/// # Errors
/// Input errors in [`TileBuildError`], including duplicate files, no paired training
/// overlap and the underlying calibration error.
pub fn fit_aggregates(
    plan: &CountPlan,
    aggregates: &[AggregateView<'_>],
    train_through: MonthIndex,
) -> Result<Calibration, TileBuildError> {
    plan.check_cutoff(train_through)?;
    let index = AggregateIndex::new(plan, aggregates)?;
    let mut modis = Vec::new();
    let mut viirs = Vec::new();
    for offset in 0..plan.months {
        let month = MonthIndex::from_index(plan.first.index() + offset);
        if month.index() < 144 || month > train_through {
            continue;
        }
        if let (Some(m), Some(v)) = (
            index.counts(plan, month, plan.modis),
            index.counts(plan, month, plan.viirs),
        ) {
            modis.extend(m);
            viirs.extend(v);
        }
    }
    if modis.is_empty() {
        return Err(TileBuildError::NoTrainingOverlap);
    }
    fit(plan.tile, &modis, &viirs).map_err(TileBuildError::Calibration)
}

/// Applies a fitted count calibration to complete source tile-months.
///
/// Uses MODIS whenever its file exists, including its zero cells; only a missing
/// MODIS file allows mapped VIIRS. Never adds two satellites or fills unknown months.
/// The result is count-only development output; sigma/energy slots are unavailable.
///
/// # Errors
/// [`TileBuildError`] for invalid source files, unknown months, a different calibration
/// tile, or an unsupported positive VIIRS count.
pub fn apply(
    calibration: &Calibration,
    plan: &CountPlan,
    aggregates: &[AggregateView<'_>],
) -> Result<CountTile, TileBuildError> {
    if calibration.tile() != plan.tile {
        return Err(TileBuildError::WrongCalibrationTile);
    }
    let index = AggregateIndex::new(plan, aggregates)?;
    let mut counts = Vec::with_capacity(usize::from(plan.months) * TILE_CELLS);
    for offset in 0..plan.months {
        let month = MonthIndex::from_index(plan.first.index() + offset);
        if let Some(modis) = index.counts(plan, month, plan.modis) {
            counts.extend(
                modis
                    .into_iter()
                    .map(|value| rounded_plane_value(f64::from(value))),
            );
        } else if let Some(viirs) = index.counts(plan, month, plan.viirs) {
            for value in viirs {
                counts.push(rounded_plane_value(
                    calibration
                        .map_count(value)
                        .map_err(TileBuildError::Calibration)?,
                ));
            }
        } else {
            return Err(TileBuildError::MissingMonth(month));
        }
    }
    Ok(CountTile {
        plan: *plan,
        counts,
        paired_months: index.paired_months(plan),
        sigma: None,
        energy: None,
    })
}

fn rounded_plane_value(value: f64) -> u16 {
    let mut low = 0_u16;
    let mut high = u16::MAX;
    while low < high {
        let middle = low + (high - low) / 2;
        if value < f64::from(middle) + 0.5 {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    low
}

#[cfg(test)]
mod tests;

mod overlap;
pub use overlap::{OverlapReport, ReportError, overlap_report};
