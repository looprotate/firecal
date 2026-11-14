//! Opt-in validation against local NASA CSVs; no external records enter git or fixtures.

use firecal_ingest::{Aggregator, CsvHeader, ParseError};
use firecal_types::{MonthIndex, Satellite, aggregate_len};
use std::{collections::BTreeMap, fs, path::PathBuf, process::Command};

#[test]
#[ignore = "set FIRECAL_VALIDATION_INPUT to a flat directory containing one real FIRMS month"]
fn real_month_matches_a_simple_serial_oracle() {
    let input = PathBuf::from(
        std::env::var_os("FIRECAL_VALIDATION_INPUT").expect("local real-month directory"),
    );
    let mut paths: Vec<_> = fs::read_dir(&input)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "csv"))
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "real CSV input is required");
    let mut serial: BTreeMap<(Satellite, MonthIndex), Aggregator> = BTreeMap::new();
    let mut month = None;
    let mut rows = 0_u64;
    let mut kept = 0_u64;
    for path in paths {
        let file = fs::read(path).unwrap();
        let mut lines = file.split(|byte| *byte == b'\n');
        let header = CsvHeader::parse(lines.next().unwrap()).unwrap();
        for line in lines {
            match header.parse_record(line) {
                Ok(record) => {
                    if let Some(previous) = month {
                        assert_eq!(record.month, previous, "input must contain one month");
                    }
                    month = Some(record.month);
                    serial
                        .entry((record.satellite, record.month))
                        .or_insert_with(|| Aggregator::new(record.satellite, record.month))
                        .add(&record)
                        .unwrap();
                    rows += 1;
                    kept += 1;
                }
                Err(ParseError::EmptyRecord) => (),
                Err(ParseError::LowConfidence | ParseError::NonVegetation) => rows += 1,
                Err(error) => panic!("real input is malformed: {error:?}"),
            }
        }
    }
    assert!(
        !serial.is_empty(),
        "real input must include kept detections"
    );
    let root = std::env::temp_dir().join(format!("firecal-real-month-{}", std::process::id()));
    fs::create_dir(&root).unwrap();
    for workers in [1, 4] {
        let output = root.join(format!("workers-{workers}"));
        let result = Command::new(env!("CARGO_BIN_EXE_firecal-ingest"))
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
        assert!(summary.contains(&format!(
            "rows={rows} kept={kept} filtered={} files={}",
            rows - kept,
            serial.len()
        )));
        let files = fs::read_dir(&output)
            .unwrap()
            .map(|directory| fs::read_dir(directory.unwrap().path()).unwrap().count())
            .sum::<usize>();
        assert_eq!(files, serial.len());
        for ((satellite, month), aggregator) in &serial {
            let name = match satellite {
                Satellite::Terra => "terra",
                Satellite::Aqua => "aqua",
                Satellite::SuomiNpp => "snpp",
                Satellite::Noaa20 => "noaa20",
                Satellite::Noaa21 => "noaa21",
            };
            let mut expected = vec![0; aggregate_len(aggregator.records().unwrap().len()).unwrap()];
            aggregator.write(&mut expected).unwrap();
            let filename = format!("{:04}{:02}.fca", month.year(), month.month());
            assert_eq!(
                fs::read(output.join(name).join(filename)).unwrap(),
                expected,
                "{name} / {workers} workers"
            );
        }
        println!(
            "workers={workers}: {} (all {} FCA1 files match the independent serial oracle)",
            summary.trim(),
            serial.len()
        );
    }
    fs::remove_dir_all(root).unwrap();
}
