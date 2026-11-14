//! How a month compares with what is normal for its calendar month: the anomaly and whether it is unusual.

use crate::{Climatology, QueryError};
use firecal_types::{Error, MonthIndex};

/// Where [`compare_with_climatology`] writes, one entry per month; both are overwritten.
#[derive(Debug)]
pub struct ComparisonOut<'a> {
    /// `(value - median) / max(median, 1)`: 0 is normal, 1 is twice the median, -1 is nothing.
    pub anomaly: &'a mut [f64],
    /// `|value - median| > 2 * sigma`: the difference clears the uncertainty band.
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
    let months = value.len();
    if sigma.len() != months || out.anomaly.len() != months || out.unusual.len() != months {
        return Err(QueryError::OutputLengthMismatch);
    }
    let rows = value
        .iter()
        .zip(sigma)
        .zip(out.anomaly.iter_mut().zip(out.unusual.iter_mut()));
    for (offset, ((number, spread), (anomaly, unusual))) in rows.enumerate() {
        let at = u16::try_from(offset)
            .ok()
            .and_then(|offset| first_month.index().checked_add(offset))
            .ok_or(Error::MonthRangeTooLong)?;
        let median = climatology
            .median(MonthIndex::from_index(at))
            .ok_or(QueryError::NoBaselineForMonth)?;
        let difference = number - median;
        *anomaly = difference / median.max(1.0);
        *unusual = difference.abs() > 2.0 * spread;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
