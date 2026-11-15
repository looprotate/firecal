import type { Cell, Grid } from './grid.ts';
import { CELL, GRID_WIDTH, columnX } from './layout.ts';
import { formatNumber, monthTitle } from './labels.ts';
import { diamondAt } from './marks.ts';
import { sketch, sketchLine, sketchRect } from './sketch.ts';
import type { CalendarSeries } from './series.ts';

const HEIGHT = 96;
const MIDDLE = HEIGHT / 2;
/** Room above and below the tallest bar for its diamond. */
const HEADROOM = 12;
const BAR = 12;
/** The bars grow out of the median line one after another, this many milliseconds apart. */
const WAVE = 45;

/** How far `cell` is from its median, as the core's anomaly, and its sensor band at two sigma in the same units. */
function reach(cell: Cell): { anomaly: number; band: number } {
  const band = Number.isFinite(cell.sigma) ? (2 * cell.sigma) / Math.max(cell.median, 1) : 0;
  return { anomaly: cell.anomaly, band };
}

/**
 * The words for a month of the season against its median: what the strip says to a screen reader and what the
 * page shows for the current month. Every number in it comes from the core.
 */
export function describeSeasonMonth(cell: Cell, plane: CalendarSeries['plane']): string {
  const unit = plane === 'energy' ? 'MW' : 'hotspots';
  const head = `${monthTitle(cell.year, cell.month)}: ${formatNumber(cell.value)} ${unit}`;
  if (!Number.isFinite(cell.median)) return `${head}. No past years to compare with.`;
  // The core divides by the median or by one, whichever is larger: under one the percent is not of the median.
  const change = cell.median >= 1
    ? `, ${cell.anomaly >= 0 ? '+' : '−'}${formatNumber(Math.abs(cell.anomaly) * 100)}%`
    : '';
  const against = `${head} against a median of ${formatNumber(cell.median)}${change}`;
  if (!Number.isFinite(cell.sigma)) return `${against}. No uncertainty to judge it by.`;
  const verdict = cell.unusual ? 'Unusual.' : 'Within the usual range.';
  return `${against}, ±${formatNumber(2 * cell.sigma)} at two sigma (estimated). ${verdict}`;
}

/**
 * The season in progress (the last year row) against the median of past years, one column under each month: a
 * bar from the median line to the month's anomaly, its sensor band at two sigma, and the diamond when the core
 * flags it. Bars share one scale, so the tallest bar and its band fit.
 */
export function renderStripSvg(grid: Grid, plane: CalendarSeries['plane']): string {
  const year = grid.years.at(-1);
  const cells = (grid.rows.at(-1) ?? []).flatMap((cell, column) =>
    cell !== null && Number.isFinite(cell.median) ? [{ cell, column }] : [],
  );
  if (year === undefined || cells.length === 0) return '';
  const largest = Math.max(...cells.map(({ cell }) => Math.abs(reach(cell).anomaly) + reach(cell).band), 0.01);
  const scale = (MIDDLE - HEADROOM) / largest;
  const columns = cells.map(({ cell, column }) => {
    const { anomaly, band } = reach(cell);
    const center = columnX(column) + CELL / 2;
    const end = MIDDLE - anomaly * scale;
    const top = Math.min(MIDDLE, end);
    const height = Math.max(Math.abs(end - MIDDLE), 1);
    const reachOut = anomaly >= 0 ? end - band * scale - 7 : end + band * scale + 7;
    const seed = 400 + cell.monthIndex;
    const high = end - band * scale;
    const low = end + band * scale;
    const at = ` style="--d:${column * WAVE}ms"`;
    const wing = (y: number, part: number): string => sketchLine([center - 4, y], [center + 4, y], seed + part, 0.3);
    return `<g role="listitem" aria-label="${describeSeasonMonth(cell, plane)}">`
      + `<path class="anomaly-bar activity"${at}${anomaly < 0 ? ' data-below=""' : ''} d="${sketchRect(center - BAR / 2, top, BAR, height, Math.min(2, height / 2), seed, 0.5)}"/>`
      + `<path class="sigma-band uncertainty"${at} pathLength="1" d="${sketch([[center, high], [center, low]], false, seed + 1, 0.4)}${wing(high, 2)}${wing(low, 3)}"/>`
      + (cell.unusual ? diamondAt(center, reachOut, seed, 12 * WAVE + 200) : '')
      + '</g>';
  }).join('');
  return `<svg class="anomaly-strip" role="list" aria-label="${year} against the median of past years" viewBox="0 0 ${GRID_WIDTH} ${HEIGHT}" width="${GRID_WIDTH}" height="${HEIGHT}">`
    + `<text class="axis" x="0" y="${MIDDLE + 4}">Median</text>`
    + `<path class="median-line" pathLength="1" d="${sketchLine([columnX(0) - 4, MIDDLE], [GRID_WIDTH, MIDDLE], year, 0.8)}"/>${columns}</svg>`;
}
