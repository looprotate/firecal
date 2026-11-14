//! The `cells.csv` of an area as the browser asks it: the same request block as the series, and a buffer in
//! memory for the text.

use crate::memory::Memory;
use crate::series_call::{prepare, read_request, tile_files};
use crate::status::{INVALID, STATUS_OK, status_of};
use firecal_query::write_cells_csv;

const LENGTH_BYTES: u32 = 4;

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
    let capacity = buffer_len.checked_sub(LENGTH_BYTES).ok_or(INVALID)?;
    memory.read(buffer_address, buffer_len).ok_or(INVALID)?;
    let mut text = vec![0; usize::try_from(capacity).map_err(|_| INVALID)?];
    let written = {
        let files = tile_files(memory, &entries)?;
        let prepared = prepare(&request, &files)?;
        write_cells_csv(
            &prepared.tiles,
            &prepared.area,
            prepared.first_month,
            prepared.months,
            prepared.plane,
            &mut text,
        )
        .map_err(status_of)?
    };
    let out = memory.write(buffer_address, buffer_len).ok_or(INVALID)?;
    let (length_out, text_out) =
        out.split_at_mut(usize::try_from(LENGTH_BYTES).map_err(|_| INVALID)?);
    length_out.copy_from_slice(&u32::try_from(written).map_err(|_| INVALID)?.to_le_bytes());
    text_out
        .get_mut(..written)
        .ok_or(INVALID)?
        .copy_from_slice(text.get(..written).ok_or(INVALID)?);
    Ok(())
}

#[cfg(test)]
mod tests;
