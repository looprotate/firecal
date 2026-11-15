# Calendar

Months across, years down, one cell per month of a Series. It draws; every number comes from the core.

- `grid.ts` places the months of a Series in year rows. `ramp.ts` gives each value a ramp step. `render.ts` writes
  the SVG as text, so it is tested without a browser. `mount.ts` puts it in the page and handles focus and hover.
- **Ramp scale.** A value maps to `ceil(7 * sqrt(value / max))`, so any activity is at least step 1 and a few huge
  months do not flatten the rest. `max` is the largest value in the series on screen.
- **Colors** are never written here: cells carry a class `ramp-0` to `ramp-7` and `calendar.css` reads
  `web/tokens/tokens.css`.
- **Keyboard.** Tab enters at the newest month; arrows move one cell, Home and End the row, PageUp and PageDown a
  year. The month, value and estimated uncertainty are in each cell's label and in the readout under the grid.
- **Drawn by hand.** Every shape (cells, marks, rings, bars, the figure's axes) is drawn with the hand of the site
  and the film (`sketch.ts` over `site/js/draw.js`): a fixed wobble seeded by what the shape is, so a month keeps
  its wobble across redraws. The data keeps the measured ramp, never a pastel.
- **Drawing in and out.** A new Series undraws the old calendar, then pops in row by row on a short spring, its
  flags after it, its rings tracing themselves (`waveDelay` in `render.ts`); the container carries `entering` or
  `leaving` meanwhile, which is the page's cue to boil it. The same Series is redrawn in place, still.
- **Focus ring** is a separate hand-drawn rectangle offset from the cell by a gap, so it shows on every step.
- **Provisional months** (from `provisional_from` of the Series on) carry a triangle in the bottom left corner,
  `--provisional-size` along each side, inside the cell's rounded corner, and say "provisional" in their label. The
  token test proves it shows on every step and on an empty cell; `marks.test.mjs` proves where it is drawn.
- **Unusual months** carry a diamond in the top right corner, ink with a paper edge, and say "unusual" in their
  label. Only the core decides: the calendar draws `unusual` from the Series and never compares values itself.
- **The season against the median**, under the legend: one column under each month of the last year, a bar from the
  median line to the core's anomaly, its sensor band at two sigma in sky (estimated), and the diamond when flagged.
  The sentence under it reads the newest month in words, so nothing depends on the drawing.
- **Controls**, above the grid, only when the page offers them (`CalendarActions`): Count and Energy as pressed
  buttons, which ask the page for the other plane, and `series.csv` and `cells.csv`, which save the core's text
  as files. A polygon has no `cells.csv`, and the toolbar says so instead of offering a button that fails.
- **Analog years**, at the bottom: the past years the core ranks most like the season in progress, nearest first.
  Pressing one outlines its row in the calendar; when the season is not a whole year, its words list the months that
  came after it in that year. History, not a forecast, and the page says so.
- **Before and after harmonization** (`mountOverlapFigure`): Cube's overlap report as two panels on the same
  square-root axes, one point per tile, MODIS across and VIIRS up: raw VIIRS above the diagonal, harmonized VIIRS on
  it. A month used for the fit says its agreement is by construction, not a validation. "Save as SVG" writes the
  figure with the token values inside, so the pitch can use the same drawing.
- **Legend** under the readout: the empty cell and the seven steps from none to most, then a cell with each mark and
  what it means in words. It is drawn from the same classes as the cells.

## Accessibility, checked by hand (2026-11-14)

- **Keyboard:** Tab reaches the map, its two drawing tools and zoom, the four coordinates, "Use this area", "Pin for
  offline use", the plane switch and the files when the core answers, one cell of the calendar (the newest), the
  analog years and "Save as SVG", in that order. Arrows, Home, End, PageUp and PageDown move inside the calendar; the
  focus ring is ink, offset from the cell, and shows on every step.
- **Screen readers:** every cell is a grid cell whose label says the month, the value, the estimated uncertainty, and
  "provisional" or "unusual" when it is; the strip is a list whose items are sentences; the legend is a list of words.
  The readout under the grid is not a live region: it repeats what the focused cell already says.
- **Contrast:** every visible text measured against its real background reaches 4.5:1 (smallest, error text on a
  field, 4.51:1); the ramp, the marks and the meaning colors are proven at 3:1 by `web/tokens/ramp.test.mjs`.
- **Without color:** in grayscale the ramp keeps its order and both marks stand out on every step; unusual and
  provisional are shapes in fixed corners and words in the labels; before and after in the figure are rings and dots.
- **Motion:** the calendar drawing in and out, and the shake of a refused entry, turn off under
  `prefers-reduced-motion`; then a new Series simply replaces the old.
- **Not tested:** VoiceOver, NVDA and Windows High Contrast; drawing on the map needs a pointer, and typed coordinates
  are the keyboard path to the same area.

```
node --test web/calendar
```
