//! What can go wrong when a question is asked of the data.

use firecal_types::Error;

/// A question that cannot be answered as asked; each variant says what the caller should change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryError {
    /// The area has no extent: its south is not below its north, or its west not left of its east, or a
    /// bound is not a number.
    EmptyArea,
    /// A latitude outside -90 to 90 degrees.
    LatitudeOutOfRange,
    /// A longitude outside -180 to 180 degrees.
    LongitudeOutOfRange,
    /// The value and sigma output buffers do not have the same length.
    OutputLengthMismatch,
    /// A baseline whose first year is after its last.
    EmptyBaseline,
    /// A baseline with more years in one calendar month than the median can sort: more than 64.
    BaselineTooLong,
    /// A month to compare that no baseline year has a value for.
    NoBaselineForMonth,
    /// A season of zero months or more than twelve.
    SeasonMonthsOutOfRange,
    /// The season in progress lacks a month in the series, so it cannot be ranked.
    SeasonIncomplete,
    /// A polygon whose edges cross or touch each other.
    SelfIntersecting,
    /// A polygon that goes all the way around a pole, which has no longitude to cut it at.
    EnclosesPole,
    /// An output buffer too small for the text.
    BufferTooSmall,
    /// A tile or month problem reported by the shared types, such as a month the tile does not hold.
    Types(Error),
}

impl From<Error> for QueryError {
    fn from(error: Error) -> Self {
        Self::Types(error)
    }
}
