import { arc, archPoints, path, resample, rough, rrect } from '../../site/js/draw.js';
import type { AreaBoundsText } from './area-input.ts';
import { BOIL_FILTER } from './boil.ts';
import { drawnLogo } from './logo.ts';

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

/** A section marker drawn by hand: a pastel shape with an ink outline, 20 pixels square. */
const marker = (kind: string, points: number[][], seed: number): string => {
  const d = path(rough(resample(points, true), seed, 0.45, 0, 0), true);
  return `<svg class="marker marker-${kind}" viewBox="0 0 20 20" aria-hidden="true"><path class="marker-fill" d="${d}"/><path class="marker-line" pathLength="1" d="${d}"/></svg>`;
};

/** The section markers: a rectangle for the area you draw, an arch for the calendar, a half sun for the sensors. */
const MARKER = {
  area: marker('area', rrect(2, 4, 16, 12, 2), 11),
  calendar: marker('calendar', archPoints(13 / 440).map(([x, y]) => [(x ?? 0) + 10, (y ?? 0) + 18.5]), 13),
  harmonization: marker('harmonization', arc(10, 15, 8, 8, Math.PI, Math.PI * 2, 12), 17),
};

const field = (name: keyof AreaBoundsText, label: string, limit: number): string =>
  `<label>${label}<span class="field" data-frame="field"><input name="${name}" type="number" inputmode="decimal" min="${-limit}" max="${limit}" step="any" required autocomplete="off"></span></label>`;

/** Writes the page: the area panel with the map and the coordinates, and the calendar panel. */
export function renderShell(root: HTMLElement): AppShell {
  root.innerHTML = `
    ${BOIL_FILTER}
    <header class="topbar">
      <a class="brand" href="/" aria-label="firecal, about the project">${drawnLogo()}<span>firecal</span></a>
      <p class="topbar-caption">Burning activity from MODIS and VIIRS, harmonized</p>
    </header>
    <main class="workspace">
      <section class="intro" aria-labelledby="page-title">
        <h1 id="page-title">Burning activity, month by month.</h1>
        <p class="intro-copy">Pick an area. The calendar shows the hotspots seen each month, how sure that number is, and how it compares with past years.</p>
      </section>
      <div class="workspace-grid">
        <section class="panel" data-frame="panel" aria-labelledby="area-title">
          <div class="panel-heading">
            <h2 id="area-title">${MARKER.area}Area</h2>
            <span class="area-label" id="area-label"></span>
          </div>
          <div class="map-frame" data-frame="map"><div class="map" id="map" role="region" aria-label="Map: draw a rectangle or a polygon"></div></div>
          <form class="area-form" id="area-form" aria-label="Area by coordinates" novalidate>
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
        <section class="panel" data-frame="panel" aria-labelledby="calendar-title">
          <div class="panel-heading">
            <h2 id="calendar-title">${MARKER.calendar}Calendar</h2>
          </div>
          <div class="calendar-mount" id="calendar"></div>
          <output class="runtime-status" id="runtime-status" aria-live="polite">Loading the core…</output>
        </section>
      </div>
      <section class="panel harmonization" data-frame="panel" id="harmonization" aria-labelledby="harmonization-title" hidden>
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
