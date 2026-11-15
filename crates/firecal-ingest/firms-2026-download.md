# Issue #23: South America FIRMS CSV download

Completed on 2026-11-14 through the official area CSV API. The user's requested
bounding box is west -82, south -56, east -34, north 13, for 2026-06-23 through
2026-11-14 inclusive. The region is this bounding box, not a political-country
mask. No archive approval was needed for the API route.

## Results

- **116 CSV files**, **3,006,747 raw detections**, **251,108,012 bytes**.
- Started 2026-11-14 18:47:37 UTC; finished 18:54:42 UTC.
- Elapsed **425.259204 seconds** (7 minutes 5 seconds), including writing,
  validation, checkpoints and two-second spacing between sequential requests.
- Summed per-file request-and-validation time: **192.594547 seconds**. This is
  not a pure network benchmark; the remainder includes deliberate spacing.
- Each request covers one to five days. File inventory, exact byte sizes,
  record counts and per-file times are in `firms-2026-files.md`.

The five satellite identifiers were actually observed: Terra, Aqua, N20, N21
and N (S-NPP). MODIS returns Terra/Aqua together; the three VIIRS platforms are
requested separately. The N21 alias exposed by real API input now has a synthetic
regression test and maps to the existing `Satellite::Noaa21` enum variant.

## Selected availability and processing

The API availability snapshot was consulted before planning. For each sensor
and day, SP was preferred where available, otherwise NRT. Every available
requested sensor-day is assigned exactly once; blocks do not overlap.

| Source | Selected dates (inclusive) | CSVs | Raw records | Bytes |
|---|---|---:|---:|---:|
| MODIS_SP | 2026-06-23–2026-08-31 | 14 | 75,595 | 6,069,493 |
| MODIS_NRT | 2026-09-01–2026-11-14 | 15 | 171,741 | 14,899,387 |
| VIIRS_NOAA20_SP | 2026-06-23–2026-07-31 | 8 | 114,275 | 9,328,536 |
| VIIRS_NOAA20_NRT | 2026-08-01–2026-11-14 | 22 | 852,176 | 71,873,776 |
| VIIRS_NOAA21_SP | 2026-06-23–2026-07-31 | 8 | 109,185 | 8,912,646 |
| VIIRS_NOAA21_NRT | 2026-08-01–2026-11-14 | 22 | 822,928 | 69,397,261 |
| VIIRS_SNPP_SP | 2026-06-23–2026-07-31 | 8 | 96,900 | 7,711,580 |
| VIIRS_SNPP_NRT | 2026-08-01–2026-11-02 | 19 | 763,947 | 62,915,333 |

S-NPP is unavailable for **2026-11-03–2026-11-14** according to the provider's
availability response, consistent with the reported end of delivery. Those 12
days are explicitly recorded as unavailable; they are not filled with zeros or
invented observations. Other sources reach the requested 2026-11-14 endpoint.

June is partial because the request starts on the 23rd. November is partial;
the 14th is the current day at retrieval time and is not a finalized full day.
S-NPP's November coverage stops on the 2nd. API date availability and response
success do not establish cloud-free observation coverage for each cell/day.

Observed SP versions are MODIS `61.03` and VIIRS `2`. Provisional responses
include MODIS `6.1NRT`, VIIRS `2.0NRT`, and some NOAA-20 `2.1URT` records served
by the combined NRT endpoint. Preserve this provenance and the retrieval time.
SP CSVs contain `type`; NRT responses omit it, so the source-type filter cannot
be applied there. Confidence and day/night fields are present in every CSV.
Do not claim identical processing quality or vegetation-only filtering for NRT.

## Storage and verification

The worktree and canonical shared checkout contain the same verified originals:

```text
data/raw/firms-south-america-20260623-20261114/*.csv
data/firms-2026/availability.csv
data/firms-2026/availability.json
data/firms-2026/download-plan.json
data/firms-2026/download-manifest.json
data/firms-2026/verified-summary.json
data/firms-2026/aggregate-manifest.json
```

The manifest records source, requested start/end, row count, exact bytes,
SHA-256, per-file elapsed time, retrieval timestamp, header and observed
satellite/product-version metadata. It contains no MAP_KEY or credential URL.
Every file's hash/size, required columns, row date window and coordinate bounds
were checked. All 116 expected requests are present. A separate audit verifies
the planned availability coverage and absence of sensor-day overlaps.

Only the dedicated raw directory is an ingestion input. Do not recursively
ingest all of `data/`, which also contains validation copies and older datasets.
Raw originals are preserved and ignored by git; no real records become fixtures.
The credential was supplied in the downloader process environment and was not
saved to source files, reports, manifests or logs. Saved public links and a
credential-free query template are in `firms-api-access.md`.

## Full ingestion check

The release executable consumed all 3,006,747 rows without malformed-record
errors, kept **2,735,338** and filtered **271,409**, producing **30 FCA1 files**
(five satellites by six months), totaling **3,142,600 bytes**:

```text
data/aggregates-south-america-20260623-20261114/<satellite>/YYYYMM.fca
```

One-worker execution: 1,980 ms / 1,517,918 rows/s. Four-worker execution:
2,181 ms / 1,378,323 rows/s. These single warm-cache validation runs do not
demonstrate a parallel speedup. Every output file is byte-identical between
the two runs, and all satellite/month detection and daytime totals match an
independent CSV confidence/type filter. The 30 files are copied to the canonical
checkout without overwriting the earlier Uruguay handoff.

These are raw satellite aggregates, not harmonized tiles. Per-satellite NRT
boundaries and partial/unavailable periods must remain visible downstream;
Cube's use of them is recorded in `crates/firecal-harmonize/south-america-validation.md`. No file-format
change is made.
