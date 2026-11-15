import type { CalendarSeries } from './series.ts';

type Plane = CalendarSeries['plane'];

/** What the page lets the calendar do. The calendar draws the controls; the page asks the core. */
export interface CalendarActions {
  /** Asks the core for the other plane; the page then calls `update`. Absent: the series has one plane only. */
  choosePlane?(plane: Plane): void;
  /** The core's `series.csv` and `cells.csv` for what is on screen, made when asked; no `cells` for a polygon. */
  csv?: { series(): Uint8Array; cells?(): Uint8Array };
}

/** The controls the actions call for. */
export function toolbarParts(actions: CalendarActions): ToolbarParts {
  return { planes: actions.choosePlane !== undefined, csv: actions.csv !== undefined, cells: actions.csv?.cells !== undefined };
}

/** Which controls to draw. */
export interface ToolbarParts {
  planes: boolean;
  csv: boolean;
  cells: boolean;
}

const PLANES: [Plane, string][] = [['count', 'Hotspot count'], ['energy', 'Radiative power']];

/** The controls above the calendar, or nothing when the page offers no action. */
export function renderToolbar(plane: Plane, parts: ToolbarParts): string {
  const planes = parts.planes
    ? `<div class="calendar-planes" role="group" aria-label="Measure">${PLANES.map(([name, label]) =>
      `<button type="button" data-plane="${name}" aria-pressed="${name === plane}">${label}</button>`).join('')}</div>`
    : '';
  const cells = parts.cells
    ? '<button type="button" data-file="cells" aria-label="Download cells.csv">Grid CSV ↓</button>'
    : '<span class="calendar-note">cells.csv is for rectangles</span>';
  const files = parts.csv
    ? `<div class="calendar-files"><button type="button" data-file="series" aria-label="Download series.csv">Monthly CSV ↓</button>${cells}</div>`
    : '';
  return planes || files ? `<div class="calendar-toolbar">${planes}${files}</div>` : '';
}

/** Saves `bytes` as a file named `name` through the browser's download. */
export function download(name: string, bytes: Uint8Array, type = 'text/csv'): void {
  const url = URL.createObjectURL(new Blob([bytes.slice()], { type }));
  const link = document.createElement('a');
  link.href = url;
  link.download = name;
  link.click();
  URL.revokeObjectURL(url);
}
