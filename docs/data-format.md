# Data format

The contracts between the pieces. Proposed on day one; changing one is a contract change (see `AGENTS.md`).
The shared Rust types in `firecal-types` read and write these formats; nothing else does.

## Grid and time

- Equirectangular, **0.25° cells**: 1440 columns × 720 rows. Cell `(row, col)` covers latitude
  `90 - 0.25*(row+1) .. 90 - 0.25*row` and longitude `-180 + 0.25*col .. -180 + 0.25*(col+1)`.
  `CellIndex = row * 1440 + col`.
- Time is **months**: `MonthIndex = (year - 2000) * 12 + (month - 1)`. The series starts at index 10 (2000-11,
  where the FIRMS archive starts MODIS Collection 6.1) and runs to the latest month available.
- Why 0.25°: coarse enough that VIIRS and MODIS counts per cell are comparable and files stay small; fine enough
  for a regional area. Open to change only before the first full pipeline run.

## Satellites

`Satellite` is an enum, stored as a `u8`: `0` Terra, `1` Aqua (MODIS); `2` S-NPP, `3` NOAA-20, `4` NOAA-21
(VIIRS). Kept apart until harmonization.

What the FIRMS archive page lists (checked 2026-11-13; see `docs/references.md`):

| Product | Archive starts |
|---|---|
| MODIS Collection 6.1 (Terra and Aqua) | 1 November 2000 |
| VIIRS S-NPP 375 m | 20 January 2012 |
| VIIRS NOAA-20 375 m | 1 April 2018 |
| VIIRS NOAA-21 375 m | 17 January 2024 |

The overlap that harmonization is fitted on is therefore 2012 onward. When Aqua starts inside the MODIS
collection is not confirmed: read it from the data.

## Aggregate file `.fca` (ingest → harmonize)

One file per satellite per month: `data/aggregates/<satellite>/<yyyymm>.fca`. Little-endian.

| Offset | Type | Field |
|---|---|---|
| 0 | `u32` | magic `FCA1` |
| 4 | `u8` | satellite |
| 5 | `u8` | reserved, zero |
| 6 | `u16` | month index |
| 8 | `u32` | record count `n` |
| 12 | `n` × 16 bytes | records, sorted by cell |

Record: `cell: u32`, `count: u32` (detections kept), `frp_mw_sum: f32` (fire radiative power, megawatts),
`day_count: u32` (how many of the `count` were daytime).

### Reading the FIRMS CSV

- **Read columns by header name, never by position.** The header row of the real file is the source of truth, and
  MODIS and VIIRS differ. Expected, to confirm against the first real download: MODIS has `latitude`, `longitude`,
  `brightness`, `scan`, `track`, `acq_date`, `acq_time`, `satellite`, `instrument`, `confidence`, `version`,
  `bright_t31`, `frp`, `daynight` (and `type` in the standard product); VIIRS has `bright_ti4` and `bright_ti5`
  in place of `brightness` and `bright_t31`.
- **Confidence differs by sensor.** VIIRS 375 m is low, nominal or high, written `l`, `n`, `h` in the CSV (a
  0, 1, 2 scale in raster products). MODIS is a number from 0 to 100; FIRMS groups it as low below 30, nominal 30
  to 79 and high from 80 (confirm against the MODIS C6/C6.1 user guide).
- **Kept detections:** nominal and high for both sensors; low is dropped. Day and night are both kept.
- **Type:** keep presumed vegetation fire (`type` 0). Volcanoes (1) and offshore sources (3) are dropped. Other
  static land sources (2, such as gas flares) are an open question for the team; the default is to drop them.
- **Standard versus near real time.** Use the **standard** (science quality) product for every month it covers.
  NRT is replaced by standard with about a five month lag, so the newest months can only come from NRT. The
  tiles mark where NRT starts (`nrt_from_month` in `index.json`) and the app labels those months **provisional**.
  Comparing a provisional month with a standard history carries a bias we have not measured: say so, and measure
  it if time allows.
- Decisions recorded here so they are not argued twice.

## Tiles (harmonize → browser)

The world is cut into **10° × 10° tiles** (40 × 40 cells = 1600), 36 × 18 = 648 possible files. The browser
loads only the tiles an area touches.

```
tiles/index.json              months available, calibration version, planes, tile list, build date
tiles/t_<row10>_<col10>.bin   one tile; row10 and col10 are its row and column in the 18 × 36 tile grid
```

A tile with no hotspots in any month is not written.

### Tile file (little-endian)

| Offset | Type | Field |
|---|---|---|
| 0 | `u32` | magic `FCT1` |
| 4 | `u16` | `month_count` |
| 6 | `u16` | `first_month_index` |
| 8 | `u8` | `plane_count` (3) |
| 9 | 7 bytes | reserved, zero |
| 16 | planes, in order below | each `u16` × `month_count` × 1600, month-major: `[month][cell_row][cell_col]` |

