# Estimated count uncertainty (#32)

`fit_sigma` binds an error model to the exact count calibration used for its
residuals. `fit_sigma_aggregates` assembles that training population from paired,
complete standard-product tile-months. `apply_with_sigma` writes the existing
FCT1 sigma plane without changing count bytes or the shared file format. The
CLI now uses this path by default, with an active-cell floor of one count.
The independent FRP plane is described in `energy.md`; count sigma never
serves as FRP uncertainty. All output remains development-only.

## Assumptions and arithmetic

For each matched training cell-month, evaluate the actual VIIRS mapping against
its paired MODIS reference count, in sorted (month, cell) order. Compute

```text
r = mapped_viirs - modis_reference
residual_rms = sqrt(sum(r*r) / sample_count)
```

The mean residual is not removed, so systematic errors, including structural
zero errors, remain in the scale. These are in-sample residuals; they can
underestimate future errors. No training sample may be a held-out month.

The MODIS-only measurement proxy is `max(sqrt(raw_count), active_floor)` for
positive counts and zero for a known zero reference count. It does not model
undetected physical fire, instrument-specific detection error or a calibrated
probability distribution. The one-count active minimum is explicit because
the shipped uncertainty is quantized to whole counts; it is an assumption to
validate, not a physical measurement claimed from the FIRMS records.

Mapped VIIRS uses

```text
active_floor = configured_floor if raw_viirs > 0, otherwise 0
base = max(residual_rms, sqrt(mapped_count), active_floor)
span = last_training_month - first_training_month + 1
time_factor = 1 + distance_to_training_interval / span
range_factor = 1 + max(raw_viirs - max_training_viirs, 0) / max(max_training_viirs, 1)
sigma = base * time_factor * range_factor
```

These monotonic widening rules are transparent heuristics fixed before the
coverage test. They do not establish Gaussian errors or nominal confidence.
A zero VIIRS observation retains nonzero residual RMS if the overlap showed
missed MODIS reference counts. An entirely zero training population with zero
observations stays zero; unsupported positive VIIRS still returns an error.
MODIS output does not receive a calibration-time penalty because it did not
use the mapping.

Sigma is rounded upward with a bounded integer search, then saturated at
65535. Any count encoded at the maximum stored value also carries sigma 65535,
marking the representation boundary; that saturated field cannot express an
arbitrarily large error or support an interval-coverage claim. A tile is omitted
only when all available count/sigma values are zero. A zero mapped count with
nonzero sigma is retained, preserving uncertainty from actual VIIRS activity.

The JSON index records model version, residual RMS, active-cell floor, training
bounds and widening policies. It declares `planes: ["count", "sigma"]` and
`real_holdout_coverage: "not_validated"`. When `apply_with_energy` is used, the
third plane receives independently calibrated FRP; it still has no sigma of
its own. The count-only library API remains usable for #31 diagnostics.

## Hand-computed and held-out checks

Four training observations span January/March 2023, two cells per month.
Every VIIRS count is 4; paired MODIS counts are 0 and 4 in each month. Quantile
mapping gives 2 at VIIRS 4. Residuals are `+2,-2,+2,-2`, so RMS is exactly 2.
The training calendar span is three months; three months beyond either end
therefore doubles the scale. VIIRS 8 also doubles the range factor independently.
A one-sample zero-mapping example retains a systematic MODIS error of 2.

February/April are wholly held out. Each supplies ten fixed reference counts,
with VIIRS 4 in every case. Nineteen references are in 0–4; one April reference
is deliberately 12. The estimated two-sigma band covers exactly 19 of 20
references (95%) and leaves the extreme one outside. This is a hand-declared
synthetic regression fixture, not a measured real-world 95% coverage guarantee.
The test fails if the scale is reduced or inflated enough to alter that count.

A second executable fixture proves count/sigma readback and area aggregation:
two mapped cells `[1.5,3]` one month beyond a one-month training window encode
as counts `[2,3]` and sigmas `[3,4]`. The query returns count 5 and sigma 5 by
quadrature. Further tests cover zero activity, positive VIIRS mapped to zero,
wrong training populations, source/tile binding, invalid floors, saturation,
and 1,000 seeded models with finite, monotonic widening (seed `0xC0BE0032`).

## Real-file pipeline evidence

Using the previously audited South America release, reran all 24 complete
interior tiles on July/August standard Terra, fitting on July standard
Terra/S-NPP only. The final run is retained locally under ignored
`data/south-america-sigma-validation-v2/`; previous diagnostic runs are preserved.

Eighteen nonempty binaries and six omitted empty tiles have count-plane bytes
identical to #31. Every sigma cell agrees with the independently computed
integer ceiling of the MODIS counting proxy. The shared native query returns
counts `[7000,13359]`; sigma squared sums are `[9132,17358]`, giving estimated
regional scales about `[95.5615,131.7498]` under the query's independent-cell
quadrature assumption. Those regional scales are mechanical model outputs,
not validated errors on physical fire activity.

Only July is a complete standard overlap month in this package. August VIIRS
is provisional, so no independent real standard-month coverage rate is claimed.
More complete standard overlap years are required to validate the error model,
its active-cell floor, widening rules and spatial dependence. The application
must continue to label sigma estimated, and these development tiles must not
be presented as a validated science product.
