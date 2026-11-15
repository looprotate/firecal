import { rampStep, type RampStep } from '../calendar/ramp.ts';
import { formatNumber, monthTitle } from '../calendar/labels.ts';
import type { AreaMap } from '../map/map.ts';
import type { Series } from './series.ts';

/** A presentation step for a real core value, using the calendar's own scale. */
export function monthlyShade(series: Series | null, monthIndex?: number): { monthIndex: number; value: number; step: RampStep } | null {
  if (!series || series.value.length === 0) return null;
  const month = monthIndex ?? series.first_month_index + series.value.length - 1;
  const offset = month - series.first_month_index;
  if (!Number.isInteger(offset) || offset < 0 || offset >= series.value.length) return null;
  const value = series.value[offset];
  if (value === undefined || !Number.isFinite(value)) return null;
  const max = series.value.reduce((largest, item) => Number.isFinite(item) ? Math.max(largest, item) : largest, 0);
  return { monthIndex: month, value, step: rampStep(value, max) };
}

/** Connects month hover, keyboard focus and taps to the actual area total, without computing new statistics. */
export function bindMapActivity(root: HTMLElement, map: AreaMap, readSeries: () => Series | null, onMonth?: (month: number) => void): { refresh(monthIndex?: number): void; clear(): void } {
  const label = root.querySelector<HTMLOutputElement>('#map-activity-label');
  const clear = (): void => {
    map.setActivity(null);
    if (label) label.textContent = 'No activity data for this selection.';
  };
  const show = (monthIndex?: number): void => {
    const series = readSeries();
    const shade = monthlyShade(series, monthIndex);
    if (!shade || !series) { clear(); return; }
    const color = getComputedStyle(document.documentElement).getPropertyValue(`--ramp-${shade.step}`).trim();
    map.setActivity(color);
    const year = 2000 + Math.floor(shade.monthIndex / 12);
    const month = shade.monthIndex % 12 + 1;
    onMonth?.(shade.monthIndex);
    if (label) label.textContent = `${monthTitle(year, month)} · ${formatNumber(shade.value)} ${series.plane === 'energy' ? 'MW' : 'hotspots'}`;
    root.querySelectorAll('.map-month-selected').forEach((cell) => cell.classList.remove('map-month-selected'));
    const firstYear = 2000 + Math.floor(series.first_month_index / 12);
    root.querySelector(`#calendar [data-row="${year - firstYear}"][data-column="${month - 1}"]`)?.classList.add('map-month-selected');
  };
  const select = (event: Event): void => {
    if (!(event.target instanceof Element)) return;
    const cell = event.target.closest<SVGElement>('#calendar [data-row][data-column]');
    const series = readSeries();
    if (!cell || !series) return;
    const firstYearMonth = Math.floor(series.first_month_index / 12) * 12;
    show(firstYearMonth + Number(cell.dataset.row) * 12 + Number(cell.dataset.column));
  };
  for (const event of ['pointerover', 'focusin', 'click']) root.addEventListener(event, select);
  return { refresh: (month) => show(month), clear };
}
