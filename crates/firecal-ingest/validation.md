# Issue #26: real-month ingestion validation

Validated on Linux on 2026-11-14 using official public NASA FIRMS country
downloads. These files contain individual hotspot detections, despite the
download page's "yearly summary" label. This validation does not replace the
requested South America download in #23.

## Sources and preparation

Downloaded the following official originals and preserved them unchanged under
the ignored `data/validation/originals/` directory:

- MODIS, Uruguay, 2024: 47,320 bytes.
  https://firms.modaps.eosdis.nasa.gov/data/country/modis/2024/modis_2024_Uruguay.csv
- VIIRS NOAA-20, Uruguay, 2024: 121,605 bytes.
  https://firms.modaps.eosdis.nasa.gov/data/country/viirs-jpss1/2024/viirs-jpss1_2024_Uruguay.csv
- VIIRS S-NPP, Uruguay, 2024: 110,910 bytes.
  https://firms.modaps.eosdis.nasa.gov/data/country/viirs-snpp/2024/viirs-snpp_2024_Uruguay.csv

See `docs/references.md` at the repository root for the official file listing
and attribute/provenance documentation. Only January 2024 rows were selected,
using the original header and `acq_date` prefix `2024-01-`. A CSV reader/writer
preserved field values and emitted LF line endings. Originals were not modified.
The resulting flat input directory is `data/validation/january-2024/`.

| Input | January rows | Observed version field | SHA-256 of monthly CSV |
|---|---:|---|---|
| MODIS | 84 | `61.03` | `180082737ab6d3de62203fc87cfd9d6f784c4cd1b48fe833c38ba2ade1b2bf0f` |
| NOAA-20 | 60 | `2` | `cebd60718ff8d99b7f4e9cfaea9aeac5722ba151a4b30f19a6938268cbc1684b` |
| S-NPP | 62 | `2` | `238f0b3a482d903ad9ba64163fa0ae2afb003accdb364e290697111f8ae07f51` |

The NOAA-20 file uses satellite identifier `N20`. A synthetic regression test
first failed with `InvalidSatellite`; accepting that alias corrected the real
input without changing any shared contract. No NASA records became fixtures.

## Procedure and evidence

With `FIRECAL_VALIDATION_INPUT` set to the monthly directory, run:

```text
cargo test --release -p firecal-ingest --test real_month -- --ignored --nocapture
```

The opt-in test first parses the input into a simple serial oracle with no
batches or worker scheduling. It then executes the release binary with 1 and
4 workers, checks the emitted file count, and compares every file's full bytes
with the independent serial encoding. A mismatch fails the test. Temporary
outputs are removed on success; original input files are preserved.

Result: **206 rows, 113 kept, 93 filtered, four satellite-month files** for
January 2024 (Terra, Aqua, S-NPP, NOAA-20). All four files matched the serial
oracle byte for byte for both worker counts. NOAA-21 was not part of this public
validation input; its synthetic coverage remains in the normal suite.

Recorded end-to-end release execution, including CSV reading and FCA1 writes:

| Requested workers | Integer elapsed milliseconds | Rows/second |
|---:|---:|---:|
| 1 | 1 | 103,185 |
| 4 | 0 | 280,404 |

Rates use monotonic nanosecond timing; displayed milliseconds are truncated, so
0 ms means sub-millisecond execution. These are single runs on a tiny country
month with warm caches, not a reliable speedup estimate or a world-scale pitch
claim. The earlier 100,000-row synthetic run took 38 ms with 1 worker and 41 ms
with 4, showing that scheduling overhead can outweigh parallel gains. The real
archive's throughput and the world run remain the work of #27.
