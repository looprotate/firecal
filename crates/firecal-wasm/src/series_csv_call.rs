//! The `series.csv` of an area, as the browser asks it.

use crate::comparison::compare;
use crate::history::read_history;
use crate::memory::Memory;
use crate::output::{store_text, text_capacity};
use crate::status::{INVALID, STATUS_OK, status_of};
use firecal_query::{SeriesRows, write_series_csv};

/// Writes `series.csv` for the request into the buffer: first the length of the text as a `u32`, then the text.
///
/// The columns are the series, the median of each calendar month over the baseline, the anomaly and whether it
/// is unusual. Returns a status from [`crate::status`]: `STATUS_OUTPUT_TOO_SMALL` for a buffer too small for
/// the text, and nothing is written unless the status is zero.
pub fn run_series_csv<M: Memory>(
    memory: &mut M,
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> u32 {
    let mut answer = || -> Result<(), u32> {
        let (request, history) = read_history(memory, request_address, request_len)?;
        let comparison = compare(&request, &history)?;
        let mut text = vec![0; text_capacity(memory, buffer_address, buffer_len)?];
        let rows = SeriesRows {
            value: &history.value,
            sigma: &history.sigma,
            median: &comparison.median,
            anomaly: &comparison.anomaly,
            unusual: &comparison.unusual,
        };
        let written = write_series_csv(history.first_month, &rows, &mut text).map_err(status_of)?;
        store_text(
            memory,
            buffer_address,
            buffer_len,
            text.get(..written).ok_or(INVALID)?,
        )
    };
    answer().map_or_else(|status| status, |()| STATUS_OK)
}

#[cfg(test)]
mod tests;
