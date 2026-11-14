//! `series.csv`: one row per month with the value, its uncertainty and how it compares with what is normal.

use crate::QueryError;
use crate::csv_text::TextBuffer;
use firecal_types::{Error, MonthIndex};

const HEADER: &str = "year,month,value,sigma,median,anomaly,unusual\n";

/// The columns of the series, one entry per month; all must have the same length.
#[derive(Debug, Clone, Copy)]
pub struct SeriesRows<'a> {
    /// The summed value of each month.
    pub value: &'a [f64],
    /// The uncertainty of each month; NaN when there is none.
    pub sigma: &'a [f64],
    /// The median of each month's calendar month; NaN when the baseline has none.
    pub median: &'a [f64],
    /// The anomaly of each month; NaN when there is no median.
    pub anomaly: &'a [f64],
    /// Whether each month clears the uncertainty band.
    pub unusual: &'a [bool],
}

/// Writes `series.csv` into `out` and returns how many bytes it wrote.
///
/// A header, then a row
/// `year,month,value,sigma,median,anomaly,unusual` for each month from `first_month`. Value has no decimals,
/// sigma and anomaly three, median one; a number that is NaN or infinite is left empty. `unusual` is `true`
/// or `false`. Lines end in `\n`.
///
/// # Errors
/// [`QueryError::OutputLengthMismatch`] when the columns differ in length; [`QueryError::BufferTooSmall`];
/// [`QueryError::Types`] with [`Error::EmptyMonthRange`] or [`Error::MonthRangeTooLong`].
pub fn write_series_csv(
    first_month: MonthIndex,
    rows: &SeriesRows<'_>,
    out: &mut [u8],
) -> Result<usize, QueryError> {
    let months = rows.value.len();
    if [
        rows.sigma.len(),
        rows.median.len(),
        rows.anomaly.len(),
        rows.unusual.len(),
    ]
    .iter()
    .any(|length| *length != months)
    {
        return Err(QueryError::OutputLengthMismatch);
    }
    if months == 0 {
        return Err(Error::EmptyMonthRange.into());
    }
    let mut text = TextBuffer::new(out);
    text.put(format_args!("{HEADER}"))?;
    let columns = rows
        .value
        .iter()
        .zip(rows.sigma)
        .zip(rows.median)
        .zip(rows.anomaly)
        .zip(rows.unusual);
    for (offset, ((((value, sigma), median), anomaly), unusual)) in columns.enumerate() {
        let at = u16::try_from(offset)
            .ok()
            .and_then(|offset| first_month.index().checked_add(offset))
            .ok_or(Error::MonthRangeTooLong)?;
        let month = MonthIndex::from_index(at);
        text.put(format_args!("{},{},", month.year(), month.month()))?;
        for (number, decimals) in [(value, 0), (sigma, 3), (median, 1), (anomaly, 3)] {
            text.put_number(*number, decimals)?;
            text.put(format_args!(","))?;
        }
        text.put(format_args!("{unusual}\n"))?;
    }
    Ok(text.written())
}

#[cfg(test)]
mod tests;
