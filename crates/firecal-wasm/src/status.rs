//! The status a series export returns: zero for success, a small code for a question that cannot be answered,
//! and [`INVALID`] for a request that cannot even be read.

use firecal_query::QueryError;
use firecal_types::Error;

/// Returned by an export that could not do what was asked.
pub const INVALID: u32 = u32::MAX;
/// The series was written.
pub const STATUS_OK: u32 = 0;
/// The area has no extent.
pub const STATUS_EMPTY_AREA: u32 = 1;
/// A latitude or longitude outside the world.
pub const STATUS_AREA_OUT_OF_RANGE: u32 = 2;
/// No months, or more than the month index holds.
pub const STATUS_MONTH_RANGE_INVALID: u32 = 3;
/// A tile that cannot be read, or sits at a row and column outside the tile grid.
pub const STATUS_TILE_INVALID: u32 = 4;
/// A tile that does not hold a requested month.
pub const STATUS_MONTH_NOT_IN_TILE: u32 = 5;

/// The status that tells the caller what to change.
pub(crate) const fn status_of(error: QueryError) -> u32 {
    match error {
        QueryError::EmptyArea => STATUS_EMPTY_AREA,
        QueryError::LatitudeOutOfRange | QueryError::LongitudeOutOfRange => {
            STATUS_AREA_OUT_OF_RANGE
        }
        QueryError::OutputLengthMismatch => INVALID,
        QueryError::Types(Error::EmptyMonthRange | Error::MonthRangeTooLong) => {
            STATUS_MONTH_RANGE_INVALID
        }
        QueryError::Types(Error::MonthOutsideTile) => STATUS_MONTH_NOT_IN_TILE,
        QueryError::Types(_) => STATUS_TILE_INVALID,
    }
}
