//! Monthly raw and mapped VIIRS totals over genuinely paired overlap months.

use super::{AggregateIndex, CountPlan, TileBuildError, fit_aggregates};
use firecal_types::{AggregateView, MonthIndex, Satellite, TileId};
use std::fmt::Write;

/// Why a region's monthly overlap report cannot be generated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportError {
    /// The caller selected no tiles.
    EmptyRegion,
    /// A tile occurs more than once in the selected region.
    DuplicateTile,
    /// A tile's input, training or count mapping is invalid.
    Tile(TileBuildError),
}

#[derive(Debug, Clone)]
struct Row {
    tile: TileId,
    month: MonthIndex,
    modis_source: Satellite,
    viirs_source: Satellite,
    training: bool,
    modis: u64,
    viirs: u64,
    mapped: f64,
}

/// Deterministic monthly overlap diagnostics before tile rounding or saturation.
///
/// Rows exist only when both chosen source files cover the tile-month. A training
/// label describes fitting membership, never an independent validation result.
#[derive(Debug, Clone)]
pub struct OverlapReport {
    rows: Vec<Row>,
}

impl OverlapReport {
    /// Number of tile-month rows; at most 648 tiles times 65535 months.
    #[must_use]
    pub fn row_count(&self) -> u32 {
        u32::try_from(self.rows.len()).unwrap_or(u32::MAX)
    }
    /// CSV in tile-row, tile-column, month order, with a header and LF line endings.
    /// Raw totals are integers; mapped totals have six decimal places. Values are
    /// unsaturated count estimates, not FRP, and are not the MODIS-preferred tile plane.
    #[must_use]
    pub fn csv(&self) -> String {
        let mut csv = String::from(
            "tile_row,tile_col,year,month,month_index,modis_satellite,viirs_satellite,split,raw_modis_count,raw_viirs_count,harmonized_viirs_count\n",
        );
        for row in &self.rows {
            // Writing primitive fields into a String cannot fail.
            let _ = writeln!(
                csv,
                "{},{},{},{},{},{},{},{},{},{},{:.6}",
                row.tile.row(),
                row.tile.col(),
                row.month.year(),
                row.month.month(),
                row.month.index(),
                row.modis_source.code(),
                row.viirs_source.code(),
                if row.training { "training" } else { "held_out" },
                row.modis,
                row.viirs,
                row.mapped
            );
        }
        csv
    }
}

/// Fits each selected tile and reports paired source months without hiding differences.
///
/// Plans may be supplied in any order and must select unique, completely covered
/// tiles. Fixed sources and a complete standard-product assertion are inherited
/// from each plan. Fits use only overlap months through `train_through`. All
/// paired report months are evaluated by the VIIRS mapping even when MODIS exists;
/// missing files yield no row, while an explicit empty file is a genuine zero.
///
/// # Errors
/// [`ReportError`] for empty/duplicate regions or invalid training/input/mapping.
pub fn overlap_report(
    plans: &[CountPlan],
    aggregates: &[AggregateView<'_>],
    train_through: MonthIndex,
) -> Result<OverlapReport, ReportError> {
    if plans.is_empty() {
        return Err(ReportError::EmptyRegion);
    }
    let mut plans = plans.to_vec();
    plans.sort_by_key(|plan| plan.tile());
    if plans
        .windows(2)
        .any(|pair| matches!(pair,[a,b] if a.tile()==b.tile()))
    {
        return Err(ReportError::DuplicateTile);
    }
    let mut rows = Vec::new();
    for plan in &plans {
        let calibration =
            fit_aggregates(plan, aggregates, train_through).map_err(ReportError::Tile)?;
        let index = AggregateIndex::new(plan, aggregates).map_err(ReportError::Tile)?;
        for month in index.paired_months(plan) {
            let modis = index
                .counts(plan, month, plan.modis)
                .ok_or(ReportError::Tile(TileBuildError::MissingMonth(month)))?;
            let viirs = index
                .counts(plan, month, plan.viirs)
                .ok_or(ReportError::Tile(TileBuildError::MissingMonth(month)))?;
            // A tile has 1600 u32 counts: these totals fit u64. Saturation is defensive.
            let raw_modis = modis
                .iter()
                .fold(0_u64, |sum, &value| sum.saturating_add(u64::from(value)));
            let raw_viirs = viirs
                .iter()
                .fold(0_u64, |sum, &value| sum.saturating_add(u64::from(value)));
            let mut mapped = 0.0;
            for value in viirs {
                mapped += calibration
                    .map_count(value)
                    .map_err(|error| ReportError::Tile(TileBuildError::Calibration(error)))?;
            }
            rows.push(Row {
                tile: plan.tile,
                month,
                modis_source: plan.modis,
                viirs_source: plan.viirs,
                training: month <= train_through,
                modis: raw_modis,
                viirs: raw_viirs,
                mapped,
            });
        }
    }
    Ok(OverlapReport { rows })
}

#[cfg(test)]
mod tests;
