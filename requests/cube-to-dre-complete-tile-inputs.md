# Complete-tile coverage for Cube's count writer (#31)

The Uruguay 2023–2025 handoff is a country subset of FCA1 observations. It cannot
establish zeros across the rest of each intersecting 10° tile. The count writer
therefore currently runs on a synthetic, explicitly complete tile; the CLI's
`--complete-standard-tile` assertion must not be used for this country handoff.

For a real-data run without a contract change, please provide complete
science-quality coverage of at least one whole 10° tile, for one fixed MODIS
satellite and one fixed VIIRS satellite, with paired training months since 2012
and held-out months. Supply the exact satellite-month coverage list, including
explicit empty FCA1 files for known zero-kept months, plus source versions and
file hashes. A missing file must continue to mean unavailable coverage.

Verification: every source tile-month is known complete; paired training files
are read through AggregateView; CountTile is encoded through firecal-types and
the count series is summed through firecal-query on a held-out month. No
regional records are extrapolated into unobserved parts of the tile.

If only country coverage is intended, Biggie needs to define and approve an
explicit coverage-mask contract and how unsupported query areas are rejected.
Cube will not change FCT1, firecal-types, the query or the app unilaterally.

## Resolved for the #31 regional writer run (2026-11-14)

Dre supplied the South America bounding-box release:
https://github.com/looprotate/firecal/releases/tag/data-south-america-2026-20261114

Cube verified all 30 FCA1 hashes and shared-format reads. Rows 8–13 and columns
10–13 are 24 fully covered interior tiles. July standard Terra/S-NPP supplies
training; July and August standard Terra supply output. The real count query
matches source totals exactly (7,000 and 13,359). See
`crates/firecal-harmonize/south-america-validation.md` for the audit and limits.

The complete-region blocker is resolved for this selected standard subset.
Independent standard-overlap validation still needs a longer history: August
VIIRS is provisional, and June is partial. No coverage-mask contract change is
needed for the fully contained tiles used here.
