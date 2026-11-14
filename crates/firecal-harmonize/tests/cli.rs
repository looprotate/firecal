//! The count, estimated-sigma and FRP binary writes real files that the shared reader and query can consume.

#[cfg(test)]
mod support;
use firecal_query::{Rectangle, SeriesOut, series};
use firecal_types::{MonthIndex, Plane, TileId, TileView};
use std::process::{Command, Output};
use support::Fixture;

#[cfg(test)]
fn run(output: &std::path::Path, flag: &str, files: &[std::path::PathBuf]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_firecal-harmonize"))
        .args(["build", flag])
        .arg(output)
        .args(["0", "0", "276", "3", "0", "2", "276", "2026-11-14"])
        .args(files)
        .output()
        .unwrap()
}

#[test]
fn cli_builds_count_region_in_path_with_spaces_and_rejects_overwrite() {
    let fixture = Fixture::new("cli-region");
    let output = fixture.root.join("salida con espacio ñ");
    let result = run(&output, "--complete-standard-tile", &fixture.inputs);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let bytes = std::fs::read(output.join("t_0_0.bin")).unwrap();
    let view = TileView::parse(&bytes).unwrap();
    let area = Rectangle::new(80.0, 90.0, -180.0, -170.0).unwrap();
    let mut values = [0.0; 3];
    series(
        &[(TileId::new(0, 0).unwrap(), view)],
        &area,
        MonthIndex::from_index(276),
        Plane::Count,
        SeriesOut {
            value: &mut values,
            sigma: None,
        },
    )
    .unwrap();
    assert_eq!(
        values.map(f64::to_bits),
        [6.0_f64, 0.0, 5.0].map(f64::to_bits)
    );
    let mut power = [0.0; 3];
    series(
        &[(TileId::new(0, 0).unwrap(), view)],
        &area,
        MonthIndex::from_index(276),
        Plane::Energy,
        SeriesOut {
            value: &mut power,
            sigma: None,
        },
    )
    .unwrap();
    assert_eq!(
        power.map(f64::to_bits),
        [60.0_f64, 0.0, 15.0].map(f64::to_bits)
    );
    let index = std::fs::read_to_string(output.join("index.json")).unwrap();
    assert!(index.contains("\"development_only\":true"));
    assert!(index.contains("\"sample_count\":1600"));
    assert!(index.contains("\"planes\":[\"count\",\"sigma\",\"energy\"]"));
    assert!(index.contains("\"real_holdout_coverage\":\"not_validated\""));
    assert_eq!(
        view.value(Plane::Sigma, MonthIndex::from_index(276), 0, 0)
            .unwrap(),
        2
    );
    assert_eq!(
        view.value(Plane::Sigma, MonthIndex::from_index(278), 0, 0)
            .unwrap(),
        4
    );
    assert!(
        !run(&output, "--complete-standard-tile", &fixture.inputs)
            .status
            .success()
    );
    assert_eq!(bytes, std::fs::read(output.join("t_0_0.bin")).unwrap());
    // Input order cannot change either the binary or its calibration metadata.
    let mut reversed = fixture.inputs.clone();
    reversed.reverse();
    let second = fixture.root.join("reordered");
    assert!(
        run(&second, "--complete-standard-tile", &reversed)
            .status
            .success()
    );
    assert_eq!(bytes, std::fs::read(second.join("t_0_0.bin")).unwrap());
    assert_eq!(
        index,
        std::fs::read_to_string(second.join("index.json")).unwrap()
    );
}

#[test]
fn cli_rejects_unasserted_coverage_duplicate_or_damaged_inputs_before_writing() {
    let fixture = Fixture::new("cli-invalid");
    let output = fixture.root.join("output");
    assert!(
        !run(&output, "--country-subset", &fixture.inputs)
            .status
            .success()
    );
    assert!(!output.exists());
    let mut duplicated = fixture.inputs.clone();
    duplicated.push(fixture.inputs[0].clone());
    assert!(
        !run(&output, "--complete-standard-tile", &duplicated)
            .status
            .success()
    );
    assert!(!output.exists());
    let damaged = fixture.root.join("damaged.fca");
    std::fs::write(&damaged, b"bad").unwrap();
    assert!(
        !run(&output, "--complete-standard-tile", &[damaged])
            .status
            .success()
    );
    assert!(!output.exists());
    assert!(
        !Command::new(env!("CARGO_BIN_EXE_firecal-harmonize"))
            .output()
            .unwrap()
            .status
            .success()
    );
}
