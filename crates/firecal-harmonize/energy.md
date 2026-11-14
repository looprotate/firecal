# Independent FRP calibration (#33)

The existing FCT1 `energy` plane now stores independently harmonized monthly
summed fire radiative power, in megawatts. It is a sum of observed FRP, not a
time integral in joules, a fire count or a burned-area estimate. The count sigma
plane does not describe uncertainty in FRP; energy queries must request no sigma.

## Method and input precision

`fit_energy` takes the matched standard-overlap cell-month FRP fields from FCA1.
Their stored type is f32. Every value is promoted to f64 before sorting,
accumulation, averaging or interpolation. Finite nonnegative input is required;
signed zero is normalized to positive zero before exact tie comparisons.
No integer conversion or tile saturation occurs during fitting.

An `EnergyCalibration` has its own ordered knots, sample count and tile. It
cannot be substituted for the count `Calibration`, and no count scale factor
is used. The quantile policy matches #29: independently sorted marginal values,
mean targets for tied VIIRS ranks, structural zero, linear interpolation between
distinct knots and a clamped upper tail. Each tied mean stays within that
rank group's observed MODIS minimum/maximum to prevent roundoff from breaking
monotonicity. Empty/mismatched populations, invalid power and unsupported
positive input after all-zero VIIRS training are errors.

`fit_energy_aggregates` selects the same covered tile and paired training months
as the count/sigma pipeline, but reads only FRP for this fit. `apply_with_energy`
keeps the existing count/sigma result and adds the separately fitted power plane.
An existing MODIS file remains authoritative for the whole tile-month, including
zero power. Mapped VIIRS is used only if that MODIS file is absent.

At encoding, round half values upward and saturate at 65535 MW per cell-month,
using the same deterministic bounded integer search as the count plane. The
index records how many FRP cell-months actually exceeded that storage maximum;
clipping is distinct from ordinary nearest-integer rounding.

The resulting tile binds its energy values to the actual FRP calibration used.
Its JSON index lists all three available planes and a separate
`energy_calibration` object with version `frp-quantile-v1`, megawatt units, its
own knots/sample count, clamping policy, clipping count and unvalidated real
calibration status. `sigma_for: "count"` prevents confusing the units of sigma
with power. `development_only` remains true pending scientific validation.

## Hand-computed fixture

The synthetic complete tile has two nonzero cells in January 2023:

| Signal | MODIS | VIIRS | Fitted mapping |
|---|---|---|---|
| Count | 2, 4 | 4, 8 | one half |
| Summed FRP (MW) | 20, 40 | 5, 10 | four times |

Other training cells are known zeros. Count and power therefore require
visibly different calibrations. The FRP knots are `(0,0), (5,20), (10,40)`.
At VIIRS 1.25 MW, interpolation gives 5 MW; 2.5 gives 10; 20 clamps to 40.
February has an explicit empty MODIS file despite VIIRS power 99, so its output
is zero. March is VIIRS-only with 1.25 and 2.5 MW. The expected regional series
is counts `[6,0,5]`, summed FRP `[60,0,15]`.

Unit and executable tests read actual FCT1 files and query `Plane::Energy`
through `firecal-query`. Reversing input order preserves all output bytes and
metadata. The actual previous #32 executable and the new executable were also
run on identical synthetic FCA1 inputs: the count/sigma prefix is byte-identical.
The new energy plane has exactly the hand-written totals above. A deliberate
wrong interpolation factor made the hand test fail (`0.625 != 5`), then the
restored implementation passed.

Further tests cover fractional values, signed zero, ties, invalid and nonfinite
input, subnormal/maximum FCA1 power, zero-only training, clipping, wrong tiles,
known-zero source precedence and valid format-level energy-only output. A
seeded property test exercises 1,000 monotonic/reproducible fits with fractional
power (seed `0xC0BE0033`).

## Real-file pipeline check

Using the already audited South America release, rebuilt the 24 complete
interior tiles with July/August standard Terra and July standard S-NPP training.
Eighteen binaries are nonempty and six tiles remain omitted. All count/sigma
bytes match the retained #32 outputs. Every encoded FRP cell agrees with an
independent source-record iteration applying the required rounding/saturation.
The shared native energy query returns July 235,163 and August 459,555 MW of
summed FRP, with zero clipped cells in either month. Unrounded source sums are
about 235,073.100 and 459,408.000 MW; the difference is per-cell integer rounding,
not a fitted accuracy metric. Local generated files are under ignored
`data/south-america-energy-validation/` and source data remain untouched.

This is file/pipeline validation. Both real output months use observed MODIS
power. The package supplies only one complete standard overlap month, so it
does not establish out-of-sample FRP calibration accuracy. The VIIRS fallback
mapping is covered by the independent synthetic fixture; a longer standard
overlap is still needed for real calibration and count-sigma validation.
