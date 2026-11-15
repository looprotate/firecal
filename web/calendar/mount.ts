import { renderAnalogs } from './analogs.ts';
import { buildGrid, type Grid } from './grid.ts';
import { describeCell } from './labels.ts';
import { latestPosition, nextPosition, type Position } from './keyboard.ts';
import { renderLegend } from './legend.ts';
import { renderCalendarSvg, waveDelay } from './render.ts';
import { describeSeasonMonth, renderStripSvg } from './strip.ts';
import { download, renderToolbar, toolbarParts, type CalendarActions } from './toolbar.ts';
import type { CalendarSeries } from './series.ts';

/** A calendar drawn into a page element. */
export interface Calendar {
  /** Draws another Series in the same place, with the actions the page offers for it: a new one undraws the old and
   * pops in month by month; the same one is redrawn in place, still. */
  update(series: CalendarSeries, actions?: CalendarActions): void;
  /** Removes the calendar and its listeners. */
  destroy(): void;
}

function positionOf(target: EventTarget | null): Position | null {
  if (!(target instanceof Element)) return null;
  const cell = target.closest<SVGElement>('[data-row]');
  if (cell === null) return null;
  return { row: Number(cell.dataset.row), column: Number(cell.dataset.column) };
}

/** How long the old calendar takes to undraw before the new one draws, in milliseconds. */
const LEAVE_MS = 260;
/** How long the flags and rings take to draw once the last cell is in. */
const SETTLE_MS = 900;

const stillMotion = (): boolean => window.matchMedia('(prefers-reduced-motion: reduce)').matches;

function cellAt(root: HTMLElement, at: Position): SVGElement | null {
  return root.querySelector<SVGElement>(`[data-row="${at.row}"][data-column="${at.column}"]`);
}

/**
 * Draws the calendar of `series` into `container`, with a readout line under it. The value of a month shows
 * in the readout on hover and on focus; the arrow keys move between months. `actions` add the plane switch and the
 * CSV files above it.
 */
