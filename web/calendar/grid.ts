import { rampStep, type RampStep } from './ramp.ts';
import type { CalendarSeries } from './series.ts';

const FIRST_YEAR = 2000;

/** One month of the series, placed in the calendar. */
export interface Cell {
  monthIndex: number;
  year: number;
  /** 1 to 12. */
  month: number;
  value: number;
  /** NaN when the plane has no uncertainty (energy). */
  sigma: number;
  step: RampStep;
  /** From near-real-time data, not yet science quality. */
  provisional: boolean;
  /** The median of the same calendar month over the baseline years; NaN when there is none. */
  median: number;
  /** `(value - median) / max(median, 1)` from the core; NaN when there is no median. */
  anomaly: number;
  /** Flagged by the core: a rise past the sensor band and above every baseline year. */
  unusual: boolean;
}

/** Years down, months across. A column holds `null` where the series has no month. */
export interface Grid {
  years: number[];
  rows: (Cell | null)[][];
}

/** The year of a month index (`docs/data-format.md`: index 0 is January 2000). */
export function yearOfMonthIndex(monthIndex: number): number {
  return FIRST_YEAR + Math.floor(monthIndex / 12);
}

/** The month, 1 to 12, of a month index. */
export function monthOfMonthIndex(monthIndex: number): number {
  return (monthIndex % 12) + 1;
}

/** Places every month of the series in its year row and month column. Oldest year first. */
export function buildGrid(series: CalendarSeries): Grid {
  const count = series.value.length;
  if (count === 0) return { years: [], rows: [] };

  const first = series.first_month_index;
  const firstYear = yearOfMonthIndex(first);
  const lastYear = yearOfMonthIndex(first + count - 1);
  const years = Array.from({ length: lastYear - firstYear + 1 }, (_, at) => firstYear + at);
  const max = series.value.reduce((largest, value) => Math.max(largest, value), 0);
  const provisionalFrom = series.provisional_from ?? Number.POSITIVE_INFINITY;

  const rows: (Cell | null)[][] = years.map(() => Array.from({ length: 12 }, () => null));
  series.value.forEach((value, at) => {
    const monthIndex = first + at;
    const row = rows[yearOfMonthIndex(monthIndex) - firstYear];
    if (row === undefined) return;
    row[monthOfMonthIndex(monthIndex) - 1] = {
      monthIndex,
      year: yearOfMonthIndex(monthIndex),
      month: monthOfMonthIndex(monthIndex),
      value,
      sigma: series.sigma[at] ?? Number.NaN,
      step: rampStep(value, max),
      provisional: monthIndex >= provisionalFrom,
      median: series.median?.[monthOfMonthIndex(monthIndex) - 1] ?? Number.NaN,
      anomaly: series.anomaly?.[at] ?? Number.NaN,
      unusual: series.unusual?.[at] ?? false,
    };
  });
  return { years, rows };
}
