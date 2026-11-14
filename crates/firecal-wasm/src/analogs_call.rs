//! The analog years of the season in progress, as the browser asks them.

use crate::comparison::baseline_of;
use crate::history::read_history;
use crate::memory::Memory;
use crate::output::block;
use crate::status::{
    INVALID, STATUS_OK, STATUS_OUTPUT_TOO_SMALL, STATUS_SEASON_INVALID, status_of,
};
use firecal_query::{Analog, Season, analog_years};

const COUNT_BYTES: usize = 4;
const SLOT_BYTES: usize = 12;
const MOST_ANALOGS: usize = 64;

/// Writes the past years most like the season in progress, nearest first.
///
/// The buffer holds first the number found as a `u32`, then room for as many slots as fit after it, each a
/// `u32` year and an `f64` distance. The number of slots the buffer has is how many analogs are asked for, at most 64. Returns a status from
/// [`crate::status`]: `STATUS_OUTPUT_TOO_SMALL` for a buffer shorter than the count, and nothing is written
/// unless the status is zero.
pub fn run_analogs<M: Memory>(
    memory: &mut M,
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> u32 {
    let mut answer = || -> Result<(), u32> {
        let (request, history) = read_history(memory, request_address, request_len)?;
        let slots = usize::try_from(buffer_len)
            .map_err(|_| INVALID)?
            .checked_sub(COUNT_BYTES)
            .ok_or(STATUS_OUTPUT_TOO_SMALL)?
            / SLOT_BYTES;
        let year = u16::try_from(request.season_year).map_err(|_| STATUS_SEASON_INVALID)?;
        let months = u16::try_from(request.season_months).map_err(|_| STATUS_SEASON_INVALID)?;
        let season = Season::new(year, months).map_err(status_of)?;
        let empty = Analog {
            year: 0,
            distance: 0.0,
        };
        let mut found = vec![empty; slots.min(MOST_ANALOGS)];
        let count = analog_years(
            history.first_month,
            &history.value,
            baseline_of(&request)?,
            season,
            &mut found,
        )
        .map_err(status_of)?;
        let needed = COUNT_BYTES + slots.min(MOST_ANALOGS) * SLOT_BYTES;
        let out = block(memory, buffer_address, buffer_len, needed)?;
        let (count_out, slots_out) = out.split_at_mut(COUNT_BYTES);
        count_out.copy_from_slice(&u32::try_from(count).map_err(|_| INVALID)?.to_le_bytes());
        for (slot, analog) in slots_out
            .as_chunks_mut::<SLOT_BYTES>()
            .0
            .iter_mut()
            .zip(&found)
            .take(count)
        {
            let (year_out, distance_out) = slot.split_at_mut(4);
            year_out.copy_from_slice(&u32::from(analog.year).to_le_bytes());
            distance_out.copy_from_slice(&analog.distance.to_le_bytes());
        }
        Ok(())
    };
    answer().map_or_else(|status| status, |()| STATUS_OK)
}

#[cfg(test)]
mod tests;
