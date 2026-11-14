//! Ordered per-satellite, per-month accumulators serialized by firecal-types.

use crate::Hotspot;
use firecal_types::{AggregateRecord, CellIndex, MonthIndex, Satellite, write_aggregate};
use std::collections::BTreeMap;

/// Invalid inputs, capacity limits, or an output-format error during aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregateError {
    /// The detection belongs to a different satellite or month.
    MismatchedGroup,
    /// Latitude or longitude is non-finite or outside the Earth grid.
    InvalidCoordinate,
    /// Fire radiative power is negative or non-finite.
    InvalidEnergy,
    /// The next detection would overflow the FCA1 `u32` count.
    CountOverflow,
    /// The accumulated FRP would exceed the finite FCA1 `f32` range.
    EnergyOverflow,
    /// An error from the shared FCA1 writer, including insufficient output space.
    Format(firecal_types::Error),
}

#[derive(Debug, Clone, Copy, Default)]
struct CellTotal {
    count: u32,
    frp_megawatts: f64,
    day_count: u32,
}

/// A single satellite-month's sparse cell totals, updated in input order.
///
/// Counts are checked `u32`, not saturated `u16` tile values. FRP accumulates in
/// `f64` and rounds once to `f32` when records are materialized for FCA1. Keep
/// input order fixed for reproducible floating-point sums. Detections are not
/// deduplicated; the caller must avoid overlapping archive/NRT inputs.
#[derive(Debug, Clone)]
pub struct Aggregator {
    satellite: Satellite,
    month: MonthIndex,
    cells: BTreeMap<CellIndex, CellTotal>,
}

impl Aggregator {
    /// Creates an empty group. An empty group writes a valid zero-record FCA1 file.
    #[must_use]
    pub const fn new(satellite: Satellite, month: MonthIndex) -> Self {
        Self {
            satellite,
            month,
            cells: BTreeMap::new(),
        }
    }

    /// Adds one kept detection; any error leaves the accumulator unchanged.
    ///
    /// The parser performs confidence/source filtering before this call. Grid
    /// edges belong to the cell south/east of the edge. The outer south/east
    /// limits (-90 latitude, +180 longitude) clamp to the last row/column.
    /// `day_count` counts daytime detections, not distinct acquisition days.
    ///
    /// # Errors
    /// Rejects a mismatched satellite/month, invalid coordinates or energy, and
    /// counts or energy sums that would not fit the FCA1 format.
    pub fn add(&mut self, hotspot: &Hotspot) -> Result<(), AggregateError> {
        if hotspot.satellite != self.satellite || hotspot.month != self.month {
            return Err(AggregateError::MismatchedGroup);
        }
        let cell = crate::grid::cell(hotspot.latitude, hotspot.longitude)?;
        if !hotspot.frp_megawatts.is_finite() || hotspot.frp_megawatts < 0.0 {
            return Err(AggregateError::InvalidEnergy);
        }
        let previous = self.cells.get(&cell).copied().unwrap_or_default();
        let count = previous
            .count
            .checked_add(1)
            .ok_or(AggregateError::CountOverflow)?;
        let day_count = previous
            .day_count
            .checked_add(u32::from(hotspot.daytime))
            .ok_or(AggregateError::CountOverflow)?;
        let frp_megawatts = previous.frp_megawatts + hotspot.frp_megawatts;
        if !frp_megawatts.is_finite() || frp_megawatts > f64::from(f32::MAX) {
            return Err(AggregateError::EnergyOverflow);
        }
        self.cells.insert(
            cell,
            CellTotal {
                count,
                frp_megawatts,
                day_count,
            },
        );
        Ok(())
    }

    /// Returns FCA1 records in strictly increasing cell order, rounding FRP once.
    ///
    /// # Errors
    /// [`AggregateError::EnergyOverflow`] if a total cannot fit a finite `f32`.
    pub fn records(&self) -> Result<Vec<AggregateRecord>, AggregateError> {
        self.cells
            .iter()
            .map(|(cell, total)| {
                Ok(AggregateRecord {
                    cell: *cell,
                    count: total.count,
                    frp_megawatts: stored_energy(total.frp_megawatts)?,
                    day_count: total.day_count,
                })
            })
            .collect()
    }

    /// Writes FCA1 bytes to the caller's buffer via the shared format writer.
    ///
    /// Size the buffer with `firecal_types::aggregate_len(records.len())` using
    /// [`Self::records`]. The binary, not this pure library, saves the bytes to disk.
    ///
    /// # Errors
    /// A record conversion error or [`AggregateError::Format`], notably
    /// `BufferTooSmall`. The shared writer may partially fill an undersized buffer.
    pub fn write(&self, output: &mut [u8]) -> Result<usize, AggregateError> {
        write_aggregate(self.satellite, self.month, &self.records()?, output)
            .map_err(AggregateError::Format)
    }
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "FCA1 explicitly stores f32 FRP; the finite range is checked and only the final sum is rounded"
)]
fn stored_energy(value: f64) -> Result<f32, AggregateError> {
    if !value.is_finite() || value < 0.0 || value > f64::from(f32::MAX) {
        return Err(AggregateError::EnergyOverflow);
    }
    Ok(value as f32)
}

#[cfg(test)]
mod tests;
