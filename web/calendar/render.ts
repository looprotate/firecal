import type { Grid } from './grid.ts';
import { describeCell, monthTitle } from './labels.ts';
import { latestPosition } from './keyboard.ts';
import { CELL, GAP, GRID_WIDTH, LEFT, TOP, columnX } from './layout.ts';
import { provisionalTriangle, unusualDiamond } from './marks.ts';
import type { CalendarSeries } from './series.ts';
import { sketchRect } from './sketch.ts';

const RING_OFFSET = 4;
const RING_PAD = 6;
/** How far the outline of an analog year's row stands off its cells. */
const ANALOG_PAD = 3;
const MONTH_LETTERS = ['J', 'F', 'M', 'A', 'M', 'J', 'J', 'A', 'S', 'O', 'N', 'D'];
/** How far the hand wobbles the outline of a cell, in SVG units: a fortieth of its side, as on the site. */
const CELL_WOBBLE = CELL / 40;
/** The cells pop in row by row, this many milliseconds apart, and a little later along each row. */
const ROW_WAVE = 26;
const COLUMN_WAVE = 3;

/** When a month starts to pop in, in milliseconds after the calendar starts drawing. */
export function waveDelay(row: number, column: number): number {
  return row * ROW_WAVE + column * COLUMN_WAVE;
}

const cellPath = (x: number, y: number, seed: number): string => sketchRect(x, y, CELL, CELL, 6, seed, CELL_WOBBLE);

/** The SVG of the calendar as text. Colors come from CSS classes `ramp-0` to `ramp-7`, never from here. */
export function renderCalendarSvg(grid: Grid, plane: CalendarSeries['plane'], analogYear: number | null = null): string {
  const width = GRID_WIDTH + RING_PAD;
  const height = TOP + grid.rows.length * (CELL + GAP) - GAP + RING_PAD;
  const entry = latestPosition(grid);

  const flagsAt = waveDelay(grid.rows.length, 12);

  const letters = MONTH_LETTERS.map(
    (letter, column) =>
      `<text class="axis" aria-hidden="true" style="--d:${waveDelay(0, column)}ms" x="${columnX(column) + CELL / 2}" y="12" text-anchor="middle">${letter}</text>`,
  ).join('');

  const rows = grid.rows
    .map((row, at) => {
      const y = TOP + at * (CELL + GAP);
      const year = `<text class="axis" aria-hidden="true" style="--d:${waveDelay(at, 0)}ms" x="${LEFT - 8}" y="${y + CELL / 2 + 4}" text-anchor="end">${grid.years[at] ?? ''}</text>`;
      const cells = row
        .map((cell, column) => {
          const x = columnX(column);
          const year = grid.years[at] ?? 0;
          const seed = (year - 2000) * 12 + column;
          const wave = `style="--d:${waveDelay(at, column)}ms"`;
          if (cell === null) {
            const label = monthTitle(year, column + 1);
            return `<path class="cell absent" role="gridcell" aria-label="${label}: not in the series" aria-disabled="true" ${wave} d="${cellPath(x, y, seed)}"/>`;
          }
          const entering = entry !== null && entry.row === at && entry.column === column;
          const shape = `<path class="cell ramp-${cell.step}" role="gridcell" tabindex="${entering ? 0 : -1}" data-row="${at}" data-column="${column}" data-x="${x}" data-y="${y}" aria-label="${describeCell(cell, plane)}" ${wave} d="${cellPath(x, y, seed)}"/>`;
          const provisional = cell.provisional ? provisionalTriangle(x, y, seed, flagsAt) : '';
          return shape + provisional + (cell.unusual ? unusualDiamond(x, y, seed, flagsAt) : '');
        })
        .join('');
      return `<g role="row">${year}${cells}</g>`;
    })
    .join('');

  const analogRow = analogYear === null ? -1 : grid.years.indexOf(analogYear);
  const analogRing = analogRow < 0
    ? ''
    : `<path class="analog-ring" aria-hidden="true" pathLength="1" d="${sketchRect(columnX(0) - ANALOG_PAD, TOP + analogRow * (CELL + GAP) - ANALOG_PAD, GRID_WIDTH - LEFT + 2 * ANALOG_PAD, CELL + 2 * ANALOG_PAD, 9, analogYear ?? 0, 1.2)}"/>`;

  return `<svg class="calendar-grid" role="grid" aria-label="Burning activity calendar, years down and months across" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}">${letters}${analogRing}${rows}<path class="focus-ring" visibility="hidden" d="${sketchRect(-RING_OFFSET, -RING_OFFSET, CELL + 2 * RING_OFFSET, CELL + 2 * RING_OFFSET, 10, 7, 0.8)}"/></svg>`;
}
