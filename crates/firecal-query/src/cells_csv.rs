//! `cells.csv`: one row for every cell of an area and every month of a range.

use crate::area::{column_center, row_center};
use crate::csv_text::TextBuffer;
use crate::{Area, QueryError};
use firecal_types::{CellIndex, Error, MonthIndex, Plane, TileId, TileView};

const HEADER: &str = "year,month,cell_lat,cell_lon,value,sigma\n";

/// Writes `cells.csv` into `out` and returns how many bytes it wrote.
///
/// A header, then for each of `months` months from `first_month`, a row `year,month,cell_lat,cell_lon,value,sigma` for each cell of `area`, rows
/// north to south and each row west to east. Latitude and longitude are the center of the cell, with three
/// decimals. A cell in a tile that is not in `tiles` has value 0. For [`Plane::Count`] the sigma is that of
/// the count; for the other planes it is left empty, since they have none. Lines end in `\n`.
///
/// # Errors
/// [`QueryError::BufferTooSmall`] when `out` cannot hold it; [`QueryError::Types`] with
/// [`Error::EmptyMonthRange`] or [`Error::MonthRangeTooLong`] for the months, or
/// [`Error::MonthOutsideTile`] when a tile lacks a requested month.
pub fn write_cells_csv(
    tiles: &[(TileId, TileView<'_>)],
    area: &impl Area,
    first_month: MonthIndex,
    months: u16,
    plane: Plane,
    out: &mut [u8],
) -> Result<usize, QueryError> {
    if months == 0 {
        return Err(Error::EmptyMonthRange.into());
    }
    first_month
        .index()
        .checked_add(months - 1)
        .ok_or(Error::MonthRangeTooLong)?;
    let mut text = TextBuffer::new(out);
    text.put(format_args!("{HEADER}"))?;
    for offset in 0..months {
        let month = MonthIndex::from_index(first_month.index() + offset);
        for row in area.rows() {
            for col in area.columns().filter(|col| area.selects(row, *col)) {
                let cell = CellIndex::new(row, col)?;
                let found = tiles
                    .iter()
                    .find(|(id, _)| *id == TileId::of_cell(cell))
                    .map(|(_, tile)| tile);
                let at = (
                    row % firecal_types::TILE_SIDE,
                    col % firecal_types::TILE_SIDE,
                );
                let read = |plane| found.map_or(Ok(0), |tile| tile.value(plane, month, at.0, at.1));
                text.put(format_args!("{},{},", month.year(), month.month()))?;
                text.put_number(row_center(row), 3)?;
                text.put(format_args!(","))?;
                text.put_number(column_center(col), 3)?;
                text.put(format_args!(",{},", read(plane)?))?;
                if plane == Plane::Count {
                    text.put(format_args!("{}", read(Plane::Sigma)?))?;
                }
                text.put(format_args!("\n"))?;
            }
        }
    }
    Ok(text.written())
}

#[cfg(test)]
mod tests;
