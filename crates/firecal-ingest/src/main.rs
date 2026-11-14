//! Streaming FIRMS CSV ingestion at the filesystem, threading and timing boundary.

mod pipeline;

use std::{num::NonZeroUsize, path::PathBuf, process::ExitCode, time::Instant};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ingest: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    if !(2..=3).contains(&arguments.len()) {
        return Err(
            "usage: firecal-ingest <input-directory> <new-output-directory> [workers]".into(),
        );
    }
    let input = PathBuf::from(arguments.first().ok_or("missing input directory")?);
    let output = PathBuf::from(arguments.get(1).ok_or("missing output directory")?);
    let workers = match arguments.get(2) {
        Some(value) => value
            .to_str()
            .ok_or("workers must be a positive integer")?
            .parse::<NonZeroUsize>()
            .map_err(|_| "workers must be a positive integer")?,
        None => std::thread::available_parallelism().unwrap_or(NonZeroUsize::MIN),
    };
    let started = Instant::now();
    let summary = pipeline::execute(&input, &output, workers)?;
    let elapsed = started.elapsed();
    let rate = u128::from(summary.rows) * 1_000_000_000 / elapsed.as_nanos().max(1);
    println!(
        "rows={} kept={} filtered={} files={} elapsed_ms={} rows_per_second={rate}",
        summary.rows,
        summary.kept,
        summary.filtered,
        summary.files,
        elapsed.as_millis()
    );
    Ok(())
}
