import type { AreaBoundsText } from './area-input.ts';
import type { Series } from './series.ts';

export interface AppShell {
  mapHost: HTMLElement;
  calendarHost: HTMLElement;
  runtimeStatus: HTMLOutputElement;
  areaForm: HTMLFormElement;
  areaInputs: Record<keyof AreaBoundsText, HTMLInputElement>;
  areaStatus: HTMLOutputElement;
  mapBounds: HTMLElement;
  mapCoordinates: HTMLElement;
  pinRegionButton: HTMLButtonElement;
  pinStatus: HTMLOutputElement;
}

/** Builds the shared page shell and gives the calendar package a stable mount point. */
export function renderShell(root: HTMLElement, series: Series): AppShell {
  const metricLabel = series.plane === 'count' ? 'HOTSPOT COUNTS' : 'FIRE ENERGY';
  root.innerHTML = `
    <header class="topbar">
      <a class="wordmark" href="/" aria-label="Firecal home">FIRECAL<span>●</span></a>
      <p class="topbar-caption">SATELLITE FIRE ACTIVITY</p>
      <p class="topbar-status"><span class="status-dot" aria-hidden="true"></span> FIELD MODE</p>
    </header>
    <main class="workspace">
      <section class="intro" aria-labelledby="page-title">
        <div>
          <p class="eyebrow">SOUTH AMERICA · ${metricLabel}</p>
          <h1 id="page-title">Fire activity, over time.</h1>
          <p class="intro-copy">Satellite observations, harmonized into a calendar for the place you choose.</p>
        </div>
        <div class="series-summary" aria-label="Loaded sample series">
          <span class="summary-label">SAMPLE REGION</span>
          <strong>${formatLatitude(series.area.south)} to ${formatLatitude(series.area.north)}</strong>
          <span>${series.months.length} monthly observations</span>
        </div>
      </section>
      <section class="workspace-grid" aria-label="Area and fire calendar">
        <section class="map-card" aria-labelledby="map-title">
          <div class="card-heading">
            <div><p class="eyebrow">01 · SELECT AN AREA</p><h2 id="map-title">Map</h2></div>
            <span class="map-bounds" id="map-bounds">${formatLongitude(series.area.west)} / ${formatLongitude(series.area.east)}</span>
          </div>
          <div class="map-frame" id="map" aria-label="Map area picker"></div>
          <div class="map-footer"><span>Draw on the map or enter coordinates</span><span class="map-coordinates" id="map-coordinates">${formatLongitude(series.area.west)}&nbsp; · &nbsp;${formatLatitude(series.area.south)}</span></div>
          <form class="area-form" id="area-form" aria-label="Enter area coordinates">
            <div class="coordinate-fields">
              <label>West longitude<input name="west" type="number" min="-180" max="180" step="any" required value="${series.area.west}" autocomplete="off"></label>
              <label>South latitude<input name="south" type="number" min="-90" max="90" step="any" required value="${series.area.south}" autocomplete="off"></label>
              <label>East longitude<input name="east" type="number" min="-180" max="180" step="any" required value="${series.area.east}" autocomplete="off"></label>
              <label>North latitude<input name="north" type="number" min="-90" max="90" step="any" required value="${series.area.north}" autocomplete="off"></label>
            </div>
            <div class="area-form-actions">
              <output class="area-status" id="area-status" aria-live="polite"></output>
              <button type="submit">Use this area</button>
            </div>
          </form>
          <div class="pin-region">
            <button type="button" id="pin-region">Pin this region for offline use</button>
            <output class="pin-status" id="pin-status" aria-live="polite"></output>
          </div>
        </section>
        <section class="calendar-card" aria-labelledby="calendar-title">
          <div class="card-heading">
            <div><p class="eyebrow">02 · EXPLORE THE SEASONS</p><h2 id="calendar-title">Burning calendar</h2></div>
            <span class="mode-chip">${series.plane.toUpperCase()}</span>
          </div>
          <p class="calendar-intro">Monthly fire activity across the years, with historical context.</p>
          <div class="calendar-mount" id="calendar" aria-label="Fire activity calendar"></div>
          <output class="runtime-status" id="runtime-status" aria-live="polite">Loading the Firecal core…</output>
        </section>
      </section>
      <footer class="page-footer"><span>MODIS + VIIRS · HAND-MADE SAMPLE</span><span>OFFLINE-READY FIELD TOOL</span></footer>
    </main>
  `;

  const mapHost = requiredElement(root, '#map');
  const calendarHost = requiredElement(root, '#calendar');
  const runtimeStatus = requiredOutput(root, '#runtime-status');
  const areaForm = requiredForm(root, '#area-form');
  const areaInputs = {
    west: requiredInput(areaForm, '[name="west"]'),
    south: requiredInput(areaForm, '[name="south"]'),
    east: requiredInput(areaForm, '[name="east"]'),
    north: requiredInput(areaForm, '[name="north"]'),
  };
  const areaStatus = requiredOutput(root, '#area-status');
  const mapBounds = requiredElement(root, '#map-bounds');
  const mapCoordinates = requiredElement(root, '#map-coordinates');
  const pinRegionButton = requiredButton(root, '#pin-region');
  const pinStatus = requiredOutput(root, '#pin-status');
  return {
    mapHost,
    calendarHost,
    runtimeStatus,
    areaForm,
    areaInputs,
    areaStatus,
    mapBounds,
    mapCoordinates,
    pinRegionButton,
    pinStatus,
  };
}

function requiredElement(root: HTMLElement, selector: string): HTMLElement {
  const element = root.querySelector<HTMLElement>(selector);
  if (!element) throw new Error(`The app shell is missing ${selector}`);
  return element;
}

function requiredOutput(root: HTMLElement, selector: string): HTMLOutputElement {
  const element = root.querySelector<HTMLOutputElement>(selector);
  if (!element) throw new Error(`The app shell is missing ${selector}`);
  return element;
}

function requiredForm(root: HTMLElement, selector: string): HTMLFormElement {
  const element = root.querySelector<HTMLFormElement>(selector);
  if (!element) throw new Error(`The app shell is missing ${selector}`);
  return element;
}

function requiredButton(root: HTMLElement, selector: string): HTMLButtonElement {
  const element = root.querySelector<HTMLButtonElement>(selector);
  if (!element) throw new Error(`The app shell is missing ${selector}`);
  return element;
}

function requiredInput(root: HTMLElement, selector: string): HTMLInputElement {
  const element = root.querySelector<HTMLInputElement>(selector);
  if (!element) throw new Error(`The app shell is missing ${selector}`);
  return element;
}

function formatCoordinate(value: number): string {
  return Number.isInteger(value) ? String(Math.abs(value)) : Math.abs(value).toFixed(1);
}

function formatLatitude(value: number): string {
  return `${formatCoordinate(value)}° ${value < 0 ? 'S' : 'N'}`;
}

function formatLongitude(value: number): string {
  return `${formatCoordinate(value)}° ${value < 0 ? 'W' : 'E'}`;
}
