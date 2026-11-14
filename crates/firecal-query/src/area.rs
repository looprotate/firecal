//! A rectangle of latitude and longitude and the grid cells it touches.

#![cfg_attr(
    test,
    expect(
        clippy::suboptimal_flops,
        reason = "only with std, which tests have: fused multiply-add rounds once, and wasm must round like native"
    )
)]

use crate::QueryError;
use core::ops::Range;
use firecal_types::{GRID_COLUMNS, GRID_ROWS};

const CELL_DEGREES: f64 = 0.25;

/// A lat/lon rectangle with extent. It selects every cell it overlaps with positive area, so a rectangle
/// thinner than a cell selects that cell, and one that only touches a cell's edge does not.
///
/// It cannot cross the antimeridian: `west` is left of `east`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    rows: (u32, u32),
    columns: (u32, u32),
}

impl Rectangle {
    /// The rectangle from `south` to `north` (-90 to 90) and `west` to `east` (-180 to 180), in degrees.
    ///
    /// # Errors
    /// [`QueryError::LatitudeOutOfRange`], [`QueryError::LongitudeOutOfRange`] or
    /// [`QueryError::EmptyArea`]; a bound that is not a number is out of range.
    pub fn new(south: f64, north: f64, west: f64, east: f64) -> Result<Self, QueryError> {
        if !(-90.0..=90.0).contains(&south) || !(-90.0..=90.0).contains(&north) {
            return Err(QueryError::LatitudeOutOfRange);
        }
        if !(-180.0..=180.0).contains(&west) || !(-180.0..=180.0).contains(&east) {
            return Err(QueryError::LongitudeOutOfRange);
        }
        if south >= north || west >= east {
            return Err(QueryError::EmptyArea);
        }
        let rows = rows_between(south, north);
        let columns = columns_between(west, east);
        Ok(Self {
            rows: (rows.start, rows.end),
            columns: (columns.start, columns.end),
        })
    }

    /// The grid rows the rectangle overlaps, 0 at the north: a non-empty range inside `0..720`.
    #[must_use]
    pub const fn rows(&self) -> Range<u32> {
        self.rows.0..self.rows.1
    }

    /// The grid columns the rectangle overlaps, 0 at 180 degrees west: a non-empty range inside `0..1440`.
    #[must_use]
    pub const fn columns(&self) -> Range<u32> {
        self.columns.0..self.columns.1
    }
}

/// A region of the grid that a series sums over: a bounding block of rows and columns, and which cells in it
/// belong.
pub trait Area {
    /// The grid rows that can hold a selected cell: a non-empty range inside `0..720`.
    fn rows(&self) -> Range<u32>;
    /// The grid columns that can hold a selected cell: a non-empty range inside `0..1440`.
    fn columns(&self) -> Range<u32>;
    /// Whether the cell at `row` and `col` belongs to the area.
    fn selects(&self, row: u32, col: u32) -> bool;
}

impl Area for Rectangle {
    fn rows(&self) -> Range<u32> {
        Self::rows(self)
    }

    fn columns(&self) -> Range<u32> {
        Self::columns(self)
    }

    fn selects(&self, _row: u32, _col: u32) -> bool {
        true
    }
}

/// The grid rows that overlap the latitudes `south` to `north` with positive area.
pub(crate) fn rows_between(south: f64, north: f64) -> Range<u32> {
    count_rows(|row| south_edge(row) >= north)..count_rows(|row| north_edge(row) > south)
}

/// The grid columns that overlap the longitudes `west` to `east` with positive area.
pub(crate) fn columns_between(west: f64, east: f64) -> Range<u32> {
    count_columns(|col| east_edge(col) <= west)..count_columns(|col| west_edge(col) < east)
}

/// The latitude of the middle of grid row `row`.
pub(crate) fn row_center(row: u32) -> f64 {
    90.0 - CELL_DEGREES * (f64::from(row) + 0.5)
}

/// The longitude of the middle of grid column `col`.
pub(crate) fn column_center(col: u32) -> f64 {
    -180.0 + CELL_DEGREES * (f64::from(col) + 0.5)
}

// Cell edges are multiples of 0.25, exact in f64, so these comparisons never depend on rounding. Counting
// the cells that satisfy a monotonic test avoids floor and casts.
fn count_rows(test: impl Fn(u32) -> bool) -> u32 {
    let counted = (0..GRID_ROWS).filter(|row| test(*row)).count();
    u32::try_from(counted).unwrap_or(GRID_ROWS)
}

fn count_columns(test: impl Fn(u32) -> bool) -> u32 {
    let counted = (0..GRID_COLUMNS).filter(|col| test(*col)).count();
    u32::try_from(counted).unwrap_or(GRID_COLUMNS)
}

fn north_edge(row: u32) -> f64 {
    90.0 - CELL_DEGREES * f64::from(row)
}

fn south_edge(row: u32) -> f64 {
    90.0 - CELL_DEGREES * f64::from(row + 1)
}

fn west_edge(col: u32) -> f64 {
    -180.0 + CELL_DEGREES * f64::from(col)
}

fn east_edge(col: u32) -> f64 {
    -180.0 + CELL_DEGREES * f64::from(col + 1)
}

#[cfg(test)]
mod tests;
