/**
 * What the calendar reads from a Series: the plane, where it starts, one value and sigma per month, the core's
 * comparison with past years and the first month from near-real-time data, when the series has them.
 */
export interface CalendarSeries {
  plane: 'count' | 'energy';
  /** Optional shared presentation scale when comparing two areas. */
  scale_max?: number;
  first_month_index: number;
  value: number[];
  sigma: number[];
  /** The core's comparison, when the series has one: the median of each calendar month, January first. */
  median?: number[];
  anomaly?: number[];
  unusual?: boolean[];
  provisional_from?: number | null;
  /** The past years most like the season in progress, nearest first. */
  analogs?: { year: number; distance: number }[];
}
