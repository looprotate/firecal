//! Region report CSV execution, missing-source behavior and overwrite protection.

#[cfg(test)]
mod support;
use std::process::Command;
use support::Fixture;

#[test]
fn report_cli_writes_a_region_with_training_labels_and_honest_held_out_difference() {
    let fixture = Fixture::new("report-region");
    let path = fixture.root.join("overlap con espacio ñ.csv");
    let run = |output: &std::path::Path, files: &[std::path::PathBuf]| {
        Command::new(env!("CARGO_BIN_EXE_firecal-harmonize"))
            .args(["report", "--complete-standard-tile"])
            .arg(output)
            .args(["0", "0", "2", "1", "276", "3", "0", "2", "276"])
            .args(files)
            .output()
            .unwrap()
    };
    let result = run(&path, &fixture.inputs);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let csv = std::fs::read_to_string(&path).unwrap();
    assert_eq!(csv.lines().count(), 5);
    assert!(csv.contains("0,0,2023,1,276,0,2,training,6,12,6.000000\n"));
    assert!(csv.contains("0,0,2023,2,277,0,2,held_out,0,8,4.000000\n"));
    assert!(csv.contains("1,0,2023,2,277,0,2,held_out,0,0,0.000000\n"));
    assert!(!csv.contains(",278,"));
    assert!(!run(&path, &fixture.inputs).status.success());
    assert_eq!(csv, std::fs::read_to_string(&path).unwrap());
    let mut reversed = fixture.inputs.clone();
    reversed.reverse();
    let second = fixture.root.join("reversed.csv");
    assert!(run(&second, &reversed).status.success());
    assert_eq!(csv, std::fs::read_to_string(second).unwrap());
}

#[test]
fn report_cli_rejects_invalid_extents_and_duplicates_before_creating_csv() {
    let fixture = Fixture::new("report-invalid");
    let path = fixture.root.join("output.csv");
    for region in [
        ["0", "0", "0", "1"],
        ["17", "0", "2", "1"],
        ["0", "35", "1", "2"],
    ] {
        let result = Command::new(env!("CARGO_BIN_EXE_firecal-harmonize"))
            .args(["report", "--complete-standard-tile"])
            .arg(&path)
            .args(region)
            .args(["276", "3", "0", "2", "276"])
            .args(&fixture.inputs)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(!path.exists());
    }
    let result = Command::new(env!("CARGO_BIN_EXE_firecal-harmonize"))
        .args(["report", "--complete-standard-tile"])
        .arg(&path)
        .args(["0", "0", "1", "1", "276", "3", "0", "2", "276"])
        .args(&fixture.inputs)
        .arg(&fixture.inputs[0])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(!path.exists());
}
