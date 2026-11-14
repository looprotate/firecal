/** What the calendar reads from a Series: the plane, where it starts, and one value and sigma per month. */
export interface CalendarSeries {
  plane: 'count' | 'energy';
  first_month_index: number;
  value: number[];
  sigma: number[];
}
