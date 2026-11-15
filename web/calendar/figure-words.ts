import { MONTH_NAMES, formatNumber } from './labels.ts';
import type { OverlapRow } from './overlap.ts';

const monthName = (index: number): string => `${MONTH_NAMES[index % 12] ?? ''} ${2000 + Math.floor(index / 12)}`;

const totals = (rows: readonly OverlapRow[]): string => {
  const sum = (pick: (row: OverlapRow) => number): string => formatNumber(rows.reduce((total, row) => total + pick(row), 0));
  return `MODIS ${sum((row) => row.modis)}, raw VIIRS ${sum((row) => row.viirs)}, harmonized ${sum((row) => row.harmonized)}`;
};

/**
 * The figure's lines of words: the months and tiles, then the totals of the months the calibration was fitted on,
 * which agree by construction, apart from the totals of the months held out of the fit, which test it.
 */
export function overlapFacts(rows: readonly OverlapRow[]): string[] {
  if (rows.length === 0) return ['No overlap months.'];
  const months = rows.map((row) => row.monthIndex);
  const first = Math.min(...months);
  const last = Math.max(...months);
  const tiles = new Set(rows.map((row) => `${row.tileRow}/${row.tileCol}`)).size;
  const span = first === last ? monthName(first) : `${monthName(first)} to ${monthName(last)}`;
  const fit = rows.filter((row) => row.training);
  const held = rows.filter((row) => !row.training);
  if (first === last && held.length === 0) {
    return [`${span}, ${tiles} tiles. ${totals(rows)}.`, 'Fit month: the agreement is by construction, not a validation.'];
  }
  const lines = [`${span}, ${tiles} tiles.`];
  if (fit.length > 0) lines.push(`Fit months: ${totals(fit)}, equal by construction.`);
  if (held.length > 0) lines.push(`Held out of the fit: ${totals(held)}.`);
  return lines;
}

/** What the figure shows, in one sentence for a screen reader. */
export function describeOverlap(rows: readonly OverlapRow[]): string {
  return overlapFacts(rows).join(' ');
}
