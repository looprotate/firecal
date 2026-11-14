//! The series of an area: the harmonized value and its uncertainty, month by month.

use crate::root::square_root;
use crate::{Area, QueryError};
use core::ops::Range;
use firecal_types::{Error, MonthIndex, Plane, TILE_SIDE, TileId, TileView};

/// Where [`series`] writes. `value` holds one entry per month; `sigma`, when present, must have the same
/// length. Both are overwritten.
#[derive(Debug)]
pub struct SeriesOut<'a> {
    /// The summed value of the chosen plane for each month.
    pub value: &'a mut [f64],
    /// The uncertainty of each month: the cells' sigmas added in quadrature. `None` to skip it.
    pub sigma: Option<&'a mut [f64]>,
}

/// Sums `plane` over the cells of `area` for each month from `first_month`, one per entry of `out.value`.
///
/// A tile that is not in `tiles` (ocean, no hotspots) counts as zeros; if two entries have the same
/// id the first is used.
///
/// The sigma comes from the [`Plane::Sigma`] plane, which is the uncertainty of the count: ask for it with
/// [`Plane::Count`]. Sums of whole numbers stay exact in `f64` far beyond the world (1 036 800 cells of
/// 65 535 squared is under 2^53), so the answer does not depend on the order of the additions.
///
/// # Errors
/// [`QueryError::OutputLengthMismatch`] when `sigma` and `value` differ in length;
/// [`QueryError::Types`] with [`Error::EmptyMonthRange`] for no months, [`Error::MonthRangeTooLong`] when the
/// months do not fit the index, and [`Error::MonthOutsideTile`] when a tile in `tiles` lacks a requested month.
pub fn series(
    tiles: &[(TileId, TileView<'_>)],
    area: &impl Area,
    first_month: MonthIndex,
    plane: Plane,
    out: SeriesOut<'_>,
) -> Result<(), QueryError> {
    let SeriesOut { value, mut sigma } = out;
    let months = value.len();
    if sigma.as_ref().is_some_and(|sigma| sigma.len() != months) {
        return Err(QueryError::OutputLengthMismatch);
    }
    if months == 0 {
        return Err(Error::EmptyMonthRange.into());
    }
    let months = u16::try_from(months).map_err(|_| Error::MonthRangeTooLong)?;
    first_month
        .index()
        .checked_add(months - 1)
        .ok_or(Error::MonthRangeTooLong)?;
    value.fill(0.0);
    if let Some(sigma) = sigma.as_deref_mut() {
        sigma.fill(0.0);
    }
    for tile_row in tile_span(&area.rows()) {
        for tile_col in tile_span(&area.columns()) {
            let id = TileId::new(tile_row, tile_col)?;
            let Some((_, tile)) = tiles.iter().find(|(known, _)| *known == id) else {
                continue;
            };
            let rows = inside_tile(&area.rows(), tile_row);
            let columns = inside_tile(&area.columns(), tile_col);
            for row in rows {
                for col in columns.clone() {
                    let (grid_row, grid_col) =
                        (tile_row * TILE_SIDE + row, tile_col * TILE_SIDE + col);
                    if !area.selects(grid_row, grid_col) {
                        continue;
                    }
                    let cell = Cell { tile, row, col };
                    add_cell(&cell, first_month, plane, value, sigma.as_deref_mut())?;
                }
            }
        }
    }
    if let Some(sigma) = sigma {
        for squares in sigma {
            *squares = square_root(*squares);
        }
    }
    Ok(())
}

fn add_at(slots: &mut [f64], offset: u16, amount: f64) {
    if let Some(slot) = slots.get_mut(usize::from(offset)) {
        *slot += amount;
    }
}

/// The tile rows or columns that a range of grid rows or columns reaches.
const fn tile_span(cells: &Range<u32>) -> Range<u32> {
    cells.start / TILE_SIDE..(cells.end - 1) / TILE_SIDE + 1
}

/// The part of a range of grid rows or columns that falls in tile `tile`, as positions inside the tile.
fn inside_tile(cells: &Range<u32>, tile: u32) -> Range<u32> {
    let start = tile * TILE_SIDE;
    cells.start.max(start) - start..cells.end.min(start + TILE_SIDE) - start
}

/// One cell of a tile.
struct Cell<'a, 'b> {
    tile: &'a TileView<'b>,
    row: u32,
    col: u32,
}

/// Adds the cell's `plane` value, and its squared sigma when `sigma` is given, to each month.
fn add_cell(
    cell: &Cell<'_, '_>,
    first_month: MonthIndex,
    plane: Plane,
    value: &mut [f64],
    mut sigma: Option<&mut [f64]>,
) -> Result<(), QueryError> {
    for offset in 0..u16::try_from(value.len()).map_err(|_| Error::MonthRangeTooLong)? {
        let month = MonthIndex::from_index(first_month.index() + offset);
        add_at(
            value,
            offset,
            f64::from(cell.tile.value(plane, month, cell.row, cell.col)?),
        );
        if let Some(sigma) = sigma.as_deref_mut() {
            let each = f64::from(cell.tile.value(Plane::Sigma, month, cell.row, cell.col)?);
            add_at(sigma, offset, each * each);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
