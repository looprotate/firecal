import type { Cell } from './grid.ts';
import type { CalendarSeries } from './series.ts';

/** Month names, January first. */
export const MONTH_NAMES = [
  'January',
  'February',
  'March',
  'April',
  'May',
  'June',
  'July',
  'August',
  'September',
  'October',
  'November',
  'December',
] as const;

function formatNumber(value: number): string {
  return Math.round(value).toLocaleString('en-US');
}

/** Name of the month of a cell, such as `March 2024`. */
export function monthTitle(year: number, month: number): string {
  return `${MONTH_NAMES[month - 1] ?? ''} ${year}`;
}

/**
 * The words for a cell: what a screen reader says and what the readout shows on focus and hover. The
 * uncertainty is said to be estimated, and left out when the plane has none; a provisional month says so.
 */
export function describeCell(cell: Cell, plane: CalendarSeries['plane']): string {
  const unit = plane === 'energy' ? 'MW of fire radiative power' : 'hotspots';
  const sigma = Number.isFinite(cell.sigma) ? `, uncertainty ±${formatNumber(cell.sigma)} (estimated)` : '';
  const provisional = cell.provisional ? ', provisional' : '';
  return `${monthTitle(cell.year, cell.month)}: ${formatNumber(cell.value)} ${unit}${sigma}${provisional}`;
}
