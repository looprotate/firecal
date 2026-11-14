# Monthly overlap report (#34)

The native `report` subcommand writes monthly per-tile raw MODIS, raw VIIRS and
mapped VIIRS counts for the before/after figure. All counts are evaluated before
FCT1 rounding and saturation. This is not an FRP report or a preferred-source
query: VIIRS is mapped even when a MODIS file exists, so disagreement stays visible.

## Command and coverage

```text
cargo run -p firecal-harmonize -- report --complete-standard-tile OUTPUT.csv FIRST_ROW FIRST_COL ROW_COUNT COL_COUNT FIRST_MONTH MONTH_COUNT MODIS_CODE VIIRS_CODE TRAIN_THROUGH FILE.fca...
```

The output must not already exist; its parent directory must exist. Paths with
spaces and non-ASCII characters work. No date argument is required for a report.
The existing `build` command keeps its original arguments and behavior.

The flag asserts complete science-quality coverage of every selected tile in
each supplied satellite-month. It must not be used for country-clipped data or
provisional/partial source months. Satellite codes and month indices are the
shared contract's values. The rectangular tile selection must be nonempty and
inside the 18-by-36 tile grid. Library callers may pass unique plans in any order;
output is always sorted by tile row, tile column and month.

For each tile the existing count calibration fits paired months from 2012 onward
through the cutoff. CSV rows exist only when both source files exist. An explicit
empty FCA1 file is a known zero; an absent file produces no overlap row. The
function does not invent missing observations. Unsupported positive VIIRS input
remains an error instead of an apparently successful zero.

## CSV interpretation

Header:

```text
tile_row,tile_col,year,month,month_index,modis_satellite,viirs_satellite,split,raw_modis_count,raw_viirs_count,harmonized_viirs_count
```

Raw totals are u64 integers, so values above the tile's 65535 encoding maximum
are preserved. Harmonized totals use six decimal places; CSV formatting has at
most half a millionth of a count of decimal-rounding error per row. The internal
sum follows a deterministic cell order. Line endings are LF with a final newline.

`split=training` means the month participated in fitting. `split=held_out` means
it did not. Neither label is a claim of accuracy. A held-out known-zero MODIS
month paired with nonzero VIIRS still reports the actual nonzero mapped VIIRS
estimate; the report never substitutes MODIS to create artificial agreement.

The hand-declared fixture trains January on MODIS `[2,4]` and VIIRS `[4,8]`, then
reserves February with MODIS `[1,3]` and VIIRS `[2,6]`. Raw VIIRS monthly totals
12 and 8 map to 6 and 4, matching the reference totals within absolute tolerance
1e-12. This arithmetic tolerance bounds floating roundoff for the small exact
half-count fixture; it is not a fitted allowance for sensor error. A separate
negative fixture preserves a difference (MODIS 0, raw VIIRS 8, mapped VIIRS 4).
Tests also cover missing months, input order, duplicate/empty selections, invalid
extents, large raw counts, source-file errors and protection of existing CSVs.
A deliberate sign reversal in mapped accumulation fails the agreement test.

## South America result, 2026-11-14

Using the already audited release, selected complete interior rows 8–13 and
columns 10–13, with July/August standard Terra and July standard S-NPP. Command
arguments after the output path are `8 10 6 4 318 2 0 2 318`, followed by:

```text
aggregates/terra/202607.fca
aggregates/terra/202608.fca
aggregates/snpp/202607.fca
```

The CSV has 24 rows: July only, all labeled training. August VIIRS is provisional
and was deliberately not supplied as standard overlap. Totals: raw MODIS 7000,
raw VIIRS 67025, harmonized VIIRS 7000.000000. Every raw tile total was checked
against the source FCA1 records. Every mapped training total agrees within
1e-6 counts with an independent marginal-mass identity: MODIS total minus any
reference mass lost by the structural-zero anchor. The measured lost mass is
zero here. This checks training arithmetic, not independent real-world accuracy.

Generated local file: `data/south-america-overlap-counts.csv`, 1260 bytes.
SHA-256: `6fc30e3dc8107af656507b671413620d5a5287bb79d54811da30c9f6c4617a57`.
The file stays ignored and is not uploaded or committed. For a figure, label
this sample as July 2026 training overlap; do not present it as held-out evidence.
Independent real accuracy and sigma coverage still need a longer standard history.
