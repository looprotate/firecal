/**
 * What the calendar reads from a Series: the plane, where it starts, one value and sigma per month, and the first
 * month from near-real-time data, if the series says.
 */
export interface CalendarSeries {
  plane: 'count' | 'energy';
  first_month_index: number;
  value: number[];
  sigma: number[];
  provisional_from?: number | null;
}
