import type { Grid } from './grid.ts';
import { latestPosition } from './keyboard.ts';
import { MONTH_NAMES, formatNumber } from './labels.ts';
import type { CalendarSeries } from './series.ts';

/** A past year and its rank distance from the season in progress, as the core returns it. */
export interface Analog {
  year: number;
  distance: number;
}

/**
 * The words for an analog year: its distance and, when the season in progress is not a whole year, the values of
 * the months that came after it in that year: what happened next. The values are the series', unchanged.
 */
export function describeAnalog(analog: Analog, grid: Grid, plane: CalendarSeries['plane']): string {
  const head = `${analog.year}, distance ${formatNumber(analog.distance)}.`;
  const newest = latestPosition(grid);
  const row = grid.rows[grid.years.indexOf(analog.year)];
  if (newest === null || row === undefined || newest.column === 11) return head;
  const next = row.slice(newest.column + 1).flatMap((cell) =>
    cell === null ? [] : [`${MONTH_NAMES[cell.month - 1] ?? ''} ${formatNumber(cell.value)}`]);
  if (next.length === 0) return head;
  return `${head} Then ${next.join(', ')} ${plane === 'energy' ? 'MW' : 'hotspots'}.`;
}

/**
 * The past years most like the season in progress, nearest first, as buttons: pressing one outlines its row in the
 * calendar. Empty when the series has no analogs.
 */
export function renderAnalogs(
  analogs: readonly Analog[],
  grid: Grid,
  plane: CalendarSeries['plane'],
  chosen: number | null,
): string {
  if (analogs.length === 0) return '';
  const season = grid.years.at(-1) ?? '';
  const items = analogs.map((analog) =>
    `<li><button type="button" data-year="${analog.year}" aria-pressed="${analog.year === chosen}">${analog.year}</button>`
    + `<span>${describeAnalog(analog, grid, plane).replace(/^\d+, /, '')}</span></li>`).join('');
  return `<h3 class="calendar-season-title">Years most like ${season}</h3><ol class="calendar-analogs">${items}</ol>`
    + '<p class="calendar-note">Ranked by how each month compares within its own calendar month. History, not a forecast.</p>';
}
