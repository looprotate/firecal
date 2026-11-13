//! The tile (FCT1): 40 x 40 cells by every month, in three planes. Written by harmonize, read by the query
//! and the browser. The layout is in `docs/data-format.md`; this module is the only code that knows it.

use crate::bytes::{Writer, read_u16};
use crate::{CellIndex, Error, GRID_COLUMNS, GRID_ROWS, MonthIndex};

/// The first four bytes of every tile file.
pub const TILE_MAGIC: [u8; 4] = *b"FCT1";

/// Cells along one side of a tile.
pub const TILE_SIDE: u32 = 40;
/// Cells in a tile.
pub const TILE_CELLS: usize = 1600;
/// Tiles from north to south.
pub const TILE_ROWS: u32 = GRID_ROWS / TILE_SIDE;
/// Tiles from west to east.
pub const TILE_COLUMNS: u32 = GRID_COLUMNS / TILE_SIDE;

const HEADER_BYTES: usize = 16;
const PLANE_COUNT: u8 = 3;
const VALUE_BYTES: usize = 2;

/// What a plane of a tile holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plane {
    /// Harmonized hotspot count, MODIS-equivalent.
    Count,
    /// Uncertainty of the count, one standard deviation, in counts.
    Sigma,
    /// Harmonized fire radiative power, in megawatts.
    Energy,
}

impl Plane {
    /// The planes in the order they are stored.
    pub const ALL: [Self; 3] = [Self::Count, Self::Sigma, Self::Energy];

    const fn position(self) -> usize {
        match self {
            Self::Count => 0,
            Self::Sigma => 1,
            Self::Energy => 2,
        }
    }
}

/// Which of the 18 x 36 tiles of the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileId {
    row: u32,
    col: u32,
}

impl TileId {
    /// The tile at `row` (0 to 17) and `col` (0 to 35).
    ///
    /// # Errors
    /// [`Error::CellOutsideTile`] when either is out of range.
    pub const fn new(row: u32, col: u32) -> Result<Self, Error> {
        if row >= TILE_ROWS || col >= TILE_COLUMNS {
            return Err(Error::CellOutsideTile);
        }
        Ok(Self { row, col })
    }

    /// The tile that holds `cell`.
    #[must_use]
    pub const fn of_cell(cell: CellIndex) -> Self {
        Self {
            row: cell.row() / TILE_SIDE,
            col: cell.col() / TILE_SIDE,
        }
    }

    /// The tile's row, 0 at the north.
    #[must_use]
    pub const fn row(self) -> u32 {
        self.row
    }

    /// The tile's column, 0 at 180 degrees west.
    #[must_use]
    pub const fn col(self) -> u32 {
        self.col
    }
}

/// The exact size in bytes of a tile of `month_count` months.
#[must_use]
pub fn tile_len(month_count: u16) -> usize {
    HEADER_BYTES + plane_bytes(month_count) * usize::from(PLANE_COUNT)
}

fn plane_bytes(month_count: u16) -> usize {
    usize::from(month_count) * TILE_CELLS * VALUE_BYTES
}

/// Writes a tile into `out` and returns how many bytes were written. Each plane holds `month_count * 1600`
/// values, month-major: `[month][cell_row][cell_col]`.
///
/// # Errors
/// [`Error::PlaneLengthMismatch`] and [`Error::BufferTooSmall`].
pub fn write_tile(
    first_month: MonthIndex,
    month_count: u16,
    planes: [&[u16]; 3],
    out: &mut [u8],
) -> Result<usize, Error> {
    let expected = usize::from(month_count) * TILE_CELLS;
    if planes.iter().any(|plane| plane.len() != expected) {
        return Err(Error::PlaneLengthMismatch);
    }
    let mut writer = Writer::new(out);
    writer.put(&TILE_MAGIC)?;
    writer.put(&month_count.to_le_bytes())?;
    writer.put(&first_month.index().to_le_bytes())?;
    writer.put(&[PLANE_COUNT, 0, 0, 0, 0, 0, 0, 0])?;
    for plane in planes {
        for value in plane {
            writer.put(&value.to_le_bytes())?;
        }
    }
    Ok(writer.written())
}

/// A validated tile file, read in place without copying.
#[derive(Debug, Clone, Copy)]
pub struct TileView<'a> {
    first_month: MonthIndex,
    month_count: u16,
    body: &'a [u8],
}

impl<'a> TileView<'a> {
    /// Checks the header and the length, so every later read is a bounds-checked lookup.
    ///
    /// # Errors
    /// [`Error::Truncated`], [`Error::TrailingBytes`], [`Error::BadMagic`], [`Error::ReservedNotZero`] and
    /// [`Error::UnsupportedPlaneCount`].
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Error> {
        let header = bytes.get(..HEADER_BYTES).ok_or(Error::Truncated)?;
        if header.get(..4) != Some(&TILE_MAGIC[..]) {
            return Err(Error::BadMagic);
        }
        let month_count = read_u16(header, 4).ok_or(Error::Truncated)?;
        let first_month = MonthIndex::from_index(read_u16(header, 6).ok_or(Error::Truncated)?);
        if header.get(8) != Some(&PLANE_COUNT) {
            return Err(Error::UnsupportedPlaneCount);
        }
        if header
            .get(9..HEADER_BYTES)
            .is_none_or(|reserved| reserved.iter().any(|byte| *byte != 0))
        {
            return Err(Error::ReservedNotZero);
        }
        let body = bytes.get(HEADER_BYTES..).ok_or(Error::Truncated)?;
        match body
            .len()
            .cmp(&(plane_bytes(month_count) * usize::from(PLANE_COUNT)))
        {
            core::cmp::Ordering::Less => Err(Error::Truncated),
            core::cmp::Ordering::Greater => Err(Error::TrailingBytes),
            core::cmp::Ordering::Equal => Ok(Self {
                first_month,
                month_count,
                body,
            }),
        }
    }

    /// The first month the tile holds.
    #[must_use]
    pub const fn first_month(&self) -> MonthIndex {
        self.first_month
    }

    /// How many consecutive months the tile holds.
    #[must_use]
    pub const fn month_count(&self) -> u16 {
        self.month_count
    }

    /// The value of `plane` in `month` at `local_row` and `local_col`, both 0 to 39 inside the tile.
    ///
    /// # Errors
    /// [`Error::MonthOutsideTile`] and [`Error::CellOutsideTile`].
    pub fn value(
        &self,
        plane: Plane,
        month: MonthIndex,
        local_row: u32,
        local_col: u32,
    ) -> Result<u16, Error> {
        let months_in = month
            .index()
            .checked_sub(self.first_month.index())
            .filter(|offset| *offset < self.month_count)
            .ok_or(Error::MonthOutsideTile)?;
        if local_row >= TILE_SIDE || local_col >= TILE_SIDE {
            return Err(Error::CellOutsideTile);
        }
        let cell = usize::try_from(local_row * TILE_SIDE + local_col)
            .map_err(|_| Error::CellOutsideTile)?;
        let plane_start = plane.position() * plane_bytes(self.month_count);
        let at = plane_start + (usize::from(months_in) * TILE_CELLS + cell) * VALUE_BYTES;
        read_u16(self.body, at).ok_or(Error::Truncated)
    }
}

#[cfg(test)]
mod tests;
