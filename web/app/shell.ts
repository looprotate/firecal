import logo from '../tokens/logo.svg';
import type { AreaBoundsText } from './area-input.ts';

/** The parts of the page the app reads and writes. */
export interface AppShell {
  mapHost: HTMLElement;
  calendarHost: HTMLElement;
  runtimeStatus: HTMLOutputElement;
  areaForm: HTMLFormElement;
  areaInputs: Record<keyof AreaBoundsText, HTMLInputElement>;
  areaStatus: HTMLOutputElement;
  areaLabel: HTMLElement;
  pinRegionButton: HTMLButtonElement;
  pinStatus: HTMLOutputElement;
  dataSource: HTMLElement;
  harmonization: HTMLElement;
  overlapHost: HTMLElement;
}

/** The section markers: a rectangle for the area you draw, an arch for the calendar. */
const MARKER = {
  area: '<svg class="marker" viewBox="0 0 10 10" aria-hidden="true"><rect x="0" y="2" width="10" height="7"/></svg>',
  calendar: '<svg class="marker" viewBox="0 0 10 10" aria-hidden="true"><path d="M0 10V5a5 5 0 0 1 10 0v5z"/></svg>',
  harmonization: '<svg class="marker" viewBox="0 0 10 10" aria-hidden="true"><path d="M0 8a5 5 0 0 1 10 0z"/></svg>',
};

const field = (name: keyof AreaBoundsText, label: string, limit: number): string =>
  `<label>${label}<input name="${name}" type="number" inputmode="decimal" min="${-limit}" max="${limit}" step="any" required autocomplete="off"></label>`;

/** Writes the page: the area panel with the map and the coordinates, and the calendar panel. */
export function renderShell(root: HTMLElement): AppShell {
  root.innerHTML = `
    <header class="topbar">
      <a class="brand" href="/"><img src="${logo}" alt="" width="24" height="24"><span>firecal</span></a>
      <p class="topbar-caption">Burning activity from MODIS and VIIRS, harmonized</p>
    </header>
    <main class="workspace">
      <section class="intro" aria-labelledby="page-title">
        <h1 id="page-title">Fire activity, month by month.</h1>
        <p class="intro-copy">Pick an area. The calendar shows how much burned each month, how sure that number is, and how it compares with past years.</p>
      </section>
      <div class="workspace-grid">
        <section class="panel" aria-labelledby="area-title">
          <div class="panel-heading">
            <h2 id="area-title">${MARKER.area}Area</h2>
            <span class="area-label" id="area-label"></span>
          </div>
          <div class="map-frame" id="map" role="region" aria-label="Map: draw a rectangle or a polygon"></div>
          <form class="area-form" id="area-form" aria-label="Area by coordinates">
            <div class="coordinate-fields">
              ${field('west', 'West', 180)}${field('south', 'South', 90)}${field('east', 'East', 180)}${field('north', 'North', 90)}
            </div>
            <div class="form-actions">
              <output class="area-status" id="area-status" aria-live="polite"></output>
              <button type="submit" class="primary">Use this area</button>
            </div>
          </form>
          <div class="pin-region">
            <button type="button" id="pin-region">Pin for offline use</button>
            <output class="pin-status" id="pin-status" aria-live="polite"></output>
          </div>
        </section>
        <section class="panel" aria-labelledby="calendar-title">
          <div class="panel-heading">
            <h2 id="calendar-title">${MARKER.calendar}Calendar</h2>
          </div>
          <div class="calendar-mount" id="calendar"></div>
          <output class="runtime-status" id="runtime-status" aria-live="polite">Loading the core…</output>
        </section>
      </div>
      <section class="panel harmonization" id="harmonization" aria-labelledby="harmonization-title" hidden>
        <div class="panel-heading">
          <h2 id="harmonization-title">${MARKER.harmonization}How the sensors were made to agree</h2>
        </div>
        <div id="overlap"></div>
      </section>
      <footer class="page-footer"><span>NASA FIRMS hotspots, MODIS and VIIRS</span><span id="data-source"></span></footer>
    </main>
  `;
  const areaForm = required(root, '#area-form', HTMLFormElement);
  return {
    mapHost: required(root, '#map', HTMLElement),
    calendarHost: required(root, '#calendar', HTMLElement),
    runtimeStatus: required(root, '#runtime-status', HTMLOutputElement),
    areaForm,
    areaInputs: {
      west: required(areaForm, '[name="west"]', HTMLInputElement),
      south: required(areaForm, '[name="south"]', HTMLInputElement),
      east: required(areaForm, '[name="east"]', HTMLInputElement),
      north: required(areaForm, '[name="north"]', HTMLInputElement),
    },
    areaStatus: required(root, '#area-status', HTMLOutputElement),
    areaLabel: required(root, '#area-label', HTMLElement),
    pinRegionButton: required(root, '#pin-region', HTMLButtonElement),
    pinStatus: required(root, '#pin-status', HTMLOutputElement),
    dataSource: required(root, '#data-source', HTMLElement),
    harmonization: required(root, '#harmonization', HTMLElement),
    overlapHost: required(root, '#overlap', HTMLElement),
  };
}

function required<T extends Element>(root: ParentNode, selector: string, type: new () => T): T {
  const element = root.querySelector(selector);
  if (!(element instanceof type)) throw new Error(`The page is missing ${selector}`);
  return element;
}
