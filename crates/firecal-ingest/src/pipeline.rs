//! Bounded CSV batches, deterministic satellite-month ownership and final FCA1 output.

mod batch;
mod input;
mod output;

use firecal_ingest::{Aggregator, CsvHeader, Hotspot, ParseError};
use firecal_types::{MonthIndex, Satellite};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

const BATCH_ROWS: usize = 4096;
type Key = (Satellite, MonthIndex);
type Groups = BTreeMap<Key, Aggregator>;

/// Counts of input rows and emitted satellite-month files.
#[derive(Debug, Default)]
pub(crate) struct Summary {
    /// Nonblank input data rows, including intentionally filtered detections.
    pub rows: u64,
    /// Nominal/high-confidence vegetation detections.
    pub kept: u64,
    /// Low confidence or non-vegetation detections.
    pub filtered: u64,
    /// Number of FCA1 files written.
    pub files: u32,
}

pub(crate) fn execute(
    input: &Path,
    output: &Path,
    workers: NonZeroUsize,
) -> Result<Summary, String> {
    if output.try_exists().map_err(|error| error.to_string())? {
        return Err(
            "output directory already exists; choose a new directory to preserve previous data"
                .into(),
        );
    }
    let paths = input::csv_files(input)?;
    let (groups, mut summary) = aggregate(&paths, workers)?;
    summary.files = output::write(output, &groups)?;
    Ok(summary)
}

fn aggregate(paths: &[PathBuf], workers: NonZeroUsize) -> Result<(Groups, Summary), String> {
    let mut groups = Groups::new();
    let mut summary = Summary::default();
    let mut pending = Vec::with_capacity(BATCH_ROWS);
    for path in paths {
        read_file(path, &mut pending, &mut groups, &mut summary, workers)?;
    }
    batch::add(&mut groups, pending, workers)?;
    Ok((groups, summary))
}

fn read_file(
    path: &Path,
    pending: &mut Vec<Hotspot>,
    groups: &mut Groups,
    summary: &mut Summary,
    workers: NonZeroUsize,
) -> Result<(), String> {
    let mut reader =
        BufReader::new(File::open(path).map_err(|e| format!("{}: {e}", path.display()))?);
    let mut line = Vec::new();
    reader
        .read_until(b'\n', &mut line)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let header = CsvHeader::parse(&line).map_err(|e| format!("{}:1: {e:?}", path.display()))?;
    let mut line_number = 1_u64;
    loop {
        line.clear();
        if reader
            .read_until(b'\n', &mut line)
            .map_err(|e| format!("{}: {e}", path.display()))?
            == 0
        {
            break;
        }
        increment(&mut line_number)?;
        match header.parse_record(&line) {
            Ok(record) => {
                increment(&mut summary.rows)?;
                increment(&mut summary.kept)?;
                pending.push(record);
                if pending.len() == BATCH_ROWS {
                    batch::add(groups, std::mem::take(pending), workers)?;
                }
            }
            Err(ParseError::EmptyRecord) => (),
            Err(ParseError::LowConfidence | ParseError::NonVegetation) => {
                increment(&mut summary.rows)?;
                increment(&mut summary.filtered)?;
            }
            Err(error) => return Err(format!("{}:{line_number}: {error:?}", path.display())),
        }
    }
    Ok(())
}

fn increment(value: &mut u64) -> Result<(), String> {
    *value = value.checked_add(1).ok_or("row counter overflow")?;
    Ok(())
}

#[cfg(test)]
mod tests;
