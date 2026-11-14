//! Coordinate-to-cell mapping with integer search instead of lossy float casts.

use crate::AggregateError;
use firecal_types::{CellIndex, GRID_COLUMNS, GRID_ROWS};

pub(crate) fn cell(latitude: f64, longitude: f64) -> Result<CellIndex, AggregateError> {
    if !latitude.is_finite()
        || !longitude.is_finite()
        || !(-90.0..=90.0).contains(&latitude)
        || !(-180.0..=180.0).contains(&longitude)
    {
        return Err(AggregateError::InvalidCoordinate);
    }
    let row = bin(latitude, 90.0, -0.25, GRID_ROWS);
    let col = bin(longitude, -180.0, 0.25, GRID_COLUMNS);
    CellIndex::new(row, col).map_err(AggregateError::Format)
}

fn bin(coordinate: f64, origin: f64, step: f64, limit: u32) -> u32 {
    let mut low = 0;
    let mut high = limit;
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        let offset = f64::from(middle) * step;
        let boundary = origin + offset;
        let before = if step < 0.0 {
            coordinate > boundary
        } else {
            coordinate < boundary
        };
        if before {
            high = middle;
        } else {
            low = middle;
        }
    }
    low
}

#[cfg(test)]
mod tests;
