//! The series of an area: the harmonized value and its uncertainty, month by month.

use crate::root::square_root;
use crate::{QueryError, Rectangle};
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

/// Sums `plane` over the cells of `area` for each month from `first_month`, as many months as `out.value`
/// holds. A tile that is not in `tiles` (ocean, no hotspots) counts as zeros; if two entries have the same
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
    area: &Rectangle,
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
            for offset in 0..months {
                let month = MonthIndex::from_index(first_month.index() + offset);
                let sums = sum_cells(tile, month, plane, sigma.is_some(), &rows, &columns)?;
                add_at(value, offset, sums.0);
                if let Some(sigma) = sigma.as_deref_mut() {
                    add_at(sigma, offset, sums.1);
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
fn tile_span(cells: &Range<u32>) -> Range<u32> {
    cells.start / TILE_SIDE..(cells.end - 1) / TILE_SIDE + 1
}

/// The part of a range of grid rows or columns that falls in tile `tile`, as positions inside the tile.
fn inside_tile(cells: &Range<u32>, tile: u32) -> Range<u32> {
    let start = tile * TILE_SIDE;
    cells.start.max(start) - start..cells.end.min(start + TILE_SIDE) - start
}

/// The sum of `plane` and the sum of squared sigma over the cells of one tile in one month.
fn sum_cells(
    tile: &TileView<'_>,
    month: MonthIndex,
    plane: Plane,
    with_sigma: bool,
    rows: &Range<u32>,
    columns: &Range<u32>,
) -> Result<(f64, f64), QueryError> {
    let mut total = 0.0;
    let mut squares = 0.0;
    for row in rows.clone() {
        for col in columns.clone() {
            total += f64::from(tile.value(plane, month, row, col)?);
            if with_sigma {
                let sigma = f64::from(tile.value(Plane::Sigma, month, row, col)?);
                squares += sigma * sigma;
            }
        }
    }
    Ok((total, squares))
}

#[cfg(test)]
mod tests;
