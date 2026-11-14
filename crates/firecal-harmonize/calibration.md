# Tile quantile calibration specification (#29)

Status: proposed for Biggie and Dre to check against `docs/data-format.md`.
This specifies the count calibration for #30; it does not change FCA1 or FCT1.
The observations below are synthetic, not downloaded FIRMS data.

## Calibration population

Fit independently in each 10° tile (40 × 40 cells). A sample is a cell-month
inside the intersection of the two sensors' known spatial and temporal coverage,
from 2012 onward. Match MODIS and VIIRS on cell and month before sorting their
marginal distributions. Use only complete science-quality overlap months for
fitting; reserve whole months for validation before fitting, never individual
cells from the same month. Record the training and validation months.

Keep satellites separate through ingestion. For the initial calibration, select
one fixed MODIS/VIIRS satellite pair per calibration; do not add satellites as
they launch or treat a change in the pair as the same calibration. Prefer Terra
and S-NPP where coverage allows; a fallback pair requires its own fitted
parameters and provenance. Do not sum the MODIS and mapped VIIRS observations:
when the selected MODIS source covers a cell-month, retain its count; use the
mapped VIIRS count only when that MODIS observation is unavailable.

An absent record in an aggregate is zero only inside coverage independently
known to be complete. An absent file or unavailable satellite is unknown, not
zero. In particular the Uruguay handoff is a country subset, not full coverage
of every 10° tile it intersects. Fit on its documented shared country mask and
label the calibration regional; do not apply it to unobserved parts of the tile.
FCA1 alone does not distinguish missing coverage from a zero-kept month; the
pipeline must receive coverage and source selection at the I/O edge. Never
infer coverage from the first and last nonzero record.

## Empirical quantiles and ties

Let `n` be the number of matched training cell-months. Sort each sensor's counts
independently in ascending order, with equal weights, yielding VIIRS `x[i]`
and MODIS `y[i]`. Accumulate and interpolate in `f64` in this fixed order.

Each order statistic has plotting position `p[i] = i / (n - 1)` for `n > 1`.
Pair `x[i]` with `y[i]` at that position. Collapse each run of equal VIIRS values
into one knot whose target is the arithmetic mean of that run's MODIS values.
This averages the MODIS order statistics across the tied VIIRS ranks.
Nonnegative ordered inputs therefore give nonnegative,
nondecreasing knots, including ties in either sensor.

Zero is an explicit structural exception: if the run has `x = 0`, its knot is
`(0, 0)`, even if some paired MODIS counts are positive. If training has no zero,
prepend `(0, 0)`. No VIIRS detections must never create a detection through
calibration. Document the lost fit at zeros in residuals and validation; do not
claim this exception preserves the training distribution exactly.

For consecutive distinct knots `(x0, y0)` and `(x1, y1)`, map `x` by

```text
y = y0 + (x - x0) * (y1 - y0) / (x1 - x0)
```

At a knot return its target exactly. Above the largest observed VIIRS count,
clamp to the last target. This is a conservative, monotonic boundary policy,
not validated extrapolation: report out-of-range observations and widen their
uncertainty in #32. Never silently promise that clamping estimates extreme fires.

Empty overlap is an error, not an identity calibration. A single positive
training observation gives a line from `(0, 0)` to that observation and a
constant tail; mark it insufficient for scientific validation. All-zero VIIRS
training has only `(0, 0)`; a positive application count is unsupported and is
an error rather than a fabricated zero. Reject negative or nonfinite energy
inputs when #33 reuses this method on its own FRP calibration. Counts entering
from FCA1 are already nonnegative integers.

## Hand-computed count fixture

`tests/fixtures/count-overlap.csv` holds four cell-months in cell 0, tile (0, 0).
Months 276–279 are January–April 2023. Both sources have complete synthetic
coverage of these observations. All other cells are outside this fixture's mask.

| Rank | Plotting position | VIIRS x | MODIS y | Mapped VIIRS |
|---|---|---|---|---|
| 0 | 0 | 0 | 0 | 0 |
| 1 | 1/3 | 4 | 2 | 2 |
| 2 | 2/3 | 8 | 4 | 4 |
| 3 | 1 | 16 | 8 | 8 |

There are no ties: knots are `(0,0), (4,2), (8,4), (16,8)`.
At 6, interpolation gives `2 + (6-4)*(4-2)/(8-4) = 3`.
At 12 it gives `4 + (12-8)*(8-4)/(16-8) = 6`.
At 20 the clamped result is 8. These hand-computed application expectations
are in `tests/fixtures/count-application.csv`. They are not implementation output.
Raw VIIRS totals 28; MODIS and mapped VIIRS each total 14. Each of the four
fixture months matches exactly, with absolute tolerance `1e-12` before tile
rounding (only basic operations on these small exact numbers).

A separate tie example: sorted VIIRS `[0, 4, 4, 8]`, MODIS `[0, 2, 6, 8]`
give knots `(0,0), (4,4), (8,8)`, so VIIRS 2 maps to 2 and 6 maps to 6.
A zero-tie example: VIIRS `[0, 0, 4]`, MODIS `[0, 2, 6]` gives `(0,0), (4,6)`;
the middle MODIS value is not recovered at zero, and residuals must retain it.

## Tile encoding, uncertainty, and validation

Keep the mapped value unrounded until tile encoding. Round nonnegative count
and FRP to the nearest integer, with exact half values rounded upward, then
saturate at 65535; do not round or saturate the source aggregates first.
Sigma is in count units, rounded upward and saturated as FCT1 requires.
FRP uses independent knots in megawatts, never the count scale factors.

For #32, compute residuals against matched MODIS observations, including zero
exceptions, and validate coverage on the held-out months. Sigma needs a
MODIS measurement floor and widening outside the training range. Their exact
choices and the claimed coverage rate require explicit tests in #32; this
count fixture with zero residuals does not establish real-world uncertainty.

For #30 and #34, test monotonicity (including ties, zero, one-sample and tail
cases) and held-out monthly tile totals. Write a justified absolute/relative
tolerance next to each validation assertion. The exact synthetic fit above is
an arithmetic check, not evidence that real monthly totals agree: quantile
mapping preserves marginal ranks, not the timing of individual fire events.
Do not select a tolerance after seeing the held-out result just to pass it.

Record calibration version, tile, regional mask, source satellite pair,
training/validation months, knots, tail policy, missing coverage decisions and
validation metrics alongside pipeline output. #31 will use this provenance
when writing `index.json` and shipping per-tile parameters; no binary tile
layout changes are proposed. A three-year regional overlap cannot support a
claim of twenty-year or global calibration accuracy.
