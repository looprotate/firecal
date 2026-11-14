import type { Series } from './series.ts';

/** A calendar view supplied by the calendar package at the integration point. */
export interface CalendarRenderer {
  mount(host: HTMLElement, series: Series): void;
}

export interface AppShell {
  mapHost: HTMLElement;
  calendarHost: HTMLElement;
  runtimeStatus: HTMLOutputElement;
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
            <span class="map-bounds">${formatLongitude(series.area.west)} / ${formatLongitude(series.area.east)}</span>
          </div>
          <div class="map-frame" id="map" aria-label="Map area picker"></div>
          <div class="map-footer"><span>Draw on the map or enter coordinates</span><span class="map-coordinates">${formatLongitude(series.area.west)}&nbsp; · &nbsp;${formatLatitude(series.area.south)}</span></div>
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
  return { mapHost, calendarHost, runtimeStatus };
}

/** Passes the validated Series and mount point to a calendar implementation. */
export function mountCalendar(host: HTMLElement, series: Series, renderer: CalendarRenderer): void {
  renderer.mount(host, series);
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

function formatCoordinate(value: number): string {
  return Number.isInteger(value) ? String(Math.abs(value)) : Math.abs(value).toFixed(1);
}

function formatLatitude(value: number): string {
  return `${formatCoordinate(value)}° ${value < 0 ? 'S' : 'N'}`;
}

function formatLongitude(value: number): string {
  return `${formatCoordinate(value)}° ${value < 0 ? 'W' : 'E'}`;
}
