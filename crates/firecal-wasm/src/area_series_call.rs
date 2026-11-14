//! The series of a rectangle or a polygon, as the browser asks it: an analysis request block in, the values and
//! sigmas of the whole history out.

use crate::history::read_history;
use crate::memory::Memory;
use crate::output::{block, put_doubles};
use crate::status::{INVALID, STATUS_OK};

const BYTES_PER_MONTH: usize = 16;

/// Writes `month_count` values and then `month_count` sigmas, each an `f64`, into the buffer.
///
/// The request is the analysis block; its baseline and season are not used. Returns a status from
/// [`crate::status`]: `STATUS_OUTPUT_TOO_SMALL` for a buffer shorter than `month_count * 16` bytes, and nothing
/// is written unless the status is zero.
pub fn run_area_series<M: Memory>(
    memory: &mut M,
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> u32 {
    let mut answer = || -> Result<(), u32> {
        let (_, history) = read_history(memory, request_address, request_len)?;
        let needed = history
            .value
            .len()
            .checked_mul(BYTES_PER_MONTH)
            .ok_or(INVALID)?;
        let out = block(memory, buffer_address, buffer_len, needed)?;
        let (values_out, sigmas_out) = out.split_at_mut(needed / 2);
        put_doubles(values_out, &history.value);
        put_doubles(sigmas_out, &history.sigma);
        Ok(())
    };
    answer().map_or_else(|status| status, |()| STATUS_OK)
}

#[cfg(test)]
mod tests;
