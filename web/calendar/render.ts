import type { Grid } from './grid.ts';
import { describeCell, monthTitle } from './labels.ts';
import { latestPosition } from './keyboard.ts';
import type { CalendarSeries } from './series.ts';

/** The side of a cell, in SVG units. */
export const CELL = 28;
const GAP = 6;
const LEFT = 40;
const TOP = 20;
const RING_OFFSET = 4;
const RING_PAD = 6;
const MONTH_LETTERS = ['J', 'F', 'M', 'A', 'M', 'J', 'J', 'A', 'S', 'O', 'N', 'D'];

/** Where the focus ring sits relative to the cell it surrounds, in SVG units. */
export const FOCUS_RING_OFFSET = RING_OFFSET;

/** The provisional triangle's sides, as `--provisional-size` in `web/tokens/tokens.css`. */
export const PROVISIONAL_SIZE = 10;
/** How far the triangle sits in from the cell's edges, so it stays inside the rounded corner. */
const MARK_INSET = 2;

/** The provisional triangle in the bottom left corner of the cell at `x`, `y`. */
export function provisionalTriangle(x: number, y: number): string {
  const left = x + MARK_INSET;
  const bottom = y + CELL - MARK_INSET;
  return `<path class="provisional" aria-hidden="true" d="M${left} ${bottom - PROVISIONAL_SIZE}V${bottom}H${left + PROVISIONAL_SIZE}Z"/>`;
}

/** The SVG of the calendar as text. Colors come from CSS classes `ramp-0` to `ramp-7`, never from here. */
export function renderCalendarSvg(grid: Grid, plane: CalendarSeries['plane']): string {
  const width = LEFT + 12 * (CELL + GAP) - GAP + RING_PAD;
  const height = TOP + grid.rows.length * (CELL + GAP) - GAP + RING_PAD;
  const entry = latestPosition(grid);

  const letters = MONTH_LETTERS.map(
    (letter, column) =>
      `<text class="axis" aria-hidden="true" x="${LEFT + column * (CELL + GAP) + CELL / 2}" y="12" text-anchor="middle">${letter}</text>`,
  ).join('');

  const rows = grid.rows
    .map((row, at) => {
      const y = TOP + at * (CELL + GAP);
      const year = `<text class="axis" aria-hidden="true" x="${LEFT - 8}" y="${y + CELL / 2 + 4}" text-anchor="end">${grid.years[at] ?? ''}</text>`;
      const cells = row
        .map((cell, column) => {
          const x = LEFT + column * (CELL + GAP);
          if (cell === null) {
            const label = monthTitle(grid.years[at] ?? 0, column + 1);
            return `<rect class="cell absent" role="gridcell" aria-label="${label}: not in the series" aria-disabled="true" x="${x}" y="${y}" width="${CELL}" height="${CELL}" rx="6"/>`;
          }
          const entering = entry !== null && entry.row === at && entry.column === column;
          const rect = `<rect class="cell ramp-${cell.step}" role="gridcell" tabindex="${entering ? 0 : -1}" data-row="${at}" data-column="${column}" aria-label="${describeCell(cell, plane)}" x="${x}" y="${y}" width="${CELL}" height="${CELL}" rx="6"/>`;
          return cell.provisional ? rect + provisionalTriangle(x, y) : rect;
        })
        .join('');
      return `<g role="row">${year}${cells}</g>`;
    })
    .join('');

  return `<svg class="calendar-grid" role="grid" aria-label="Burning activity calendar, years down and months across" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}">${letters}${rows}<rect class="focus-ring" visibility="hidden" width="${CELL + 2 * RING_OFFSET}" height="${CELL + 2 * RING_OFFSET}" rx="10"/></svg>`;
}
