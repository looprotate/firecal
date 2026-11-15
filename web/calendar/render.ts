import type { Grid } from './grid.ts';
import { describeCell, monthTitle } from './labels.ts';
import { latestPosition } from './keyboard.ts';
import { CELL, GAP, GRID_WIDTH, LEFT, TOP, columnX } from './layout.ts';
import { provisionalTriangle, unusualDiamond } from './marks.ts';
import type { CalendarSeries } from './series.ts';

const RING_OFFSET = 4;
const RING_PAD = 6;
/** How far the outline of an analog year's row stands off its cells. */
const ANALOG_PAD = 3;
const MONTH_LETTERS = ['J', 'F', 'M', 'A', 'M', 'J', 'J', 'A', 'S', 'O', 'N', 'D'];

/** Where the focus ring sits relative to the cell it surrounds, in SVG units. */
export const FOCUS_RING_OFFSET = RING_OFFSET;

/** The SVG of the calendar as text. Colors come from CSS classes `ramp-0` to `ramp-7`, never from here. */
export function renderCalendarSvg(grid: Grid, plane: CalendarSeries['plane'], analogYear: number | null = null): string {
  const width = GRID_WIDTH + RING_PAD;
  const height = TOP + grid.rows.length * (CELL + GAP) - GAP + RING_PAD;
  const entry = latestPosition(grid);

  const letters = MONTH_LETTERS.map(
    (letter, column) =>
      `<text class="axis" aria-hidden="true" x="${columnX(column) + CELL / 2}" y="12" text-anchor="middle">${letter}</text>`,
  ).join('');

  const rows = grid.rows
    .map((row, at) => {
      const y = TOP + at * (CELL + GAP);
      const year = `<text class="axis" aria-hidden="true" x="${LEFT - 8}" y="${y + CELL / 2 + 4}" text-anchor="end">${grid.years[at] ?? ''}</text>`;
      const cells = row
        .map((cell, column) => {
          const x = columnX(column);
          if (cell === null) {
            const label = monthTitle(grid.years[at] ?? 0, column + 1);
            return `<rect class="cell absent" role="gridcell" aria-label="${label}: not in the series" aria-disabled="true" x="${x}" y="${y}" width="${CELL}" height="${CELL}" rx="6"/>`;
          }
          const entering = entry !== null && entry.row === at && entry.column === column;
          const rect = `<rect class="cell ramp-${cell.step}" role="gridcell" tabindex="${entering ? 0 : -1}" data-row="${at}" data-column="${column}" aria-label="${describeCell(cell, plane)}" x="${x}" y="${y}" width="${CELL}" height="${CELL}" rx="6"/>`;
          const provisional = cell.provisional ? provisionalTriangle(x, y) : '';
          return rect + provisional + (cell.unusual ? unusualDiamond(x, y) : '');
        })
        .join('');
      return `<g role="row">${year}${cells}</g>`;
    })
    .join('');

  const analogRow = analogYear === null ? -1 : grid.years.indexOf(analogYear);
  const analogRing = analogRow < 0
    ? ''
    : `<rect class="analog-ring" aria-hidden="true" x="${columnX(0) - ANALOG_PAD}" y="${TOP + analogRow * (CELL + GAP) - ANALOG_PAD}" width="${GRID_WIDTH - LEFT + 2 * ANALOG_PAD}" height="${CELL + 2 * ANALOG_PAD}" rx="9"/>`;

  return `<svg class="calendar-grid" role="grid" aria-label="Burning activity calendar, years down and months across" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}">${letters}${analogRing}${rows}<rect class="focus-ring" visibility="hidden" width="${CELL + 2 * RING_OFFSET}" height="${CELL + 2 * RING_OFFSET}" rx="10"/></svg>`;
}
