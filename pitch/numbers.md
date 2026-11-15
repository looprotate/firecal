# Measured numbers

The only numbers anyone may say on stage, write in the submission or put on a slide. Each was measured, by the
person or agent named, on the machine named, and says what it does **not** show. A number that is not here is a
placeholder (`[MEASURED: ...]` in `script.md`) and stays unsaid.

Measured on 2026-11-14 at `main` f706aec unless the row says otherwise.

## The story: south-east Australia, November 2000 to December 2019

The demo's history, run end to end by Biggie's agent on 2026-11-14 on an Apple M4. Region: the nine 10° tiles from
20° to 50° south and 130° to 160° east, which hold only Australia and sea, so the public country files cover them
completely. The demo area is west 147, south −38, east 151, north −34 (East Gippsland and the New South Wales south
coast).

| Number | What it is | Measured by, how | It does not show |
|---|---|---|---|
| **30 files, 1.3 GB** | The public FIRMS country archive for Australia: MODIS 2000 to 2020, VIIRS S-NPP 2012 to 2020 | Downloaded from `firms.modaps.eosdis.nasa.gov/data/country/`; SHA-256 of each file kept with the data, outside git | A uniform MODIS collection: the public files carry versions 6.2 and 6.03 |
| **16,605,720 rows in 7.3 s** (2.26 million rows/s) | `firecal-ingest` over those files: 15,076,666 detections kept, 572 aggregate files | One run, eight workers, release build; it needed #116 first, because 159 rows have a negative FRP | A median of repeated runs, or a cold disk |
| **9 tiles of 230 months in 0.23 s** | `firecal-harmonize build`, fitted on 2012 to 2018 (Terra and S-NPP); 7 tiles with hotspots, 2,208,016 bytes each | Median of three runs (0.254, 0.217, 0.232 s), the tiles byte-identical across runs | The world |
| **Raw VIIRS 12.5 times MODIS; harmonized 0.77 times** | **2019, held out of the fit**: MODIS 50,497 hotspots over the 9 tiles, raw VIIRS 631,751, harmonized VIIRS 38,910. In the fit years, harmonized equals MODIS by construction (144,592) | `firecal-harmonize report`, 864 tile-months; the figure is `figures/harmonization-before-after.svg` | That 0.77 holds for a quiet year: 2019 was the most extreme year in the record, past the fit's largest counts, where the mapping holds its last value and so undercounts |
| **0.75 ms** | The demo's query: series, comparison and analog years over 230 months, for the demo area on its 2 tiles; a polygon inside it, 0.42 ms | Chrome 156, `performance.now`, median of seven batches of 20 | A phone |
| **4.21 MiB** | What pinning the demo area stores: 2 tiles of 230 months | The app's "Pinned 2 tiles, 4.21 MiB" | The size after gzip |
| **December 2019: 7,112 hotspots, median 7** | The core flags November and December 2019 as unusual; January 2003 (8,470, the alpine fires) is still the largest January, so January 2020 would not be flagged in this area | The app on the real tiles | A cause, or burned area: hotspots are thermal anomalies |
| **2009, 2001, 2002** | The three past years the core finds most like 2019, by rank within each month | The app | A forecast: they are history |

## The pipeline

| Number | What it is | Measured by, how | It does not show |
|---|---|---|---|
| **97,392 rows in 79 ms** (1,225,223 rows/s) | `firecal-ingest` over the whole public FIRMS archive for Uruguay, 2000 to 2025: 48 files, 7.8 MB, 91,094 detections kept, 839 aggregate files | Dre, Linux x86_64, three logical CPUs, release build, median of three four-worker runs; byte-identical across one and four workers and against an independent oracle (`crates/firecal-ingest/archive-run.md`) | Global throughput, a parallel speedup, or a calibrated calendar: the archive mixes MODIS product versions |
| **24 tiles in 0.062 s** | `firecal-harmonize build` for the 24 complete 10° tiles of the South America download (west −80 to −40, south −50 to 10): fit on July 2026 (Terra and S-NPP), counts, sigma and energy for July and August 2026 | Biggie's agent, Apple M4, release build, 24 separate runs of the binary, median of three repetitions (0.060, 0.062, 0.063 s) | A world run; with two months there is no history to calibrate on |
| **345,888 bytes** | The tiles that build writes: 18 with hotspots, 19,216 bytes each, and 6 empty tiles not written | Same run; matches Cube's figure (`crates/firecal-harmonize/south-america-validation.md`) | Size after the host's gzip |
| **9,600 bytes a month per tile** | The tile format: 1,600 cells, three 16-bit planes. 25 years is 2.75 MiB per 10° tile before gzip | Arithmetic from `docs/data-format.md`, checked by the two-month tile above (16 + 2 × 9,600 = 19,216) | How many tiles a region needs: every tile the area touches |

## Harmonization

| Number | What it is | Measured by, how | It does not show |
|---|---|---|---|
| **MODIS 7,000, raw VIIRS 67,025, harmonized VIIRS 7,000** | Hotspots in July 2026 over the 24 tiles: VIIRS sees about 9.6 times as many, and the mapping brings it back to MODIS | Cube's `firecal-harmonize report`; reproduced byte for byte by Biggie's agent (SHA-256 `6fc30e3d…a57`) | Accuracy: July is the month the mapping was fitted on, so the agreement is by construction. No held-out standard month exists in the data |
| **7,000 and 13,359 hotspots, ±95.6 and ±131.7** | The query over the whole region for July and August 2026, with the estimated sigma | Biggie's agent, in the browser through the wasm core; the same values Cube's native query gave | That sigma covers the real error at a stated rate: only tested on synthetic held-out months (19 of 20 inside two sigma) |

## The browser

| Number | What it is | Measured by, how | It does not show |
|---|---|---|---|
| **13 µs** | One area query by the wasm core: a 4° × 4° box, and a triangle drawn inside it, over the 18 real tiles, two months | Biggie's agent, Chrome 156 on an Apple M4, `performance.now` around 1,000 calls, median of 7 batches | A phone, or 25 years of months (the query reads every month it is given) |
| **0.21 ms** | The same query over the whole 40° × 60° region | Same, 200 calls per batch | The same |
| **8,968 calls, identical bytes** | The native and the wasm builds of the core give the same output for the same inputs | `cargo xtask parity`, on every change to the core; last run at `main` 2026-11-14 | Anything beyond the cases in the parity set |
| **88,474 bytes** (34,153 gzipped) | The wasm core | `web/public/firecal.wasm`, release build | The rest of the page: the app's script is 1.42 MB (365 KB gzipped), most of it the map |
| **Answers with the host stopped** | Pin a region, stop the server, reload, type a new area: the calendar answers from the core | `npm run smoke` in headless Chrome, on macOS, Windows and Linux in CI on every pull request | A real phone in airplane mode; the basemap is not stored, so the map has no land offline |

## The checks

| Number | What it is | Measured by, how |
|---|---|---|
| **393 Rust tests, 102 web tests** | All pass on macOS, Windows and Linux | `cargo xtask check` and `npm test`, at f706aec |
| **3.01:1 and up** | Every step of the calendar ramp against the page, and neighbouring steps apart under protanopia, deuteranopia and tritanopia | `web/tokens/ramp.test.mjs` |

## Not measured yet

- Query time on a phone.
- The world: no world archive was downloaded (the FIRMS archive download needs an Earthdata login and an emailed
  link); the size of the whole world offline is therefore unmeasured too.
- Held-out agreement in a quiet year: the only held-out year, 2019, is the most extreme one.
