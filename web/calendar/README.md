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
- **Focus ring** is a separate rectangle offset from the cell by a gap, so it shows on every step.
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
- **Legend** under the readout: the empty cell and the seven steps from none to most, then a cell with each mark and
  what it means in words. It is drawn from the same classes as the cells.

```
node --test web/calendar
```
