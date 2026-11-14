//! What is normal for each calendar month: the median of the baseline years.

use crate::{BaselineYears, QueryError};
use firecal_types::{Error, MonthIndex};

const MOST_BASELINE_YEARS: usize = 64;

/// The median value of each calendar month over the baseline years; `None` for a month none of them has.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Climatology {
    median: [Option<f64>; 12],
}

impl Climatology {
    /// The median for the calendar month of `month`, or `None` when the baseline has no value for it.
    #[must_use]
    pub fn median(&self, month: MonthIndex) -> Option<f64> {
        let calendar_month = usize::from(month.month() - 1);
        self.median.get(calendar_month).copied().flatten()
    }
}

/// The climatology of `value`, a monthly series that starts at `first_month`, over the `baseline` years. An
/// even number of years gives the mean of the two middle values; months outside the baseline are ignored.
///
/// # Errors
/// [`QueryError::BaselineTooLong`] for more than 64 years of one calendar month, and [`QueryError::Types`]
/// with [`Error::MonthRangeTooLong`] when the series runs past the month index.
pub fn climatology(
    first_month: MonthIndex,
    value: &[f64],
    baseline: BaselineYears,
) -> Result<Climatology, QueryError> {
    let mut median = [None; 12];
    for (slot, calendar_month) in median.iter_mut().zip(1_u16..=12) {
        let mut years = [0.0; MOST_BASELINE_YEARS];
        let mut count = 0;
        for (offset, number) in value.iter().enumerate() {
            let at = u16::try_from(offset)
                .ok()
                .and_then(|offset| first_month.index().checked_add(offset))
                .ok_or(Error::MonthRangeTooLong)?;
            let month = MonthIndex::from_index(at);
            if month.month() == calendar_month && baseline.contains(month.year()) {
                *years.get_mut(count).ok_or(QueryError::BaselineTooLong)? = *number;
                count += 1;
            }
        }
        *slot = middle_of(years.get_mut(..count));
    }
    Ok(Climatology { median })
}

fn middle_of(numbers: Option<&mut [f64]>) -> Option<f64> {
    let numbers = numbers.filter(|numbers| !numbers.is_empty())?;
    numbers.sort_unstable_by(f64::total_cmp);
    let upper = numbers.len() / 2;
    let high = *numbers.get(upper)?;
    if numbers.len() % 2 == 1 {
        Some(high)
    } else {
        let low = *numbers.get(upper - 1)?;
        Some((low + high) / 2.0)
    }
}

#[cfg(test)]
mod tests;
