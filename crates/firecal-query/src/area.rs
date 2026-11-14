//! A rectangle of latitude and longitude and the grid cells it touches.

use crate::QueryError;
use core::ops::Range;
use firecal_types::{GRID_COLUMNS, GRID_ROWS};

const CELL_DEGREES: f64 = 0.25;

/// A lat/lon rectangle with extent. It selects every cell it overlaps with positive area, so a rectangle
/// thinner than a cell selects that cell, and one that only touches a cell's edge does not.
///
/// It cannot cross the antimeridian: `west` is left of `east`.
#[derive(Debug, Clone, Copy, PartialEq)]
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
        Ok(Self {
            rows: (
                count_rows(|row| south_edge(row) >= north),
                count_rows(|row| north_edge(row) > south),
            ),
            columns: (
                count_columns(|col| east_edge(col) <= west),
                count_columns(|col| west_edge(col) < east),
            ),
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
