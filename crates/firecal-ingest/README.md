# Ingest FIRMS CSV files

The binary recursively reads CSVs, filters low-confidence and non-vegetation
rows, and writes FCA1 aggregates per satellite, month and cell. It uses the
parser from #24 and the aggregator from #25.

```text
cargo run --release -p firecal-ingest -- data/raw data/aggregates 4
```

The first argument must contain extracted, non-overlapping CSVs. ZIP archives
must be extracted first. The second argument must be a new output directory;
existing output directories and files are never overwritten. The optional
third argument is a positive worker count; the default is the machine's
available parallelism. Raw files are never modified or deleted.

Outputs use `terra`, `aqua`, `snpp`, `noaa20` and `noaa21` subdirectories, with
names such as `202401.fca`. Only groups with kept detections are written.
Statistics report nonblank data rows, kept and filtered detections, output files,
elapsed milliseconds and integer rows per second (including parsing and I/O).

## Reproducibility and limits

- Files are sorted by path, and rows retain their order within each file.
- Input is streamed into batches of at most 4096 kept rows. Accumulated cell
  totals remain resident; memory still scales with occupied satellite-month cells.
- Each satellite-month belongs to one worker within a batch. Its accumulator
  survives between batches, preserving the serial order of floating-point sums.
- Work uses `std::thread::scope` at the binary boundary; the library stays pure.
  A batch containing only one satellite-month cannot exploit multiple workers.
- Malformed input reports the file, physical line number and error kind before
  any output is created. A failure while writing can leave partial new output;
  preserve it for diagnosis and select a new directory when retrying.
- Nested symlinks are not followed. CSV extensions are case-insensitive.
- The command does not deduplicate detections or reconcile standard and NRT
  products. Supply non-overlapping inputs and keep processing provenance outside
  the aggregate format; never silently mix overlapping standard/NRT coverage.
- Normal tests use synthetic fixtures. An opt-in test also validates a local
  real month against an independent serial oracle; no real detections enter git.
  January 2024 Uruguay passed this check; see `validation.md` for the sources,
  measurements, limitations and reproduction command. The wider #23 download
  remains a separate task.

## Verification

`cargo test -p firecal-ingest` checks a simple serial oracle against parallel
FCA1 bytes, sensitive FRP sums across batches, 1/2/7/8 workers, all five
satellites, separate months, nested paths (including spaces and Unicode),
malformed input, worker errors, row-counter overflow, and preserved outputs.
`cargo xtask ready` is required before pushing.

For a flat directory of real CSVs restricted to one month, set
`FIRECAL_VALIDATION_INPUT` and run
`cargo test --release -p firecal-ingest --test real_month -- --ignored --nocapture`.
