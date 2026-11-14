//! The `cells.csv` of an area as the browser asks it: the same request block as the series, and a buffer in
//! memory for the text.

use crate::memory::Memory;
use crate::output::{store_text, text_capacity};
use crate::series_call::{prepare, read_request, rectangle_of, tile_files};
use crate::status::{INVALID, STATUS_OK, status_of};
use firecal_query::write_cells_csv;

/// Writes `cells.csv` for the request block at `request_address` into a buffer.
///
/// The `output` field of the block is not used. The buffer is `buffer_len` bytes: first the length of the
/// text as a `u32`, then the text. Returns a status from [`crate::status`]; a buffer too small for the text is
/// `STATUS_OUTPUT_TOO_SMALL`, and nothing is written unless the status is zero.
pub fn run_cells_csv<M: Memory>(
    memory: &mut M,
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> u32 {
    match answer(
        memory,
        request_address,
        request_len,
        buffer_address,
        buffer_len,
    ) {
        Ok(()) => STATUS_OK,
        Err(status) => status,
    }
}

fn answer<M: Memory>(
    memory: &mut M,
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> Result<(), u32> {
    let (request, entries) = read_request(memory, request_address, request_len)?;
    let mut text = vec![0; text_capacity(memory, buffer_address, buffer_len)?];
    let written = {
        let files = tile_files(memory, &entries)?;
        let prepared = prepare(&request, &files)?;
        let area = rectangle_of(&request)?;
        write_cells_csv(
            &prepared.tiles,
            &area,
            prepared.first_month,
            prepared.months,
            prepared.plane,
            &mut text,
        )
        .map_err(status_of)?
    };
    store_text(
        memory,
        buffer_address,
        buffer_len,
        text.get(..written).ok_or(INVALID)?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests;
