import { buildGrid, type Grid } from './grid.ts';
import { describeCell } from './labels.ts';
import { nextPosition, type Position } from './keyboard.ts';
import { renderLegendSvg } from './legend.ts';
import { FOCUS_RING_OFFSET, renderCalendarSvg } from './render.ts';
import type { CalendarSeries } from './series.ts';

/** A calendar drawn into a page element. */
export interface Calendar {
  /** Draws another Series in the same place. */
  update(series: CalendarSeries): void;
  /** Removes the calendar and its listeners. */
  destroy(): void;
}

function positionOf(target: EventTarget | null): Position | null {
  if (!(target instanceof Element)) return null;
  const cell = target.closest<SVGElement>('[data-row]');
  if (cell === null) return null;
  return { row: Number(cell.dataset.row), column: Number(cell.dataset.column) };
}

function cellAt(root: HTMLElement, at: Position): SVGElement | null {
  return root.querySelector<SVGElement>(`[data-row="${at.row}"][data-column="${at.column}"]`);
}

/**
 * Draws the calendar of `series` into `container`, with a readout line under it. The value of a month shows
 * in the readout on hover and on focus; the arrow keys move between months.
 */
export function mountCalendar(container: HTMLElement, series: CalendarSeries): Calendar {
  let current = series;
  let grid: Grid = buildGrid(series);
  const readout = document.createElement('p');
  readout.className = 'calendar-readout';
  readout.setAttribute('aria-live', 'polite');
  const host = document.createElement('div');
  host.className = 'calendar';
  const legend = document.createElement('div');
  legend.className = 'calendar';
  legend.innerHTML = renderLegendSvg();
  container.replaceChildren(host, readout, legend);

  const show = (at: Position | null): void => {
    const cell = at === null ? null : (grid.rows[at.row]?.[at.column] ?? null);
    readout.textContent = cell === null ? '' : describeCell(cell, current.plane);
  };

  const draw = (): void => {
    grid = buildGrid(current);
    host.innerHTML = renderCalendarSvg(grid, current.plane);
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
    ring.setAttribute('x', String(Number(cell.getAttribute('x')) - FOCUS_RING_OFFSET));
    ring.setAttribute('y', String(Number(cell.getAttribute('y')) - FOCUS_RING_OFFSET));
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

  host.addEventListener('focusin', onFocusIn);
  host.addEventListener('focusout', onFocusOut);
  host.addEventListener('mouseover', onFocusOrHover);
  host.addEventListener('mouseleave', onLeave);
  host.addEventListener('keydown', onKeyDown);
  draw();

  return {
    update(next) {
      current = next;
      draw();
    },
    destroy() {
      host.removeEventListener('focusin', onFocusIn);
      host.removeEventListener('focusout', onFocusOut);
      host.removeEventListener('mouseover', onFocusOrHover);
      host.removeEventListener('mouseleave', onLeave);
      host.removeEventListener('keydown', onKeyDown);
      container.replaceChildren();
    },
  };
}