export function mountCalendar(container: HTMLElement, series: CalendarSeries, actions: CalendarActions = {}): Calendar {
  let current = series;
  let offered = actions;
  let analogYear: number | null = null;
  const toolbar = document.createElement('div');
  let grid: Grid = buildGrid(series);
  const readout = document.createElement('p');
  readout.className = 'calendar-readout';
  const host = document.createElement('div');
  host.className = 'calendar';
  const legend = document.createElement('div');
  legend.className = 'calendar';
  legend.innerHTML = renderLegend();
  const season = document.createElement('section');
  season.className = 'calendar-season';
  const strip = document.createElement('div');
  strip.className = 'calendar';
  const seasonTitle = document.createElement('h3');
  seasonTitle.className = 'calendar-season-title';
  const seasonText = document.createElement('p');
  seasonText.className = 'calendar-season-current';
  season.append(seasonTitle, strip, seasonText);
  const analogs = document.createElement('section');
  analogs.className = 'calendar-season';
  container.replaceChildren(toolbar, host, readout, legend, season, analogs);

  const show = (at: Position | null): void => {
    const cell = at === null ? null : (grid.rows[at.row]?.[at.column] ?? null);
    readout.textContent = cell === null ? '' : describeCell(cell, current.plane);
  };

  let settle = 0;
  let leaving = 0;
  /** Draws the calendar of `current`, popping it in month by month. The class is the page's cue to boil it. */
  const enter = (): void => {
    draw();
    window.clearTimeout(settle);
    container.classList.add('entering');
    settle = window.setTimeout(() => container.classList.remove('entering'), waveDelay(grid.rows.length, 12) + SETTLE_MS);
  };
  /** Undraws what is on screen, then draws `current`; a second change while it leaves only changes what comes. */
  const replace = (): void => {
    if (stillMotion() || host.firstElementChild === null) {
      enter();
      return;
    }
    if (leaving !== 0) return;
    container.classList.remove('entering');
    container.classList.add('leaving');
    leaving = window.setTimeout(() => {
      leaving = 0;
      container.classList.remove('leaving');
      enter();
    }, LEAVE_MS);
  };

  const draw = (): void => {
    grid = buildGrid(current);
    toolbar.innerHTML = renderToolbar(current.plane, toolbarParts(offered));
    if (!(current.analogs ?? []).some((analog) => analog.year === analogYear)) analogYear = null;
    host.innerHTML = renderCalendarSvg(grid, current.plane, analogYear);
    analogs.innerHTML = renderAnalogs(current.analogs ?? [], grid, current.plane, analogYear);
    analogs.hidden = analogs.innerHTML === '';
    strip.innerHTML = renderStripSvg(grid, current.plane);
    const latest = latestPosition(grid);
    const newest = latest === null ? null : (grid.rows[latest.row]?.[latest.column] ?? null);
    season.hidden = strip.innerHTML === '' || newest === null;
    seasonTitle.textContent = `${grid.years.at(-1) ?? ''} against the median of past years`;
    seasonText.textContent = newest === null ? '' : describeSeasonMonth(newest, current.plane);
    show(null);
  };

  const moveRing = (at: Position | null): void => {
    const ring = host.querySelector<SVGElement>('.focus-ring');
    const cell = at === null ? null : cellAt(host, at);
    if (ring === null) return;
    if (cell === null) {
      ring.setAttribute('visibility', 'hidden');
      return;
    }
    ring.setAttribute('transform', `translate(${cell.dataset.x ?? 0} ${cell.dataset.y ?? 0})`);
    ring.setAttribute('visibility', 'visible');
  };

  const onFocusOrHover = (event: Event): void => show(positionOf(event.target));
  const onFocusIn = (event: Event): void => {
    const at = positionOf(event.target);
    show(at);
    moveRing(at);
  };
  const onFocusOut = (): void => moveRing(null);
  const onLeave = (): void => {
    const focused = positionOf(document.activeElement);
    show(focused);
  };
  const onKeyDown = (event: KeyboardEvent): void => {
    const from = positionOf(event.target);
    if (from === null) return;
    const to = nextPosition(grid, from, event.key);
    if (to === from) return;
    event.preventDefault();
    const target = cellAt(host, to);
    cellAt(host, from)?.setAttribute('tabindex', '-1');
    target?.setAttribute('tabindex', '0');
    target?.focus();
  };

  const onToolbarClick = (event: Event): void => {
    if (!(event.target instanceof Element)) return;
    const button = event.target.closest<HTMLButtonElement>('button');
    const plane = button?.dataset.plane;
    if ((plane === 'count' || plane === 'energy') && plane !== current.plane) offered.choosePlane?.(plane);
    const file = button?.dataset.file;
    if (file === 'series' && offered.csv) download('series.csv', offered.csv.series());
    if (file === 'cells' && offered.csv?.cells) download('cells.csv', offered.csv.cells());
  };

  const onAnalogClick = (event: Event): void => {
    if (!(event.target instanceof Element)) return;
    const year = Number(event.target.closest<HTMLButtonElement>('button[data-year]')?.dataset.year);
    if (!Number.isInteger(year)) return;
    analogYear = analogYear === year ? null : year;
    draw();
    analogs.querySelector<HTMLButtonElement>(`button[data-year="${year}"]`)?.focus();
  };

  toolbar.addEventListener('click', onToolbarClick);
  analogs.addEventListener('click', onAnalogClick);
  host.addEventListener('focusin', onFocusIn);
  host.addEventListener('focusout', onFocusOut);
  host.addEventListener('mouseover', onFocusOrHover);
  host.addEventListener('mouseleave', onLeave);
  host.addEventListener('keydown', onKeyDown);
  enter();

  return {
    update(next, nextActions = offered) {
      const same = next === current;
      current = next;
      offered = nextActions;
      if (same) draw();
      else replace();
    },
    destroy() {
      window.clearTimeout(settle);
      window.clearTimeout(leaving);
      host.removeEventListener('focusin', onFocusIn);
      host.removeEventListener('focusout', onFocusOut);
      host.removeEventListener('mouseover', onFocusOrHover);
      host.removeEventListener('mouseleave', onLeave);
      host.removeEventListener('keydown', onKeyDown);
      toolbar.removeEventListener('click', onToolbarClick);
      analogs.removeEventListener('click', onAnalogClick);
      container.replaceChildren();
    },
  };
}
