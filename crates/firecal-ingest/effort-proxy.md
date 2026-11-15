# Observation-effort proxy: research and demo decision (#28)

## Decision

**Drop the observation-effort feature from the current demo.** This completes
the issue's explicit alternative acceptance path: a clear note explaining why
it was dropped. No effort layer, normalization, exposure estimate or public
runtime API is implemented or advertised by this change.

The current inputs are fire detections, not an observation-denominator product.
They do not establish how often each cell was observed without a detection,
which looks were obscured, or which expected passes were unavailable. A number
derived from detected fires would confound the outcome with the observation
effort it is supposed to explain. In particular, `day_count` counts daytime
detections; it is not the number of daytime overpasses or observation days.

The shared FCT1 format also fixes exactly three planes: count, sigma and energy.
`write_tile` takes three planes and `TileView::parse` rejects any other plane
count. An effort plane therefore requires an intentional, lead-owned format
change and coordinated consumers; ingest must not append it or reuse sigma.

## Research basis

NASA FIRMS FAQ, checked 2026-11-14:
<https://www.earthdata.nasa.gov/data/tools/firms/faq>

- Polar-orbiting instruments generally observe a location twice per day, with
  more opportunities toward the poles. This is a broad description, not a
  latitude-specific coverage table or a guarantee for an individual cell/day.
- A hotspot is a pixel flagged as a thermal anomaly, not a record of every
  attempted observation. The detection algorithms also classify clouds,
  non-fire and missing/unknown pixels, which a detections-only CSV cannot supply.
- The FAQ distinguishes pixel-level quality information available in standard
  Level 2G products, including clouds and overlapping-orbit handling, from the
  simpler active-fire representations. Product choice matters for quantitative
  comparisons and must be checked before deriving a denominator.
- For location/date-specific expected observation counts and geometry, NASA
  points to its Ocean Color Overpass Predictor. Even predicted opportunities
  would not establish valid cloud-free observations or actual instrument uptime.

The nominal global revisit description cannot justify inventing a latitude
curve. An archive's first or last fire record is also not an operational uptime
log. No arbitrary polar multiplier or hardcoded mission uptime is introduced.

## Candidate method for a future implementation

An explicitly labeled *potential observation opportunity* index could be:

```text
potential_effort(cell, month) =
    sum over satellites and days in the month of
        availability(satellite, day)
        * predicted_passes(satellite, latitude_band(cell), day)
```

Availability must come from supplied operational/coverage evidence, not hotspot
counts. Predicted passes need a sourced table or orbit/swath calculation with
documented latitude bands and applicability dates. Calendar days, leap years,
partial operational months and gaps must be handled explicitly. Where either
input is unavailable, the result is unknown, not zero. Zero requires positive
evidence of no opportunities.

The unit would be expected pass-opportunities per cell-month. It would still
omit clouds, valid viewing fraction, scan geometry effects and the different
sensor detection thresholds. It must not be described as measured observation
coverage or used to divide counts without validation against suitable products.

For arithmetic only, a hand-worked hypothetical example has satellite A active
for 30 days and B for 10 days. At supplied rates of two opportunities/day for
both, the index is `30*2 + 10*2 = 80`. In a different band with independently
supplied rates of three/day, it is `30*3 + 10*3 = 120`. These are invented inputs
to explain the equation, not measured rates, a latitude model or a test proving
scientific validity. No executable estimator is shipped from this example.

## Conditions for revisiting

1. Obtain versioned coverage/uptime inputs and location-specific pass estimates,
   including an explicit policy for cloudy, unknown and missing observations.
2. Validate the index against an independent observation/quality product in
   several latitude bands, seasons and partial-availability months. Record the
   error and how far it can be interpreted as effort.
3. Have Biggie own a separate contract change: units, scale, missing-value
   representation, format version and all readers/writers must agree before any
   extra plane is emitted, and unknown exposure must never become a zero or a
   denominator for fire counts. Acceptance tests then cover full and partial
   operational months, leap February, latitude-band boundaries, absent inputs
   against measured zeros, deterministic order and native/wasm parity.
4. Then implement deterministic library code with hand-computed tests for
   calendar boundaries, latitude bands, missing inputs and active intervals,
   plus round-trip and native/wasm parity checks for the changed contract.

Until those prerequisites exist, the app and pitch must not claim that counts
are corrected for how often satellites looked. Count, energy and uncertainty
retain their existing definitions.
