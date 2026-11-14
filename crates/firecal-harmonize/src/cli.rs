//! Native command arguments; coverage assertions are explicit for both output modes.

use firecal_harmonize::CountPlan;
use firecal_types::{MonthIndex, Satellite, TILE_COLUMNS, TILE_ROWS, TileId};
use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

const USAGE: &str = concat!(
    "usage:\nfirecal-harmonize build --complete-standard-tile OUTPUT_DIR ROW COL FIRST_MONTH MONTH_COUNT MODIS_CODE VIIRS_CODE TRAIN_THROUGH YYYY-MM-DD FILE.fca...\n",
    "firecal-harmonize report --complete-standard-tile OUTPUT.csv FIRST_ROW FIRST_COL ROW_COUNT COL_COUNT FIRST_MONTH MONTH_COUNT MODIS_CODE VIIRS_CODE TRAIN_THROUGH FILE.fca...\n",
    "The flag asserts complete science-quality coverage of every selected tile in each supplied satellite-month. Country-clipped inputs do not assert full-tile coverage."
);

pub(super) enum Action {
    Build { plan: CountPlan, date: String },
    Report { plans: Vec<CountPlan> },
}
pub(super) struct Options {
    pub(super) action: Action,
    pub(super) output: PathBuf,
    pub(super) cutoff: MonthIndex,
    pub(super) files: Vec<PathBuf>,
}

fn number<T: std::str::FromStr>(
    arguments: &mut impl Iterator<Item = OsString>,
    name: &str,
) -> Result<T, String> {
    arguments
        .next()
        .ok_or_else(|| format!("missing {name}\n{USAGE}"))?
        .into_string()
        .map_err(|_| format!("{name} is not text"))?
        .parse()
        .map_err(|_| format!("invalid {name}"))
}

impl Options {
    pub(super) fn parse() -> Result<Self, String> {
        let mut arguments = std::env::args_os().skip(1);
        let report = match arguments.next().as_deref() {
            Some(command) if command == OsStr::new("build") => false,
            Some(command) if command == OsStr::new("report") => true,
            _ => return Err(USAGE.to_owned()),
        };
        if arguments.next().as_deref() != Some(OsStr::new("--complete-standard-tile")) {
            return Err(USAGE.to_owned());
        }
        let output = PathBuf::from(arguments.next().ok_or(USAGE)?);
        let row: u32 = number(&mut arguments, "first tile row")?;
        let col: u32 = number(&mut arguments, "first tile column")?;
        let (rows, cols) = if report {
            (
                number(&mut arguments, "tile row count")?,
                number(&mut arguments, "tile column count")?,
            )
        } else {
            (1_u32, 1_u32)
        };
        let row_end = row
            .checked_add(rows)
            .filter(|&end| end <= TILE_ROWS)
            .ok_or("tile row range outside grid")?;
        let col_end = col
            .checked_add(cols)
            .filter(|&end| end <= TILE_COLUMNS)
            .ok_or("tile column range outside grid")?;
        if rows == 0 || cols == 0 {
            return Err("tile range must not be empty".to_owned());
        }
        let first = MonthIndex::from_index(number(&mut arguments, "first month")?);
        let months = number(&mut arguments, "month count")?;
        let modis = Satellite::from_code(number(&mut arguments, "MODIS satellite code")?)
            .map_err(|error| format!("{error:?}"))?;
        let viirs = Satellite::from_code(number(&mut arguments, "VIIRS satellite code")?)
            .map_err(|error| format!("{error:?}"))?;
        let cutoff = MonthIndex::from_index(number(&mut arguments, "last training month")?);
        let action = if report {
            let mut plans = Vec::new();
            for tile_row in row..row_end {
                for tile_col in col..col_end {
                    let tile =
                        TileId::new(tile_row, tile_col).map_err(|error| format!("{error:?}"))?;
                    plans.push(
                        CountPlan::new(tile, first, months, modis, viirs)
                            .map_err(|error| format!("{error:?}"))?,
                    );
                }
            }
            Action::Report { plans }
        } else {
            let date = arguments
                .next()
                .ok_or("missing build date")?
                .into_string()
                .map_err(|_| "build date is not text")?;
            let tile = TileId::new(row, col).map_err(|error| format!("{error:?}"))?;
            let plan = CountPlan::new(tile, first, months, modis, viirs)
                .map_err(|error| format!("{error:?}"))?;
            Action::Build { plan, date }
        };
        let files = arguments.map(PathBuf::from).collect::<Vec<_>>();
        if files.is_empty() {
            return Err("no FCA1 files supplied".to_owned());
        }
        Ok(Self {
            action,
            output,
            cutoff,
            files,
        })
    }
}
