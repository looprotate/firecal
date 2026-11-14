# Measured ingest numbers for Eminem (#27)

Measured by Dre on 2026-11-14, Linux x86_64 with three logical CPUs visible,
Rust release build `215a276e71a2e4e966aa192f16a42bd486515b1b`.
The source, commands, all repetitions and limitations are in
`crates/firecal-ingest/archive-run.md`.

## Candidate regional number

For the aligned Uruguay 2023–2025 handoff, ingest processed **16,926 real FIRMS
rows**, retained **15,287 detections**, and wrote **142 satellite-month FCA1
files** totaling **88,760 bytes**. Three four-worker runs took 15, 13 and 14 ms;
median **14 ms** and median **1,201,973 rows/second**. Inputs were 1,375,416 bytes.

These are raw per-satellite aggregate files, not harmonized tiles. This number
does not measure browser queries or harmonization agreement. It is a small,
warm-cache country dataset in a shared environment; do not extrapolate to the
world or claim a general parallel speedup.

## Larger diagnostic run

The complete available public Uruguay archive (2000–2025) has **48 input files**,
**7,834,182 bytes**, **97,392 rows**, **91,094 kept detections**, and **839 FCA1
outputs** totaling **533,988 bytes**. Three four-worker runs: 78, 79 and 87 ms;
median **79 ms**, median **1,225,223 rows/second**. All six one-/four-worker runs
were byte-identical and all records matched an independent input oracle.

This full archive mixes MODIS product versions; it is a throughput/raw-count
diagnostic, not a uniform MODIS Collection 6.1 history. Do not describe these
839 files as a calibrated calendar. NOAA-21 is absent from these public sources.

Copy any numbers used in the pitch into `pitch/numbers.md` with this source and
the qualifications above; Dre does not own that directory. The requested
South America 2026 download in #23 is separate and is not included here.
