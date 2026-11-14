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
    /// A tile or month problem reported by the shared types, such as a month the tile does not hold.
    Types(Error),
}

impl From<Error> for QueryError {
    fn from(error: Error) -> Self {
        Self::Types(error)
    }
}
