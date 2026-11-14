# Count-only tile writer (#31)

This development pipeline implements `fit_aggregates` and
`apply(calibration, plan, aggregates) -> CountTile`, without changing FCA1/FCT1.
`CountTile::encode` delegates every binary byte to `firecal-types::write_tile`.

## Run

```text
cargo run -p firecal-harmonize -- build --complete-standard-tile OUTPUT ROW COL FIRST_MONTH MONTH_COUNT MODIS_CODE VIIRS_CODE TRAIN_THROUGH YYYY-MM-DD FILE.fca...
```

Indices and satellite codes are those of `docs/data-format.md`. For example,
`0 0 276 3 0 2 276 2026-11-14` selects tile (0,0), January–March 2023,
Terra and S-NPP, fitting only on the January overlap. Give the paths to the
corresponding FCA1 files as the final arguments, in any order. The output
folder must not exist; input paths and output paths may contain spaces or
non-ASCII characters. The build date is explicit, so replaying the same inputs
and arguments gives identical tile and index bytes.

The flag is the caller's assertion that each supplied satellite-month has
complete, science-quality coverage of the entire tile. It is not something the
program can infer from sparse records. Never use it for country-clipped data
such as Dre's Uruguay handoff: absence outside the country is unknown, not zero.
Missing zero-kept satellite-month files must be reconstructed as explicit empty
FCA1 files only when external coverage records establish that they are known
zeros. A file absent from disk never asserts zero coverage.

The requested range is nonempty and cannot overflow `MonthIndex`. Every month
requires at least one selected source file; extra satellites, out-of-range
months and duplicate satellite-month files are errors. The cutoff must lie
inside the range. Fit uses paired months from 2012 onward through that cutoff;
other months remain outside training. Source satellites are never summed.

## Source precedence and encoding

- If the MODIS file exists, its entire tile is authoritative, including cells
  with zero counts. A VIIRS detection never overrides a known MODIS zero.
- If the MODIS file is absent and VIIRS exists, apply the fitted VIIRS mapping.
- If both are absent, fail before creating the output directory.
- Round half counts upward, then saturate at 65535. Source u32 counts are never
  truncated. The bounded integer search avoids lossy floating-point casts.
- A tile whose encoded counts are all zero is omitted from the file list.
  Otherwise write `t_<row>_<col>.bin`. Write `index.json` last: a partial build
  without an index is not a completed dataset. An I/O failure can leave a partial
  fresh directory; inspect it and choose a new destination for a retry.

`index.json` records extent, available planes, date, complete-tile assertion,
source pair, calibration version, fitted knots, number of samples, exact paired
training months and the training cutoff. The binary computes these together;
when using `CountTile::index_json` directly, pass the same calibration and
cutoff used by `fit_aggregates`/`apply` so provenance remains consistent.

## Deliberate development limit

FCT1 requires three physical planes. In this count-only milestone, sigma and
energy are zero storage placeholders, **not estimates and not observations**.
The index therefore declares `planes: ["count"]`, lists the three physical
`stored_planes`, and sets `development_only: true`. Only count queries with
`SeriesOut { sigma: None, .. }` are supported. Do not publish these tiles to the
application or use the placeholder planes for anomalies or energy. Their real
models arrive in #32 and #33. No app loader or query contract was changed to
interpret placeholders automatically.

## Evidence

The synthetic complete-tile fixture has MODIS `[2,4]`, VIIRS `[4,8]` in January;
MODIS is explicitly empty in February despite VIIRS count 8; March is VIIRS-only
with `[3,6]`. January learns a half-count map. The expected count series is
`[6,0,5]`: March's 1.5 rounds to 2 and 3 stays 3. All other tile cells are known
zeros in this synthetic fixture, including those used for training.

Unit and executable tests read the actual encoded files through
`firecal-types::TileView` and sum the region using `firecal-query::series`.
They also check byte reproducibility, duplicate/unknown months, unsupported
positive VIIRS, u32 saturation, half rounding, the last polar/antimeridian cell,
empty tiles, invalid dates, malformed files and protection of existing outputs.
No real FIRMS accuracy, uncertainty coverage or full Uruguay tile coverage is
claimed by this fixture. The subsequent real-data run uses 24 fully covered interior tiles from
Dre's South America bounding-box release; see
`south-america-validation.md` for the audit, exact counts and remaining limits.