Planes:

1. **count**: harmonized hotspot count, MODIS-equivalent, rounded to the nearest integer, saturated at 65535.
2. **sigma**: uncertainty of `count`, one standard deviation, in counts, rounded up, saturated at 65535.
3. **energy**: harmonized fire radiative power in megawatts, rounded and saturated. Harmonized the same way as
   count, on its own calibration.

Served gzip-compressed by the static host. Dense is deliberate: trivial to read in Rust and wasm, and it
compresses well because most cells are zero. Whether the whole world fits offline is **measured on the first
full run**; if not, the user pins regions.

## Harmonization

Unit: **MODIS-equivalent** hotspots per cell-month. VIIRS (375 m) sees many more small fires than MODIS (1 km),
so its raw counts are higher.

Method (to be validated by the harmonize owner; the figures it produces are also pitch material):

1. Aggregate each sensor to cell-month counts, keeping each satellite apart.
2. Fit a mapping VIIRS → MODIS on the **overlap period** (VIIRS S-NPP starts 2012; MODIS still runs), per 10°
   tile so it can differ between savanna and boreal forest. Start with quantile mapping on cell-month counts.
3. Apply it to VIIRS months after MODIS ends, and wherever MODIS is missing.
4. **Sigma** comes from the calibration's residuals over the overlap years in that tile, widened for months far
   outside the range the calibration saw. Where only MODIS exists, sigma is its own measurement floor.
5. Record the calibration version in `tiles/index.json` and ship the per-tile parameters, so any number on
   screen can be traced back.
6. Check, as a test: over the overlap, harmonized VIIRS vs MODIS monthly totals per tile agree within a
   tolerance written in the test with its reason, not guessed.
7. **Effort proxy (stretch):** a per-cell-month estimate of how much the satellites looked, from satellites in
   operation and their overflight frequency by latitude. Cut first if time runs out. It would be a fourth plane
   and a new format version.

## Series (what the core returns for an area)

For an area (lat/lon rectangle or polygon), a month range and a plane (count or energy):

```
months[n]        MonthIndex
value[n]         harmonized value summed over the area
sigma[n]         uncertainty of value; errors of cells add in quadrature (sqrt, no exp or ln)
median[12]       historical median of value for each calendar month, over the baseline years
anomaly[n]       (value - median[month]) / max(median[month], 1)
unusual[n]       true when |value - median[month]| > 2 * sigma[n]
analogs[k]       the k past years most similar to the season in progress: year, distance
```

**Rectangle areas.** A rectangle selects every cell it overlaps with positive area: thinner than a cell selects that
cell, one that only touches a cell's edge does not. `west` is left of `east`, so a rectangle never crosses the
antimeridian. A tile missing from the set counts as zeros (ocean, no hotspots); a tile that is present but lacks a
requested month is an error, so the caller clamps the range to the months in `index.json`. `value` and `sigma` are
`f64`: sums of whole numbers are exact far beyond the world, and `sigma` is the square root of the sum of the cells'
squared sigma, taken with `+ - * /` only. The `sigma` plane is the uncertainty of the count, so an energy series has no
sigma of its own yet.

**Climatology and anomaly.** The median of a calendar month is taken over the baseline years that the history series
holds for it; an even count gives the mean of the two middle values. `anomaly` divides by `max(median, 1)` so a
quiet month cannot explode, and `unusual` is strict: a difference of exactly `2 * sigma` is not flagged. A month in
the range whose calendar month has no baseline value is an error, not a zero.

**Analog years.** The season in progress is January to month `m` of the current year. Each month's rank is taken among
the candidate years and the current one; ties share the middle rank, so the difference of two years is counted in
halves. A baseline year is a candidate only if the series holds all `m` months for it, and the current year is never its
own analog. Years at the same distance come oldest first.

Baseline years: all complete years before the current one, unless the UI picks others. Analog distance is the
sum over months of the difference in **rank within the calendar month** across years, so it does not need
logarithms and treats quiet and busy regions alike.

**Stub for the web owners:** until the core exists, `web/*` read `web/stub/series.json`, a hand-made `Series`
in this shape. The wasm call replaces it with no change to the UI.

## CSV export

`cells.csv`: `year,month,cell_lat,cell_lon,value,sigma` for every cell of the area and every month of the
range. `series.csv`: the `Series` above. Both are aggregates, not raw hotspots: raw per-hotspot history is out
of scope for a static site.

**Open question for the team:** recent raw hotspots would need a FIRMS API key, which cannot live in a static
site. Decide Saturday morning; the default is no.
