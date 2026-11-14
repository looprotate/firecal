//! The series question as the browser asks it: a request block in memory in, a status out, and the answer
//! written into memory. Everything is read and checked before anything is written.

use crate::memory::Memory;
use crate::request::{REQUEST_BYTES, Request, TILE_ENTRY_BYTES, TileEntry};
use crate::status::{
    INVALID, STATUS_MONTH_RANGE_INVALID, STATUS_OK, STATUS_TILE_INVALID, status_of,
};
use firecal_query::{Rectangle, SeriesOut, series};
use firecal_types::{MonthIndex, Plane, TileId, TileView};

const OUTPUT_BYTES_PER_MONTH: u32 = 16;
const PLANE_COUNT: u32 = 0;
const PLANE_ENERGY: u32 = 1;

/// Answers the request block at `request_address`, which is [`REQUEST_BYTES`] bytes long.
///
/// It writes `month_count` values and then `month_count` sigmas, each an `f64`, at the address the request
/// names. Returns a status from [`crate::status`]; an address that is not inside `memory` gives [`INVALID`],
/// never a trap.
///
/// An energy series has no sigma of its own yet, so its sigmas are NaN.
pub fn run_series<M: Memory>(memory: &mut M, request_address: u32, request_len: u32) -> u32 {
    match answer(memory, request_address, request_len) {
        Ok(()) => STATUS_OK,
        Err(status) => status,
    }
}

fn answer<M: Memory>(memory: &mut M, address: u32, len: u32) -> Result<(), u32> {
    let (request, entries) = read_request(memory, address, len)?;
    let output_len = request
        .month_count
        .checked_mul(OUTPUT_BYTES_PER_MONTH)
        .ok_or(INVALID)?;
    memory.read(request.output, output_len).ok_or(INVALID)?;
    let (value, sigma) = {
        let files = tile_files(memory, &entries)?;
        compute(&request, &files)?
    };
    let out = memory.write(request.output, output_len).ok_or(INVALID)?;
    let (values_out, sigmas_out) = out.split_at_mut(out.len() / 2);
    write_doubles(values_out, &value);
    write_doubles(sigmas_out, &sigma);
    Ok(())
}

/// The request block at `address` and its tile table, with every address checked to be inside `memory`.
pub(crate) fn read_request<M: Memory>(
    memory: &M,
    address: u32,
    len: u32,
) -> Result<(Request, Vec<TileEntry>), u32> {
    if len != REQUEST_BYTES {
        return Err(INVALID);
    }
    let request = Request::parse(memory.read(address, len).ok_or(INVALID)?).ok_or(INVALID)?;
    let table_len = request
        .tile_count
        .checked_mul(TILE_ENTRY_BYTES)
        .ok_or(INVALID)?;
    let table = memory.read(request.tile_table, table_len).ok_or(INVALID)?;
    let entries = TileEntry::parse_table(table).ok_or(INVALID)?;
    Ok((request, entries))
}

/// The bytes of each tile in the table, with its row and column.
pub(crate) fn tile_files<'m, M: Memory>(
    memory: &'m M,
    entries: &[TileEntry],
) -> Result<Vec<(u32, u32, &'m [u8])>, u32> {
    let mut files = Vec::with_capacity(entries.len());
    for entry in entries {
        let bytes = memory.read(entry.address, entry.len).ok_or(INVALID)?;
        files.push((entry.row, entry.col, bytes));
    }
    Ok(files)
}

fn write_doubles(out: &mut [u8], numbers: &[f64]) {
    for (slot, number) in out.as_chunks_mut::<8>().0.iter_mut().zip(numbers) {
        slot.copy_from_slice(&number.to_le_bytes());
    }
}

/// The tiles, area, plane and months of a request, read and checked.
pub(crate) struct Prepared<'f> {
    pub(crate) plane: Plane,
    pub(crate) months: u16,
    pub(crate) first_month: MonthIndex,
    pub(crate) tiles: Vec<(TileId, TileView<'f>)>,
    pub(crate) area: Rectangle,
}

pub(crate) fn prepare<'f>(
    request: &Request,
    files: &[(u32, u32, &'f [u8])],
) -> Result<Prepared<'f>, u32> {
    let plane = match request.plane {
        PLANE_COUNT => Plane::Count,
        PLANE_ENERGY => Plane::Energy,
        _ => return Err(INVALID),
    };
    let months = u16::try_from(request.month_count).map_err(|_| STATUS_MONTH_RANGE_INVALID)?;
    let first_month = u16::try_from(request.first_month).map_err(|_| STATUS_MONTH_RANGE_INVALID)?;
    let mut tiles = Vec::with_capacity(files.len());
    for (row, col, bytes) in files {
        let id = TileId::new(*row, *col).map_err(|_| STATUS_TILE_INVALID)?;
        let view = TileView::parse(bytes).map_err(|_| STATUS_TILE_INVALID)?;
        tiles.push((id, view));
    }
    let area = Rectangle::new(request.south, request.north, request.west, request.east)
        .map_err(status_of)?;
    Ok(Prepared {
        plane,
        months,
        first_month: MonthIndex::from_index(first_month),
        tiles,
        area,
    })
}

fn compute(request: &Request, files: &[(u32, u32, &[u8])]) -> Result<(Vec<f64>, Vec<f64>), u32> {
    let prepared = prepare(request, files)?;
    let mut value = vec![0.0; usize::from(prepared.months)];
    let mut sigma = vec![0.0; usize::from(prepared.months)];
    let with_sigma = prepared.plane == Plane::Count;
    let out = SeriesOut {
        value: &mut value,
        sigma: with_sigma.then_some(sigma.as_mut_slice()),
    };
    series(
        &prepared.tiles,
        &prepared.area,
        prepared.first_month,
        prepared.plane,
        out,
    )
    .map_err(status_of)?;
    if !with_sigma {
        sigma.fill(f64::NAN);
    }
    Ok((value, sigma))
}

#[cfg(test)]
pub(crate) mod tests;
