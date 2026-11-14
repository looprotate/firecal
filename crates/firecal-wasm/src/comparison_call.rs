//! The comparison of every month with what is normal, as the browser asks it.

use crate::comparison::compare;
use crate::history::read_history;
use crate::memory::Memory;
use crate::output::{block, put_doubles};
use crate::status::{INVALID, STATUS_OK};

const BYTES_PER_MONTH: usize = 17;

/// Writes the median, anomaly and unusual flag of each of `month_count` months.
///
/// First the medians as `f64`, then the anomalies as `f64`, then one byte per month, 1 for unusual. A month with no baseline value has NaN for both numbers and 0 for the flag. Returns a status from
/// [`crate::status`]: `STATUS_OUTPUT_TOO_SMALL` for a buffer shorter than `month_count * 17` bytes, and nothing
/// is written unless the status is zero.
pub fn run_comparison<M: Memory>(
    memory: &mut M,
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> u32 {
    let mut answer = || -> Result<(), u32> {
        let (request, history) = read_history(memory, request_address, request_len)?;
        let comparison = compare(&request, &history)?;
        let months = history.value.len();
        let needed = months.checked_mul(BYTES_PER_MONTH).ok_or(INVALID)?;
        let out = block(memory, buffer_address, buffer_len, needed)?;
        let (numbers_out, flags_out) = out.split_at_mut(months * 16);
        let (median_out, anomaly_out) = numbers_out.split_at_mut(months * 8);
        put_doubles(median_out, &comparison.median);
        put_doubles(anomaly_out, &comparison.anomaly);
        for (slot, unusual) in flags_out.iter_mut().zip(&comparison.unusual) {
            *slot = u8::from(*unusual);
        }
        Ok(())
    };
    answer().map_or_else(|status| status, |()| STATUS_OK)
}

#[cfg(test)]
mod tests;
