//! Deterministic empirical count quantiles for one tile and one fixed satellite pair.

use firecal_types::TileId;

/// Why a count calibration cannot be fitted or applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalibrationError {
    /// The sensor slices do not describe the same number of matched cell-months.
    LengthMismatch,
    /// No matched training cell-month exists.
    EmptyOverlap,
    /// More than `u32::MAX` training samples were supplied.
    TooManySamples,
    /// Positive VIIRS input has no positive training support.
    UnsupportedPositiveCount,
}

/// A VIIRS count and its target MODIS-equivalent count, before tile rounding.
#[derive(Debug, Clone)]
pub struct CountKnot {
    viirs_count: u32,
    modis_count: f64,
}

impl CountKnot {
    /// Raw VIIRS detections per cell-month.
    #[must_use]
    pub const fn viirs_count(&self) -> u32 {
        self.viirs_count
    }

    /// MODIS-equivalent detections per cell-month; may be fractional.
    #[must_use]
    pub const fn modis_count(&self) -> f64 {
        self.modis_count
    }
}

/// Ordered quantile knots for one tile; constructed only by [`fit`].
///
/// Training coverage, source pair and held-out months are the caller's responsibility.
/// A one-sample fit is mathematically defined but insufficient for scientific validation.
#[derive(Debug, Clone)]
pub struct Calibration {
    tile: TileId,
    sample_count: u32,
    knots: Vec<CountKnot>,
}

impl Calibration {
    /// The tile this calibration belongs to.
    #[must_use]
    pub const fn tile(&self) -> TileId {
        self.tile
    }

    /// Number of matched cell-months used for fitting, including zero observations.
    #[must_use]
    pub const fn sample_count(&self) -> u32 {
        self.sample_count
    }

    /// Strictly increasing VIIRS knots with nondecreasing MODIS targets.
    #[must_use]
    pub fn knots(&self) -> &[CountKnot] {
        &self.knots
    }

    /// Largest training VIIRS count; larger inputs use the clamped tail.
    #[must_use]
    pub fn max_viirs_count(&self) -> u32 {
        self.knots.last().map_or(0, |knot| knot.viirs_count)
    }

    /// Interpolates a raw count in MODIS-equivalent units without rounding or saturation.
    /// Zero maps to zero; values above the fitted range clamp to its last target.
    ///
    /// # Errors
    /// [`CalibrationError::UnsupportedPositiveCount`] if all training VIIRS counts
    /// were zero and `count` is positive.
    pub fn map_count(&self, count: u32) -> Result<f64, CalibrationError> {
        if count == 0 {
            return Ok(0.0);
        }
        if self.max_viirs_count() == 0 {
            return Err(CalibrationError::UnsupportedPositiveCount);
        }
        let mut lower_x = 0;
        let mut lower_y = 0.0;
        for knot in &self.knots {
            if count == knot.viirs_count {
                return Ok(knot.modis_count);
            }
            if count < knot.viirs_count {
                let fraction = f64::from(count - lower_x) / f64::from(knot.viirs_count - lower_x);
                let increment = fraction * (knot.modis_count - lower_y);
                return Ok(lower_y + increment);
            }
            lower_x = knot.viirs_count;
            lower_y = knot.modis_count;
        }
        Ok(lower_y)
    }
}

/// Fits empirical quantile knots from matched cell-month counts inside `tile`.
///
/// The caller must supply equal-length slices describing identical cell-months
/// with known coverage and a fixed MODIS/VIIRS source pair, only on the overlap
/// period (2012 onward). Inputs are copied and independently sorted; neither is
/// modified. Zero is anchored at zero, tied VIIRS ranks average their MODIS
/// order statistics, and no positive extrapolation is performed.
///
/// # Errors
/// Returns [`CalibrationError::LengthMismatch`] for unequal lengths,
/// [`CalibrationError::EmptyOverlap`] for empty inputs, or
/// [`CalibrationError::TooManySamples`] for more than `u32::MAX` observations.
pub fn fit(tile: TileId, modis: &[u32], viirs: &[u32]) -> Result<Calibration, CalibrationError> {
    if modis.len() != viirs.len() {
        return Err(CalibrationError::LengthMismatch);
    }
    if modis.is_empty() {
        return Err(CalibrationError::EmptyOverlap);
    }
    let sample_count = u32::try_from(modis.len()).map_err(|_| CalibrationError::TooManySamples)?;
    let mut modis = modis.to_vec();
    let mut viirs = viirs.to_vec();
    modis.sort_unstable();
    viirs.sort_unstable();
    let mut knots = vec![CountKnot {
        viirs_count: 0,
        modis_count: 0.0,
    }];
    let mut pairs = viirs.into_iter().zip(modis).peekable();
    while let Some((x, y)) = pairs.next() {
        let mut total = f64::from(y);
        let mut count = 1_u32;
        while pairs.peek().is_some_and(|(next_x, _)| *next_x == x) {
            if let Some((_, next_y)) = pairs.next() {
                total += f64::from(next_y);
                // The entire sample length was checked to fit u32 above.
                count = count.saturating_add(1);
            }
        }
        if x != 0 {
            knots.push(CountKnot {
                viirs_count: x,
                modis_count: total / f64::from(count),
            });
        }
    }
    Ok(Calibration {
        tile,
        sample_count,
        knots,
    })
}

#[cfg(test)]
mod tests;
