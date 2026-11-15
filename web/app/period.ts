import type { Series } from './series.ts';
export interface MonthRange { first: number; last: number }
export function monthCode(index: number): string { return `${2000 + Math.floor(index / 12)}-${String(index % 12 + 1).padStart(2, '0')}`; }
export function parseMonth(code: string): number | null {
  const match = /^(20\d{2})-(0[1-9]|1[0-2])$/.exec(code);
  return match ? (Number(match[1]) - 2000) * 12 + Number(match[2]) - 1 : null;
}
/** A display window intersects real coverage; disjoint windows visibly reset to the available record. */
export function resolvePeriod(requested: MonthRange | undefined, first: number, last: number): MonthRange & { adjusted: boolean } {
  if (![first,last,requested?.first??first,requested?.last??last].every(Number.isInteger)) throw new Error('The selected period is invalid.');
  if (first > last) throw new Error('No shared period is available.');
  if (requested && requested.first > requested.last) throw new Error('The start month must not be after the end month.');
  const low = Math.max(first, requested?.first ?? first), high = Math.min(last, requested?.last ?? last);
  if (low > high) return { first, last, adjusted: true };
  return { first: low, last: high, adjusted: !!requested && (low !== requested.first || high !== requested.last) };
}
/** Slices the core's existing arrays without recomputing its statistics. */
export function windowSeries(series: Series, range: MonthRange): Series {
  const from = range.first - series.first_month_index, to = range.last - series.first_month_index + 1;
  if (from < 0 || to > series.value.length || from >= to) throw new Error('The selected period is outside the available record.');
  return { ...series, first_month_index: range.first, months: series.months.slice(from, to), value: series.value.slice(from, to), sigma: series.sigma.slice(from, to), anomaly: series.anomaly.slice(from, to), unusual: series.unusual.slice(from, to) };
}
/** CSV exports retain native-core values and columns, with only out-of-window rows removed. */
export function filterCsvPeriod(bytes: Uint8Array, range: MonthRange): Uint8Array {
  const [header, ...rows] = new TextDecoder().decode(bytes).trimEnd().split('\n');
  const selected = rows.filter(row => { const [year, month] = row.split(',', 2).map(Number); const index = (year! - 2000) * 12 + month! - 1; return index >= range.first && index <= range.last; });
  return new TextEncoder().encode([header, ...selected].join('\n') + '\n');
}
