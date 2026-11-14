# South America count-writer validation (#31)

Measured in Cube's Codex environment on 2026-11-14, using the release supplied
by Dre and the existing `cube/count-tiles` binary. No source data or generated
tiles are committed. This is an end-to-end count-file validation, not evidence
of out-of-sample calibration accuracy or a validated uncertainty model.

## Input audit

Release: https://github.com/looprotate/firecal/releases/tag/data-south-america-2026-20261114

Archive: `south-america-20260623-20261114-fca.tar.gz`

SHA-256: `197e76b74b3984290c0d44e61a8da51873ec7bc6c9a7093bd55564f9cd1fd50e`

The downloaded archive matches the release's SHA256SUMS file. All 30 extracted
FCA1 files match their byte sizes and SHA-256 hashes in
`aggregate-manifest.json`. Every file also passes the shared
`firecal_types::AggregateView::parse`, with satellite, month and record count
matching its manifest entry. Totals: 3,142,600 FCA1 bytes, 196,390 cell-month
records, 2,735,338 kept detections and 1,836,455 daytime detections. These are
whole-package audit totals across five satellites and six months, not the
count series of the smaller validation region.

The README, availability and source manifests distinguish complete standard
months from partial months, NRT and unavailable S-NPP dates. In particular,
August VIIRS is provisional, and June/November are partial. They were not used
as standard training observations.

## Selected complete region and standard inputs

The source rectangle is west -82°, south -56°, east -34°, north 13°.
Its fully contained tiles are rows 8–13 and columns 10–13, inclusive: 24 tiles
covering west -80° to east -40° and south -50° to north 10°. Boundary tiles
cut by the download rectangle are excluded instead of filling missing coverage
with zeros.

Use only these source files from `aggregates/`:

- `terra/202607.fca`: complete standard MODIS July, month index 318.
- `terra/202608.fca`: complete standard MODIS August, month index 319.
- `snpp/202607.fca`: complete standard S-NPP July, for fitting only.

For every selected tile, run the existing binary with this argument pattern,
substituting its row, column and a fresh output directory:

```text
cargo run -p firecal-harmonize -- build --complete-standard-tile OUTPUT ROW COL 318 2 0 2 318 2026-11-14 aggregates/terra/202607.fca aggregates/terra/202608.fca aggregates/snpp/202607.fca
```

Tile-local calibration uses July's 1600 matched cell-months, including known
zeros, and records training month `[318]`. Both output months have actual MODIS
files, so the source-precedence rule preserves MODIS observations. July is the
only complete standard overlap month in this package; August S-NPP was not
silently treated as a standard held-out month.

## Measured result

- 24 successful builds; 18 nonempty FCT1 binaries, 6 omitted all-zero tiles.
- Total binary size: 345,888 bytes (19,216 bytes per nonempty two-month tile).
- All binaries read back through `firecal_types::TileView::parse`.
- Every nonzero source cell in the selected region agrees with its tile count.
- `firecal_query::series` over `Rectangle::new(-50, 10, -80, -40)`, starting at
  month 318 with `Plane::Count` and no sigma output, returns July 7,000 and
  August 13,359.
- Those totals exactly match an independent iteration over the selected source
  MODIS aggregate records; zero source cells were saturated at 65535.
- Every index declares count-only development output, no NRT output months,
  training month `[318]`, the selected source pair and tile coverage.

The real outputs are kept locally in the checkout's ignored
`data/south-america-standard-validation/`, with original downloaded inputs
under `data/south-america-2026/`.

## What this resolves and what remains

This resolves the missing complete-region data for the #31 file writer. No
new code or format change was needed: the asserted coverage is true for these
interior tiles and selected standard source months.

It does not validate real VIIRS fallback on an independent standard overlap
month, because the package provides only July for standard paired coverage.
Fallback behavior remains covered by the synthetic executable and unit tests.
A longer standard overlap is needed to evaluate calibration accuracy and sigma
coverage. Sigma/energy remain unavailable development placeholders pending
#32/#33; these files must not be published to the app as final science output.
