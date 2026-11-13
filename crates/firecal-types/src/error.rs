//! What can go wrong when a shared type is built from raw numbers.

/// A raw value that does not fit the calendar, the grid or an enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// A month number outside `1..=12`.
    MonthOutOfRange,
    /// A year before 2000, the base of the month index.
    YearBeforeBase,
    /// A year so late that its month index does not fit in 16 bits.
    YearTooLate,
    /// A cell row or column outside the 720 x 1440 grid.
    CellOutsideGrid,
    /// A satellite code that is not one of the five known.
    UnknownSatellite,
    /// A month range whose last month comes before its first.
    EmptyMonthRange,
    /// A month range with more months than fit in 16 bits.
    MonthRangeTooLong,
    /// A file that does not start with the magic bytes of its format.
    BadMagic,
    /// A file shorter than its own header says it is.
    Truncated,
    /// A file longer than its own header says it is.
    TrailingBytes,
    /// A byte that the format reserves as zero was not zero.
    ReservedNotZero,
    /// An output buffer too small for what must be written.
    BufferTooSmall,
    /// Records that are not in strictly increasing cell order.
    RecordsNotSorted,
    /// A record whose numbers cannot be true: negative or infinite energy, or more day detections than detections.
    RecordInvalid,
    /// More records than a 32-bit count can hold.
    TooManyRecords,
    /// A tile whose plane count is not the three the format defines.
    UnsupportedPlaneCount,
    /// A plane whose length is not `month_count * 1600` values.
    PlaneLengthMismatch,
    /// A tile or cell coordinate outside the tile.
    CellOutsideTile,
    /// A month that the tile does not hold.
    MonthOutsideTile,
}
