//! Empirical residual RMS and explicit conservative widening for estimated count uncertainty.

use crate::{Calibration, CalibrationError};
use firecal_types::{CellIndex, MonthIndex, TileId};

/// A matched reference/VIIRS observation from the calibration's training population.
#[derive(Debug, Clone, Copy)]
pub struct OverlapCount {
    /// Complete standard-product month from 2012 onward.
    pub month: MonthIndex,
    /// The observed cell, inside the calibration tile.
    pub cell: CellIndex,
    /// MODIS reference count, not a measurement of burned area or physical truth.
    pub modis_count: u32,
    /// Raw VIIRS count matched to that same cell-month.
    pub viirs_count: u32,
}

/// Why an estimated uncertainty model cannot be fitted or applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigmaError {
    /// A nonzero minimum uncertainty for active cells is required.
    ZeroMeasurementFloor,
    /// No training observations exist.
    EmptyOverlap,
    /// Residuals must describe the same number of observations as the calibration.
    SampleCountMismatch,
    /// Samples must strictly increase in (month, cell), without duplicates.
    SamplesNotOrdered,
    /// A residual observation lies outside the calibrated tile.
    CellOutsideCalibrationTile,
    /// A residual observation predates the 2012 overlap.
    BeforeOverlap,
    /// The fitted mapping cannot support an observation.
    Calibration(CalibrationError),
}

/// An estimated count-error scale bound to the exact mapping used for its residuals.
///
/// RMS includes residual bias instead of subtracting it. The counting proxy and
/// widening policies are explicit heuristics, not a calibrated confidence interval.
/// Real held-out coverage must be measured before treating the estimate as validated.
#[derive(Debug, Clone)]
pub struct SigmaModel {
    calibration: Calibration,
    residual_rms: f64,
    first: MonthIndex,
    last: MonthIndex,
    floor: u16,
}

impl SigmaModel {
    /// The exact count mapping used for fitting residuals and applying uncertainty.
    #[must_use]
    pub const fn calibration(&self) -> &Calibration {
        &self.calibration
    }
    /// Root mean squared error against the matched MODIS reference, in counts.
    #[must_use]
    pub const fn residual_rms(&self) -> f64 {
        self.residual_rms
    }
    /// Earliest training month.
    #[must_use]
    pub const fn first_month(&self) -> MonthIndex {
        self.first
    }
    /// Latest training month.
    #[must_use]
    pub const fn last_month(&self) -> MonthIndex {
        self.last
    }
    /// Positive minimum uncertainty for active cells; the counting proxy may exceed it.
    #[must_use]
    pub const fn measurement_floor(&self) -> u16 {
        self.floor
    }
    /// MODIS-only error proxy: max(sqrt(raw count), active-cell floor), before rounding.
    /// A known MODIS zero has zero reference-count error; detection omission is not modeled.
    /// No harmonization residual or temporal penalty is added to direct MODIS data.
    #[must_use]
    pub fn modis_sigma(&self, count: u32) -> f64 {
        if count == 0 {
            0.0
        } else {
            f64::from(count).sqrt().max(f64::from(self.floor))
        }
    }
    /// Estimated mapped-VIIRS error scale before tile rounding/saturation.
    ///
    /// Starts at max(residual RMS, sqrt(mapped count), active-cell floor). Multiplies
    /// by `1 + distance / training_span` outside the training month interval, then
    /// by `1 + excess / max_training_viirs` above the trained VIIRS range.
    ///
    /// # Errors
    /// [`SigmaError::Calibration`] for unsupported positive VIIRS input.
    pub fn viirs_sigma(&self, month: MonthIndex, count: u32) -> Result<f64, SigmaError> {
        let mapped = self
            .calibration
            .map_count(count)
            .map_err(SigmaError::Calibration)?;
        let active_floor = if count == 0 {
            0.0
        } else {
            f64::from(self.floor)
        };
        let base = self.residual_rms.max(mapped.sqrt()).max(active_floor);
        let index = month.index();
        let distance = if index < self.first.index() {
            self.first.index() - index
        } else {
            index.saturating_sub(self.last.index())
        };
        let span = u32::from(self.last.index() - self.first.index()) + 1;
        let temporal = 1.0 + f64::from(distance) / f64::from(span);
        let maximum = self.calibration.max_viirs_count();
        let magnitude = 1.0 + f64::from(count.saturating_sub(maximum)) / f64::from(maximum.max(1));
        Ok(base * temporal * magnitude)
    }
}

/// Fits an estimated error scale from the calibration's original matched training data.
///
/// Samples must be the same population used by `fit`, sorted by (month, cell).
/// Reserve whole other months for validation: this function must not see them.
/// Computes sqrt(mean((mapped VIIRS - MODIS)^2)) in a fixed order, retaining bias.
/// `floor_counts` supplies a positive, explicitly assumed floor for active cells.
/// Known zero cells still retain any nonzero calibration residual error.
///
/// # Errors
/// [`SigmaError`] for invalid floor, absent or mismatched samples, unordered/duplicate
/// observations, wrong cells, pre-2012 months or unsupported calibration input.
pub fn fit_sigma(
    calibration: &Calibration,
    samples: &[OverlapCount],
    floor_counts: u16,
) -> Result<SigmaModel, SigmaError> {
    if floor_counts == 0 {
        return Err(SigmaError::ZeroMeasurementFloor);
    }
    let first = samples.first().ok_or(SigmaError::EmptyOverlap)?.month;
    let last = samples.last().ok_or(SigmaError::EmptyOverlap)?.month;
    if u32::try_from(samples.len()).ok() != Some(calibration.sample_count()) {
        return Err(SigmaError::SampleCountMismatch);
    }
    let mut previous = None;
    let mut squares = 0.0;
    for sample in samples {
        if TileId::of_cell(sample.cell) != calibration.tile() {
            return Err(SigmaError::CellOutsideCalibrationTile);
        }
        if sample.month.index() < 144 {
            return Err(SigmaError::BeforeOverlap);
        }
        let key = (sample.month, sample.cell);
        if previous.is_some_and(|before| before >= key) {
            return Err(SigmaError::SamplesNotOrdered);
        }
        previous = Some(key);
        let mapped = calibration
            .map_count(sample.viirs_count)
            .map_err(SigmaError::Calibration)?;
        let residual = mapped - f64::from(sample.modis_count);
        let square = residual * residual;
        squares += square;
    }
    let residual_rms = (squares / f64::from(calibration.sample_count())).sqrt();
    Ok(SigmaModel {
        calibration: calibration.clone(),
        residual_rms,
        first,
        last,
        floor: floor_counts,
    })
}

#[cfg(test)]
mod tests;
