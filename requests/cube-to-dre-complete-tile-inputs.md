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
