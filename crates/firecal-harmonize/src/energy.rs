//! Independent VIIRS-to-MODIS quantile mapping for monthly summed FRP in megawatts.

use firecal_types::TileId;
use std::cmp::Ordering;

/// Why a calibration of summed fire radiative power cannot fit or map an observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergyError {
    /// The sensor slices describe different numbers of matched cell-months.
    LengthMismatch,
    /// No paired training observations exist.
    EmptyOverlap,
    /// The training population exceeds the fixed-width sample counter.
    TooManySamples,
    /// FRP must be finite and nonnegative.
    InvalidMegawatts,
    /// Positive VIIRS FRP has no positive training support.
    UnsupportedPositiveEnergy,
}

/// One empirical quantile knot for FRP; unrelated to count calibration knots.
#[derive(Debug, Clone)]
pub struct EnergyKnot {
    viirs: f64,
    modis: f64,
}
impl EnergyKnot {
    /// Raw VIIRS summed FRP per cell-month, in megawatts.
    #[must_use]
    pub const fn viirs_megawatts(&self) -> f64 {
        self.viirs
    }
    /// MODIS-equivalent summed FRP per cell-month, in megawatts.
    #[must_use]
    pub const fn modis_megawatts(&self) -> f64 {
        self.modis
    }
}

/// Independently fitted FRP quantiles for one tile and a fixed source pair.
///
/// FCA1 inputs are f32; all fitting, accumulation and interpolation uses f64.
/// This is summed radiative power in megawatts, not time-integrated energy.
#[derive(Debug, Clone)]
pub struct EnergyCalibration {
    tile: TileId,
    samples: u32,
    knots: Vec<EnergyKnot>,
}
impl EnergyCalibration {
    /// Tile whose matched cell-months supplied the training population.
    #[must_use]
    pub const fn tile(&self) -> TileId {
        self.tile
    }
    /// Matched training cell-month count, including known zeros.
    #[must_use]
    pub const fn sample_count(&self) -> u32 {
        self.samples
    }
    /// Ordered raw-FRP knots and their nondecreasing MODIS-equivalent targets.
    #[must_use]
    pub fn knots(&self) -> &[EnergyKnot] {
        &self.knots
    }
    /// Interpolates summed VIIRS FRP before tile rounding/saturation.
    /// Zero is structural; values beyond the largest training knot clamp to its target.
    ///
    /// # Errors
    /// [`EnergyError::InvalidMegawatts`] or [`EnergyError::UnsupportedPositiveEnergy`].
    pub fn map_frp(&self, value: f32) -> Result<f64, EnergyError> {
        let value = megawatts(value)?;
        if value <= 0.0 {
            return Ok(0.0);
        }
        if self.knots.last().is_none_or(|knot| knot.viirs <= 0.0) {
            return Err(EnergyError::UnsupportedPositiveEnergy);
        }
        let mut lower_x = 0.0;
        let mut lower_y = 0.0;
        for knot in &self.knots {
            match value.total_cmp(&knot.viirs) {
                Ordering::Equal => return Ok(knot.modis),
                Ordering::Less => {
                    let fraction = (value - lower_x) / (knot.viirs - lower_x);
                    let increment = fraction * (knot.modis - lower_y);
                    return Ok(lower_y + increment);
                }
                Ordering::Greater => {
                    lower_x = knot.viirs;
                    lower_y = knot.modis;
                }
            }
        }
        Ok(lower_y)
    }
}

fn megawatts(value: f32) -> Result<f64, EnergyError> {
    if !value.is_finite() || value < 0.0 {
        return Err(EnergyError::InvalidMegawatts);
    }
    // Normalize signed zero before exact sorting and tie comparison.
    Ok(if value <= 0.0 { 0.0 } else { f64::from(value) })
}

/// Fits the count method's quantile policy independently on FRP observations.
///
/// Caller supplies identical covered cell-months and a fixed satellite pair from
/// the standard overlap. Inputs remain unmodified. Ties average MODIS rank targets;
/// the mean stays inside that rank group's observed bounds to prevent floating
/// roundoff from breaking monotonicity. No count mapping or scale is reused.
///
/// # Errors
/// [`EnergyError`] for absent or mismatched samples, invalid FRP or too many samples.
pub fn fit_energy(
    tile: TileId,
    modis: &[f32],
    viirs: &[f32],
) -> Result<EnergyCalibration, EnergyError> {
    if modis.len() != viirs.len() {
        return Err(EnergyError::LengthMismatch);
    }
    if modis.is_empty() {
        return Err(EnergyError::EmptyOverlap);
    }
    let samples = u32::try_from(modis.len()).map_err(|_| EnergyError::TooManySamples)?;
    let mut modis = modis
        .iter()
        .copied()
        .map(megawatts)
        .collect::<Result<Vec<_>, _>>()?;
    let mut viirs = viirs
        .iter()
        .copied()
        .map(megawatts)
        .collect::<Result<Vec<_>, _>>()?;
    modis.sort_by(f64::total_cmp);
    viirs.sort_by(f64::total_cmp);
    let mut knots = vec![EnergyKnot {
        viirs: 0.0,
        modis: 0.0,
    }];
    let mut pairs = viirs.into_iter().zip(modis).peekable();
    while let Some((x, y)) = pairs.next() {
        let mut total = y;
        let mut count = 1_u32;
        let mut maximum = y;
        while pairs
            .peek()
            .is_some_and(|(next, _)| next.total_cmp(&x).is_eq())
        {
            if let Some((_, next_y)) = pairs.next() {
                total += next_y;
                maximum = next_y;
                count = count.saturating_add(1);
            }
        }
        if x > 0.0 {
            knots.push(EnergyKnot {
                viirs: x,
                modis: (total / f64::from(count)).max(y).min(maximum),
            });
        }
    }
    Ok(EnergyCalibration {
        tile,
        samples,
        knots,
    })
}

#[cfg(test)]
mod tests;
