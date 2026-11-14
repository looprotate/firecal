//! Executable checks on synthetic CSVs in isolated temporary directories.

use firecal_ingest::{Aggregator, CsvHeader, ParseError};
use firecal_types::{AggregateView, MonthIndex, Satellite, aggregate_len};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Workspace(PathBuf);
impl Workspace {
    fn new() -> std::io::Result<Self> {
        let path = std::env::temp_dir().join(format!(
            "firecal-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path)?;
        Ok(Self(path))
    }
}
impl Drop for Workspace {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.0) {
            eprintln!("test directory cleanup: {error}");
        }
    }
}

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_firecal-ingest"))
}

#[test]
fn recursive_files_keep_months_separate_and_ignore_non_csv_files() {
    let workspace = Workspace::new().unwrap();
    let input = workspace.0.join("raw");
    fs::create_dir_all(input.join("nested")).unwrap();
    let fixture = include_str!("fixtures/ten-hotspots.csv");
    fs::write(input.join("january.csv"), fixture).unwrap();
    fs::write(
        input.join("nested/february.CSV"),
        fixture.replace("2024-01-", "2024-02-"),
    )
    .unwrap();
    fs::write(input.join("ignored.txt"), "not a CSV").unwrap();
    let output = workspace.0.join("out");
    let result = command().arg(input).arg(&output).arg("3").output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8_lossy(&result.stdout).contains("rows=20 kept=12 filtered=8 files=10")
    );
    for (filename, month) in [("202401.fca", 288), ("202402.fca", 289)] {
        let file = fs::read(output.join("terra").join(filename)).unwrap();
        let view = AggregateView::parse(&file).unwrap();
        assert_eq!(view.month().index(), month);
        assert_eq!(view.records().next().unwrap().count, 2);
    }
}

#[test]
fn aggregation_errors_from_a_worker_do_not_write_output() {
    let workspace = Workspace::new().unwrap();
    let input = workspace.0.join("raw");
    fs::create_dir(&input).unwrap();
    let fixture = include_str!("fixtures/ten-hotspots.csv").replace(",1,D,0", ",1e300,D,0");
    fs::write(input.join("large.csv"), fixture).unwrap();
    let output = workspace.0.join("out");
    let result = command().arg(input).arg(&output).arg("3").output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("EnergyOverflow"));
    assert!(!output.exists());
}

#[test]
fn executable_parallel_outputs_match_a_serial_oracle_for_all_five_satellites() {
    let workspace = Workspace::new().unwrap();
    let input = workspace.0.join("raw files ñ");
    fs::create_dir(&input).unwrap();
    let fixture = include_str!("fixtures/ten-hotspots.csv");
    fs::write(input.join("sample.csv"), fixture).unwrap();
    let mut lines = fixture.lines();
    let header = CsvHeader::parse(lines.next().unwrap().as_bytes()).unwrap();
    let hotspots: Vec<_> = lines
        .filter_map(|line| match header.parse_record(line.as_bytes()) {
            Ok(record) => Some(record),
            Err(ParseError::LowConfidence | ParseError::NonVegetation) => None,
            Err(error) => panic!("invalid fixture: {error:?}"),
        })
        .collect();
    for workers in [1, 2, 7] {
        let output = workspace.0.join(format!("output-{workers}"));
        let result = command()
            .arg(&input)
            .arg(&output)
            .arg(workers.to_string())
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let summary = String::from_utf8(result.stdout).unwrap();
        assert!(
            summary.contains("rows=10 kept=6 filtered=4 files=5"),
            "{summary}"
        );
        assert!(summary.contains("rows_per_second="));
        for (satellite, name) in [
            (Satellite::Terra, "terra"),
            (Satellite::Aqua, "aqua"),
            (Satellite::SuomiNpp, "snpp"),
            (Satellite::Noaa20, "noaa20"),
            (Satellite::Noaa21, "noaa21"),
        ] {
            let mut serial = Aggregator::new(satellite, MonthIndex::from_index(288));
            for record in hotspots.iter().filter(|r| r.satellite == satellite) {
                serial.add(record).unwrap();
            }
            let mut expected = vec![0; aggregate_len(serial.records().unwrap().len()).unwrap()];
            serial.write(&mut expected).unwrap();
            let actual = fs::read(output.join(name).join("202401.fca")).unwrap();
            assert_eq!(actual, expected, "{workers} workers, {name}");
            assert_eq!(
                AggregateView::parse(&actual).unwrap().satellite(),
                satellite
            );
        }
    }
}

#[test]
fn malformed_record_reports_file_and_line_without_creating_output() {
    let workspace = Workspace::new().unwrap();
    let input = workspace.0.join("raw");
    fs::create_dir(&input).unwrap();
    let fixture = include_str!("fixtures/ten-hotspots.csv").replace("-33.5", "999");
    fs::write(input.join("bad.csv"), fixture).unwrap();
    let output = workspace.0.join("output");
    let result = command().arg(input).arg(&output).arg("2").output().unwrap();
    assert!(!result.status.success());
    let error = String::from_utf8(result.stderr).unwrap();
    assert!(
        error.contains("bad.csv:2") && error.contains("InvalidNumber"),
        "{error}"
    );
    assert!(!output.exists());
}

#[test]
fn existing_outputs_are_never_overwritten() {
    let workspace = Workspace::new().unwrap();
    let input = workspace.0.join("raw");
    fs::create_dir(&input).unwrap();
    fs::write(
        input.join("sample.csv"),
        include_bytes!("fixtures/ten-hotspots.csv"),
    )
    .unwrap();
    let output = workspace.0.join("output");
    fs::create_dir(&output).unwrap();
    fs::write(output.join("keep.txt"), "existing data").unwrap();
    let result = command().arg(input).arg(&output).output().unwrap();
    assert!(!result.status.success());
    assert_eq!(
        fs::read_to_string(output.join("keep.txt")).unwrap(),
        "existing data"
    );
    assert!(!output.join("terra").exists());
}

#[test]
fn empty_input_and_invalid_worker_counts_are_actionable_errors() {
    let workspace = Workspace::new().unwrap();
    let input = workspace.0.join("raw");
    fs::create_dir(&input).unwrap();
    let result = command()
        .arg(&input)
        .arg(workspace.0.join("out"))
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("no CSV files"));
    for workers in ["0", "invalid"] {
        let result = command()
            .arg(&input)
            .arg(workspace.0.join("out"))
            .arg(workers)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("workers"));
    }
    assert!(!command().output().unwrap().status.success());
}
