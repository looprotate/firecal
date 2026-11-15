import { organizeScreens } from './screens.ts';
import { ANALYSIS_TOOLS } from './analysis-tools.ts';
import { SUPPORT_INFO } from './support.ts';
import { renderContinentPicker } from './continents.ts';
import { PROJECT_INFO } from './project-info.ts';
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
      <span class="brand"><span>firecal<span class="brand-period">.</span></span><span class="brand-logo-slot"><img id="brand-logo" class="brand-logo" src="/branding/firecal-tree.png" alt="" aria-hidden="true" width="34" height="40" fetchpriority="high"></span></span>
      <button class="menu-toggle" type="button" aria-label="Open menu" aria-expanded="false" aria-controls="navigation-drawer"><span aria-hidden="true">☰</span></button><nav id="main-navigation" aria-label="Main navigation"><a href="/index.html">Home</a><a href="/explorer.html">Explorer</a><a href="/approach.html">Our approach</a><a href="/data.html">Data &amp; references</a><a href="/about.html">About the tool</a><a href="/support.html">Support</a></nav>
      <span class="edition">NASA SPACE APPS · 2026</span>
      <label class="language-control"><span class="language-symbol" aria-hidden="true">◎</span><span class="sr-only">Language</span><select id="language-select" aria-label="Language"><option value="en" lang="en">English</option><option value="es" lang="es">Español</option><option value="de" lang="de">Deutsch</option><option value="fr" lang="fr">Français</option><option value="zh" lang="zh">中文</option></select></label>
    </header>
    <main>
      <section class="intro" aria-labelledby="page-title">
        <div class="intro-words"><p class="eyebrow"><span class="live-dot"></span> A FIELD GUIDE TO BURNING ACTIVITY</p>
        <h1 id="page-title">A longer view<br>of <em>burning.</em></h1>
        <p class="intro-copy">Two generations of satellites. One comparable record.<br>Explore the seasons, understand the uncertainty,<br class="desktop-break"> and take the history into the field.</p><div class="intro-actions"><a class="intro-help explore-button" href="/explorer.html"><svg viewBox="0 0 24 24" width="19" height="19" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 5.5C9 3.5 5.5 3.5 2 4.5v15c3.5-1 7-1 10 1 3-2 6.5-2 10-1v-15c-3.5-1-7-1-10 1zM12 5.5v15"/></svg><span>Explore the data</span></a></div></div>
        <span class="sr-only">Real photograph of a forest wildfire at night.</span>
        <span class="photo-credit">Photograph supplied with the team design</span>
      </section>
      <div class="workspace">
      <ol class="quick-steps" aria-label="Getting started"><li><span>1</span><div><strong>Choose a region</strong><small>Choose a continent or draw an area.</small></div></li><li><span>2</span><div><strong>Explore the months</strong><small>Compare activity and uncertainty.</small></div></li><li><span>3</span><div><strong>Keep what you find</strong><small>Download a CSV or save for offline use.</small></div></li></ol>
      <div class="explorer-topline" id="explore"><span>THE EXPLORER</span><span id="connection-state">● Connected</span></div>
      <div class="workspace-grid">
        <section class="panel area-panel" aria-labelledby="area-title">
          <div class="panel-heading"><h2 id="area-title"><span class="section-number">01</span> Choose your area</h2></div>
          <p class="panel-copy">Draw a rectangle or polygon on the map,<br>or enter the coordinates below.</p>
          ${renderContinentPicker()}<p class="coverage-note">Six capital regions available · 2012–2019; Australia also includes 2000–2019</p>
          <div class="map-unit"><div class="map-container"><div class="map-frame" id="map" role="region" aria-label="Map: draw a rectangle or a polygon"></div><button class="map-expand" id="map-expand" type="button" aria-controls="map-dialog" aria-expanded="false"><span id="map-expand-label">Expand map</span><span aria-hidden="true">⤢</span></button></div>
          <div class="map-activity-key" aria-live="polite"><div class="map-key-top"><strong>Monthly activity</strong><span class="map-swatches" aria-hidden="true"><i></i><i></i><i></i><i></i><i></i></span></div><output id="map-activity-label">Choose a month in the calendar.</output><p>Area total for the selected month. Darker means more activity within the displayed period.</p></div>
          </div>
          <div class="map-caption"><span>AREA BOUNDS</span><span class="area-label" id="area-label"></span></div>
          <details class="coordinate-details"><summary>Enter coordinates instead</summary><form class="area-form" id="area-form" aria-label="Area by coordinates"><div class="coordinate-fields">
            ${field('west', 'West', 180)}${field('south', 'South', 90)}${field('east', 'East', 180)}${field('north', 'North', 90)}
          </div><div class="form-actions"><output class="area-status" id="area-status" aria-live="polite"></output><button type="submit" class="primary">Use this area <span aria-hidden="true">↗</span></button></div></form></details>
          <div class="pin-region"><div class="offline-heading"><span class="offline-shape" aria-hidden="true"></span><h3>Take it with you.</h3></div><p>Save this region while you have signal.<br>Ask new questions when you don’t.</p><button type="button" id="pin-region">Pin for offline use <span aria-hidden="true">↓</span></button><output class="pin-status" id="pin-status" aria-live="polite"></output></div>
        </section>
        <section class="panel calendar-panel" aria-labelledby="calendar-title">
          <div class="panel-heading"><h2 id="calendar-title"><span class="section-number">02</span> Read the seasons</h2><span class="data-tag">MODIS + VIIRS</span></div>
          <p class="panel-copy">Months across. Years down. The whole story in one view.</p>
          <div class="data-notice"><span class="data-notice-dot"></span><span id="data-source">Hand-made sample, not data</span></div>
          <a class="explorer-guide" href="#reading-guide">How to read it</a><p class="interaction-hint">Hover or tap a month to see its value and estimated uncertainty.</p>
          <div class="calendar-empty" id="calendar-empty" hidden><h3>Map location selected</h3><p>This area is outside the published regions. Choose a continent to open a capital with real data.</p></div>${ANALYSIS_TOOLS}<div class="calendar-mount" id="calendar"></div>
          <details class="calendar-help" id="reading-guide"><summary>How to read this calendar</summary><ul><li>Each square is one month. Darker squares mean more activity.</li><li>A diamond marks a month outside the uncertainty band and the historical range.</li><li>Blue lines show estimated uncertainty. Similar years are context, not a forecast.</li><li>Switch to radiative power to compare intensity, or download the monthly results.</li></ul></details>
          <output class="runtime-status" id="runtime-status" aria-live="polite">Loading the calendar…</output>
        </section>
      </div>
      <section class="method" id="method" aria-labelledby="method-title"><div class="method-intro"><p class="eyebrow">THE THINKING BEHIND THE CALENDAR</p><h2 id="method-title">Better context.<br><em>Honest numbers.</em></h2><p>Different sensors see differently. We make their records comparable, and keep the uncertainty in sight.</p><p class="method-extra">Counts are aggregated monthly on a 0.25° grid. The calendar, map shading, comparison and CSV all read the same core results.</p></div><div class="method-notes"><article><span class="method-glyph circle" aria-hidden="true"></span><div><h3>One record, two satellite families.</h3><p>VIIRS counts are adjusted to MODIS-equivalent counts using the years both observed. More detections don’t automatically mean more burning.</p></div></article><article><span class="method-glyph band" aria-hidden="true"></span><div><h3>Uncertainty is part of the answer.</h3><p>A month is flagged only when it clears the estimated uncertainty band and the range of the baseline years.</p></div></article><article><span class="method-glyph arch" aria-hidden="true"></span><div><h3>A little perspective from the past.</h3><p>Find the seasons that looked like this one. Analog years are historical context, never a forecast.</p></div></article></div></section>
      <section class="panel harmonization" id="harmonization" aria-labelledby="harmonization-title" hidden><div class="panel-heading"><h2 id="harmonization-title">${MARKER.harmonization}How the sensors were made to agree</h2></div><div id="overlap"></div></section>
      ${PROJECT_INFO}
      ${SUPPORT_INFO}
      <footer class="page-footer"><span class="footer-brand">firecal.</span><p>Made for the field. Built on NASA FIRMS data.<br><span>Hotspots are thermal anomalies, not measured fires or burned area.</span></p><span class="footer-note">NASA SPACE APPS · 2026</span></footer>
      </div>
      <dialog class="navigation-drawer" id="navigation-drawer" aria-labelledby="navigation-drawer-title"><div class="navigation-drawer-heading"><h2 id="navigation-drawer-title">Navigation</h2><button class="navigation-drawer-close" type="button" aria-label="Close menu">×</button></div><div class="navigation-drawer-content"></div></dialog>
      <dialog class="map-dialog" id="map-dialog" aria-labelledby="map-dialog-title"><div class="map-dialog-bar"><div><p class="eyebrow">YOUR SELECTED REGION</p><h2 id="map-dialog-title">Explore your area</h2></div><button id="map-dialog-close" type="button">Back to calendar ×</button></div><div id="map-dialog-content"></div><p class="map-dialog-hint">Draw or zoom to explore. Return to the calendar to compare months.</p></dialog>
    </main>
  `;
  organizeScreens(root);
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
