# The idea

**A burning-activity calendar that tells you how sure it is, works with no signal, and remembers which past
year this one looks like.**

The challenge: MODIS (2000–) and VIIRS (2012–) hotspot records cannot be compared directly, so nobody has a
consistent picture of when and where burning happens. We harmonize them and build a calendar over any area.

## What every team will build

Count hotspots per month, scale VIIRS to look like MODIS, draw a heatmap on a map. It will look right. It
will also hide the two things an emergency manager needs to know: **how much to trust a number**, and **what
it means for the season that is happening now**.

## What makes ours different

Four layers, each one needing the one before. Together they are one argument, not four features.

### 1. Honest numbers: uncertainty on every cell

Harmonization is an estimate, so every harmonized count carries a **sigma**. The calibration is fitted per tile
on the years both sensors saw; its residuals over those years are the error we ship, and the web app shows it.

What it buys: an anomaly is flagged only when it is **outside the uncertainty band**. "This March is unusual"
becomes "this March is unusual *and* it is not an artifact of switching sensors". Nobody else can say that.

### 2. Energy and effort, not just points

- **Energy:** FIRMS gives each hotspot its fire radiative power (FRP). The calendar can show hotspot **count** or
  total **FRP**: many small agricultural burns look very different from a few huge wildfires.
- **Effort (stretch):** a count depends on how many satellite passes looked. A proxy for observation effort per
  cell-month (satellites in operation, and how often each overflies that latitude) lets us separate "more fire"
  from "more looking". It is a proxy, and the app says so.

### 3. Analog years

Given the season in progress, the core finds the past years whose monthly profile most resembles it and shows
what happened next in each. It answers the challenge's own question, "how does this compare with previous
ones", with a picture instead of a median. It compares rank-in-month, not raw counts, so a quiet region and a
busy one are judged the same way.

### 4. Offline, in the field

The core is Rust compiled to WebAssembly and the data is static files, so the app is a **PWA**: pin a region
while you have signal, then query any polygon in it with none. The pitch line is literal: the function that
harmonized 20 years of satellites runs on the phone in the truck.

How much fits offline is **measured on the first full run**, not assumed. If the world does not fit, the user
pins regions.

## What we do not claim

- A hotspot is a thermal anomaly seen from orbit, not a measured fire or a burned area.
- Harmonized counts are **MODIS-equivalent**. The calibration is validated only on the overlap years.
- This is not a forecast. Analog years are history, not prediction.
- The newest months come from near-real-time data and are labeled provisional: they are not yet science quality.
- The effort proxy is a proxy.
- Sigma adds cell errors as if independent, which is optimistic where neighbors share a bias. The app says
  "estimated" next to it.

## Priorities, and what we cut first

1. **Must:** harmonized calendar, current month against the historical median, CSV, a map to pick the area.
2. **Pillar:** uncertainty (sigma in the tiles, band on screen, anomaly flag).
3. **Cheap, so yes:** offline PWA; FRP as a second calendar mode (ingest sums it anyway).
4. **Worth it:** analog years.
5. **First to go if time runs out:** the effort proxy.

If a layer is half-done on Sunday morning, it is removed from the demo rather than shown broken.

## Demo (three minutes, to refine with the pitch owner)

1. A place everyone knows, a recent bad season. The calendar, in one image.
2. The anomaly of the current month, flagged because it clears the uncertainty band.
3. Switch to energy: the story changes.
4. Analog years: "this season looks like these three".
5. Airplane mode on. Draw a new polygon. It still answers, in milliseconds.
6. The sentence: the sensor stops mattering, and every number says how sure it is.

## Measured, not guessed

Pitch numbers come from running the thing: how long the pipeline took on the archive, how long a query takes in
the browser, how big the offline data is, how well harmonized VIIRS agrees with MODIS over the overlap. None is
written here until it has been measured.
