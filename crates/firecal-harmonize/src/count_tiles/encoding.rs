//! FCT1 encoding through firecal-types and explicit count-only development metadata.

use super::{CountPlan, TileBuildError};
use crate::Calibration;
use firecal_types::{MonthIndex, tile_len, write_tile};

/// A complete tile's count plane, before publishing the future sigma/energy planes.
///
/// FCT1 requires three physical planes, so the other two are zero storage placeholders.
/// They have no scientific meaning. Consumers must honor `planes: ["count"]` and
/// `development_only: true` in the index and must not display uncertainty or energy.
#[derive(Debug, Clone)]
pub struct CountTile {
    pub(super) plan: CountPlan,
    pub(super) counts: Vec<u16>,
    pub(super) paired_months: Vec<MonthIndex>,
}

impl CountTile {
    /// True when no cell-month has a nonzero encoded count; its binary file is omitted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.counts.iter().all(|value| *value == 0)
    }

    /// Encodes month-major counts using the unchanged FCT1 writer.
    ///
    /// # Errors
    /// [`TileBuildError::Types`] if the shared encoder rejects the plane sizes.
    pub fn encode(&self) -> Result<Vec<u8>, TileBuildError> {
        let placeholders = vec![0; self.counts.len()];
        let mut bytes = vec![0; tile_len(self.plan.months)];
        write_tile(
            self.plan.first,
            self.plan.months,
            [&self.counts, &placeholders, &placeholders],
            &mut bytes,
        )
        .map_err(TileBuildError::Types)?;
        Ok(bytes)
    }

    /// Serializes metadata and the fitted knots for this count-only development build.
    /// `date` is supplied at the edge so the pure library never reads the clock.
    ///
    /// # Errors
    /// [`TileBuildError::InvalidBuildDate`] for an invalid YYYY-MM-DD date or
    /// [`TileBuildError::WrongCalibrationTile`] for a calibration from another tile.
    pub fn index_json(
        &self,
        calibration: &Calibration,
        train_through: MonthIndex,
        date: &str,
    ) -> Result<String, TileBuildError> {
        if calibration.tile() != self.plan.tile {
            return Err(TileBuildError::WrongCalibrationTile);
        }
        validate_date(date)?;
        let training_months = self
            .paired_months
            .iter()
            .filter(|month| **month <= train_through)
            .map(|month| month.index().to_string())
            .collect::<Vec<_>>()
            .join(",");
        let row = self.plan.tile.row();
        let col = self.plan.tile.col();
        let files = if self.is_empty() {
            String::new()
        } else {
            format!("\"t_{row}_{col}.bin\"")
        };
        let knots = calibration
            .knots()
            .iter()
            .map(|knot| format!("[{},{}]", knot.viirs_count(), knot.modis_count()))
            .collect::<Vec<_>>()
            .join(",");
        Ok(format!(
            concat!(
                "{{\"first_month_index\":{},\"month_count\":{},",
                "\"calibration_version\":\"count-quantile-v1\",\"build_date\":\"{}\",",
                "\"planes\":[\"count\"],\"stored_planes\":[\"count\",\"sigma\",\"energy\"],",
                "\"development_only\":true,\"nrt_from_month\":null,\"tiles\":[{}],",
                "\"coverage\":{{\"tile_row\":{},\"tile_col\":{},\"complete_tile\":true}},",
                "\"calibration\":{{\"modis_satellite\":{},\"viirs_satellite\":{},",
                "\"sample_count\":{},\"train_through_month\":{},\"training_months\":[{}],\"tail\":\"clamp\",",
                "\"zero\":\"structural\",\"knots\":[{}]}}}}\n"
            ),
            self.plan.first.index(),
            self.plan.months,
            date,
            files,
            row,
            col,
            self.plan.modis.code(),
            self.plan.viirs.code(),
            calibration.sample_count(),
            train_through.index(),
            training_months,
            knots
        ))
    }
}

fn validate_date(date: &str) -> Result<(), TileBuildError> {
    let parts: Vec<_> = date.split('-').collect();
    let parsed = || -> Option<(u16, u16, u16)> {
        if date.len() != 10
            || parts.len() != 3
            || parts.first()?.len() != 4
            || parts.get(1)?.len() != 2
            || parts.get(2)?.len() != 2
        {
            return None;
        }
        if !parts
            .iter()
            .all(|part| part.bytes().all(|byte| byte.is_ascii_digit()))
        {
            return None;
        }
        Some((
            parts.first()?.parse().ok()?,
            parts.get(1)?.parse().ok()?,
            parts.get(2)?.parse().ok()?,
        ))
    };
    let (year, month, day) = parsed().ok_or(TileBuildError::InvalidBuildDate)?;
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let maximum = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if leap {
                29
            } else {
                28
            }
        }
        _ => 0,
    };
    if year == 0 || day == 0 || day > maximum {
        return Err(TileBuildError::InvalidBuildDate);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
