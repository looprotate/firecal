//! How a month compares with what is normal for its calendar month: the anomaly and whether it is unusual.

use crate::{Climatology, QueryError};
use firecal_types::{Error, MonthIndex};

/// Where [`compare_with_climatology`] writes, one entry per month; both are overwritten.
#[derive(Debug)]
pub struct ComparisonOut<'a> {
    /// `(value - median) / max(median, 1)`: 0 is normal, 1 is twice the median, -1 is nothing.
    pub anomaly: &'a mut [f64],
    /// The month rose: its rise over the median clears the sensor band (`value - median > 2 * sigma`) **and** the
    /// value is above the highest of the baseline years for its calendar month. A drop is never flagged.
    pub unusual: &'a mut [bool],
}

/// Compares each month of `value`, which starts at `first_month` and has uncertainty `sigma`, with the
/// median of its calendar month in `climatology`.
///
/// # Errors
/// [`QueryError::OutputLengthMismatch`] when `sigma` and the outputs do not have the length of `value`;
/// [`QueryError::NoBaselineForMonth`] when a month has no median; [`QueryError::Types`] with
/// [`Error::MonthRangeTooLong`] when the months run past the month index.
pub fn compare_with_climatology(
    climatology: &Climatology,
    first_month: MonthIndex,
    value: &[f64],
    sigma: &[f64],
    out: ComparisonOut<'_>,
) -> Result<(), QueryError> {
    let ComparisonOut { anomaly, unusual } = out;
    let months = value.len();
    if sigma.len() != months || anomaly.len() != months || unusual.len() != months {
        return Err(QueryError::OutputLengthMismatch);
    }
    let rows = value
        .iter()
        .zip(sigma)
        .zip(anomaly.iter_mut().zip(unusual.iter_mut()));
    for (offset, ((number, spread), (anomaly_slot, unusual_slot))) in rows.enumerate() {
        let at = u16::try_from(offset)
            .ok()
            .and_then(|offset| first_month.index().checked_add(offset))
            .ok_or(Error::MonthRangeTooLong)?;
        let month = MonthIndex::from_index(at);
        let median = climatology
            .median(month)
            .ok_or(QueryError::NoBaselineForMonth)?;
        let (_, highest) = climatology
            .range(month)
            .ok_or(QueryError::NoBaselineForMonth)?;
        let difference = number - median;
        *anomaly_slot = difference / median.max(1.0);
        let above_sensor_band = difference > 2.0 * spread;
        let above_baseline_years = *number > highest;
        *unusual_slot = above_sensor_band && above_baseline_years;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
