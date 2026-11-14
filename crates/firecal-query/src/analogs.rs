//! Analog years: the past seasons that looked most like the one in progress.
//!
//! The distance between two years is the sum, over the months of the season, of the difference between their
//! ranks within that calendar month across all the years. A rank needs no logarithm and treats a quiet region
//! and a busy one alike. Ties take the middle rank, counted in halves so every number stays a whole one.

use crate::{BaselineYears, QueryError};
use firecal_types::{BASE_YEAR, MonthIndex};

const MOST_YEARS: usize = 64;

/// The part of a year that has happened: `months` months from January of `year`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Season {
    year: u16,
    months: u16,
}

impl Season {
    /// January to month `months` (1 to 12) of `year`.
    ///
    /// # Errors
    /// [`QueryError::SeasonMonthsOutOfRange`] when `months` is 0 or more than 12.
    pub const fn new(year: u16, months: u16) -> Result<Self, QueryError> {
        if months == 0 || months > 12 {
            return Err(QueryError::SeasonMonthsOutOfRange);
        }
        Ok(Self { year, months })
    }
}

/// A past year and how far its season is from the one in progress.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Analog {
    /// The calendar year.
    pub year: u16,
    /// The sum of the rank differences over the season's months: 0 is the same ranking in every month.
    pub distance: f64,
}

/// The monthly `value` series, starting at `first_month`, as years and calendar months.
#[derive(Debug, Clone, Copy)]
struct History<'a> {
    first_month: MonthIndex,
    value: &'a [f64],
}

impl History<'_> {
    fn at(&self, year: u16, month: u16) -> Option<f64> {
        let months = u32::from(year.checked_sub(BASE_YEAR)?) * 12 + u32::from(month - 1);
        let offset = months.checked_sub(u32::from(self.first_month.index()))?;
        self.value.get(usize::try_from(offset).ok()?).copied()
    }
}

/// The years that are ranked against each other, current one last.
struct Population {
    years: [u16; MOST_YEARS + 1],
    count: usize,
}

impl Population {
    fn years(&self) -> &[u16] {
        self.years.get(..self.count).unwrap_or(&[])
    }

    /// Twice the middle rank of `year` among all the years in `month`: 0 is the lowest, `2 * (years - 1)` the
    /// highest, and tied years share the middle of their places.
    fn double_rank(&self, history: &History<'_>, year: u16, month: u16) -> u32 {
        let own = history.at(year, month);
        let (mut below, mut equal) = (0_u32, 0_u32);
        for other in self.years() {
            match history.at(*other, month).partial_cmp(&own) {
                Some(core::cmp::Ordering::Less) => below += 1,
                Some(core::cmp::Ordering::Equal) => equal += 1,
                _ => {}
            }
        }
        (2 * below + equal).saturating_sub(1)
    }

    /// Twice the distance between `a` and `b` over `months` months: the sum of the rank differences, doubled.
    fn double_distance(&self, history: &History<'_>, months: u16, a: u16, b: u16) -> u32 {
        (1..=months)
            .map(|month| {
                self.double_rank(history, a, month)
                    .abs_diff(self.double_rank(history, b, month))
            })
            .sum()
    }
}

/// The past years of `baseline` most like `season`, nearest first, written into `out`; returns how many.
///
/// `value` is the monthly series of the area, starting at `first_month`. A baseline year takes part only if
/// the series holds every month of the season for it; the season's own year is never its own analog. Years at
/// the same distance come oldest first. At most `out.len()` are written.
///
/// # Errors
/// [`QueryError::SeasonIncomplete`] when the series lacks a month of the season in progress;
/// [`QueryError::BaselineTooLong`] for more than 64 candidate years.
pub fn analog_years(
    first_month: MonthIndex,
    value: &[f64],
    baseline: BaselineYears,
    season: Season,
    out: &mut [Analog],
) -> Result<usize, QueryError> {
    let history = History { first_month, value };
    if (1..=season.months).any(|month| history.at(season.year, month).is_none()) {
        return Err(QueryError::SeasonIncomplete);
    }
    let mut population = Population {
        years: [0; MOST_YEARS + 1],
        count: 0,
    };
    let mut candidates = [Analog {
        year: 0,
        distance: 0.0,
    }; MOST_YEARS];
    let mut found = 0;
    for year in baseline.first()..=baseline.last() {
        let complete = (1..=season.months).all(|month| history.at(year, month).is_some());
        if year == season.year || !complete {
            continue;
        }
        let slot = population
            .years
            .get_mut(found)
            .ok_or(QueryError::BaselineTooLong)?;
        *slot = year;
        found += 1;
    }
    let slot = population.years.get_mut(found);
    *slot.ok_or(QueryError::BaselineTooLong)? = season.year;
    population.count = found + 1;
    for (candidate, year) in candidates.iter_mut().zip(population.years()).take(found) {
        let double = population.double_distance(&history, season.months, season.year, *year);
        *candidate = Analog {
            year: *year,
            distance: f64::from(double) / 2.0,
        };
    }
    let nearest = candidates
        .get_mut(..found)
        .ok_or(QueryError::BaselineTooLong)?;
    nearest.sort_unstable_by(|a, b| a.distance.total_cmp(&b.distance).then(a.year.cmp(&b.year)));
    let written = nearest.len().min(out.len());
    for (slot, analog) in out.iter_mut().zip(nearest.iter()).take(written) {
        *slot = *analog;
    }
    Ok(written)
}

#[cfg(test)]
mod tests;
