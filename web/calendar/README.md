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
- **Focus ring** is a separate rectangle offset from the cell by a dark gap, so it shows on the lightest step too.

```
node --test web/calendar
```
