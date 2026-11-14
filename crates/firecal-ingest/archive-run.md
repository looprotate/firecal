# Issue #27: Uruguay archive run and regional handoff

Run date: 2026-11-14. Code: `215a276e71a2e4e966aa192f16a42bd486515b1b`.
Environment: Linux 6.18.57, x86_64, glibc 2.41, three logical CPUs visible,
Rust 1.99.0, release profile. This is a measured regional run, not a world run.

## Inputs and version separation

Downloaded all 48 available public Uruguay country CSVs: MODIS 2000–2025,
VIIRS S-NPP 2012–2025 and VIIRS NOAA-20 2018–2025. Original files total
7,834,182 bytes and contain 97,392 rows. Sources are the official country page
and file listing cited in `docs/references.md`; URL pattern:

```text
https://firms.modaps.eosdis.nasa.gov/data/country/<product>/<year>/<filename>
```

Products are `modis`, `viirs-snpp` and `viirs-jpss1`; filenames are
`<product>_<year>_Uruguay.csv`. The archived source manifest records every URL,
byte size, listing timestamp and SHA-256. Originals remain unchanged in the
ignored `data/raw/uruguay/` directory. Preparation took 3.828 seconds, including
45 network downloads and three copies of previously downloaded 2024 files;
this is not a clean network-only download benchmark.

The public MODIS files have different version fields: `6.2` in 2000–2017,
`6.03` in 2018–2022 and `61.03` in 2023–2025. VIIRS version is `2`. These fields
are recorded verbatim; do not treat the older public inputs as uniform MODIS
Collection 6.1. There is no NOAA-21 country file in the listing used here.

Two outputs are deliberately separate:

- `data/aggregates-history-public/`: the full available public archive, for raw
  count/performance diagnostics. It is **not** a uniform calibration dataset.
- `data/aggregates/`: only the aligned 2023–2025 inputs (MODIS `61.03`, VIIRS
  `2`), prepared for Cube. Inputs are nine CSVs totaling 1,375,416 bytes. This
  subset is also copied to the canonical checkout's ignored `data/aggregates/`.

Neither dataset contains harmonized tiles. Missing zero-kept satellite-months
are omitted by ingest. NOAA-21 is absent coverage, not a measured zero series.

## Measured runs

Commands use fresh output directories and the built release executable; build,
download, validation and packaging time are excluded. CLI timing includes CSV
reading/parsing, aggregation and FCA1 writes. Inputs were warm in the OS cache.

```text
cargo build --release -p firecal-ingest
target/release/firecal-ingest data/raw/uruguay data/aggregates-history-public 4
target/release/firecal-ingest data/raw/uruguay-2023-2025 data/aggregates 4
```

Full public archive, 97,392 rows, 91,094 kept, 6,298 filtered, 839 output files:

| Workers | Repetition | CLI elapsed ms | Rows/second |
|---:|---:|---:|---:|
| 1 | 1 | 109 | 889,787 |
| 1 | 2 | 90 | 1,081,906 |
| 1 | 3 | 81 | 1,195,322 |
| 4 | 1 | 78 | 1,243,068 |
| 4 | 2 | 79 | 1,225,223 |
| 4 | 3 | 87 | 1,118,513 |

Four-worker medians: **79 ms, 1,225,223 rows/s**. The 839 files occupy 533,988
bytes, containing 32,745 cell-month records and 82,252 daytime detections.
All six runs produce identical file sets and bytes.

Aligned 2023–2025 subset, 16,926 rows, 15,287 kept, 1,639 filtered, 142 files:

| Workers | Repetition | CLI elapsed ms | Rows/second |
|---:|---:|---:|---:|
| 4 | 1 | 15 | 1,081,643 |
| 4 | 2 | 13 | 1,266,332 |
| 4 | 3 | 14 | 1,201,973 |

Medians: **14 ms, 1,201,973 rows/s**. These are short runs on a small regional
dataset in a shared environment; rates vary with caching, scheduling and I/O.
They do not establish global throughput or a general parallel speedup.

## Correctness checks

An independent Python CSV oracle processed all original rows in sorted file
order, filtered confidence/type, computed grid cells with decimal arithmetic,
and accumulated counts, daytime detections and binary64 FRP in input order.
Every FCA1 header, sorted cell index, count, daytime count and final binary32
FRP matched that oracle. All expected keys were present with no extra records.

Each aligned file is byte-identical to its independently audited counterpart
in the full archive, and three aligned runs are byte-identical. Aligned totals:

| Satellite | Files | Kept detections | Day detections | Cell-month records |
|---|---:|---:|---:|---:|
| Terra | 34 | 372 | 296 | 232 |
| Aqua | 36 | 1,479 | 1,431 | 783 |
| S-NPP | 36 | 6,652 | 5,740 | 2,156 |
| NOAA-20 | 36 | 6,784 | 5,805 | 2,270 |
| Total | 142 | 15,287 | 13,272 | 5,441 |

Aligned FCA1 bytes total **88,760**, before archive compression. This PR adds
only a report, citations and handoff requests; no new runtime code or contracts
were changed. Existing tests and the mandatory `cargo xtask ready` gate apply.

## Deliverables

Cube's recovery instructions are in `requests/dre-to-cube-uruguay-aggregates.md`.
Eminem's measured numbers are in `requests/dre-to-eminem-uruguay-numbers.md`.
The shared checkout has `data/aggregates/` and `data/handoffs/` ready locally.

The remote regional handoff is stored in a draft data release:
https://github.com/looprotate/firecal/releases/tag/untagged-2791c568ae3c9de19614

GitHub's binary upload endpoint rejected the integration with HTTP 401, so the
exact regional gzip archive is encoded in that draft's notes, retrieved via
the normal repository API. It is not a git file or a published software release.
The round trip was verified against its archive checksum and all 142 local
FCA1 files. GitHub authentication with access to draft releases is needed.

Regional archive: `uruguay-2023-2025-aggregates.tar.gz`, 51,180 bytes.
SHA-256: `226ba6065fab16e7cd7a4b94e0736441a0ff8ef9075b78df2c21d7f59093c638`.
It contains the FCA1 files, source/output manifests and README, without raw CSVs.

The mixed-version diagnostic archive is retained locally as
`data/handoffs/uruguay-2000-2025-public-diagnostic.tar.gz`, 280,901 bytes.
SHA-256: `f6f40f508e120ff04fc7e1bd70271b537cd885a922476e6796b7856e10babc05`.
Do not use it for calibration without first resolving collection provenance.

The requested June–November 2026 South America files remain separate work in
#23. A full world run and harmonization agreement are not claimed here.
