//! The years that define what is normal.

use crate::QueryError;
use firecal_types::BASE_YEAR;

/// The first year in which every calendar month has MODIS data: the archive starts in November 2000.
const FIRST_COMPLETE_YEAR: u16 = BASE_YEAR + 1;

/// A run of calendar years, both ends included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineYears {
    first: u16,
    last: u16,
}

impl BaselineYears {
    /// The years `first` to `last`.
    ///
    /// # Errors
    /// [`QueryError::EmptyBaseline`] when `first` is after `last`.
    pub const fn new(first: u16, last: u16) -> Result<Self, QueryError> {
        if first > last {
            return Err(QueryError::EmptyBaseline);
        }
        Ok(Self { first, last })
    }

    /// The default baseline: every complete year before `current_year`, from 2001.
    ///
    /// # Errors
    /// [`QueryError::EmptyBaseline`] when there is none, that is for 2001 or earlier.
    pub const fn before(current_year: u16) -> Result<Self, QueryError> {
        match current_year.checked_sub(1) {
            Some(last) => Self::new(FIRST_COMPLETE_YEAR, last),
            None => Err(QueryError::EmptyBaseline),
        }
    }

    /// The first year.
    #[must_use]
    pub const fn first(&self) -> u16 {
        self.first
    }

    /// The last year.
    #[must_use]
    pub const fn last(&self) -> u16 {
        self.last
    }

    /// Whether `year` is one of the baseline years.
    #[must_use]
    pub const fn contains(&self, year: u16) -> bool {
        self.first <= year && year <= self.last
    }
}

#[cfg(test)]
mod tests;
