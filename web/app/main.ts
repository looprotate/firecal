import '@watergis/maplibre-gl-terradraw/dist/maplibre-gl-terradraw.css';
import 'maplibre-gl/dist/maplibre-gl.css';
import '../tokens/tokens.css';
import './style.css';
import sample from '../stub/series.json';
import { mountCalendar, mountOverlapFigure, type CalendarActions } from '../calendar/index.ts';
import { loadFirecal, type Firecal } from '../../wasm/load.ts';
import { createAreaMap } from '../map/map.ts';
import {
  AREA_SELECTED_EVENT,
  areaFromSelectionEvent,
  emitAreaSelected,
  parseAreaBounds,
  polygonFromSelectionEvent,
  type AreaBoundsText,
} from './area-input.ts';
import { degreesText, latitudeLabel, longitudeLabel } from './degrees.ts';
import { pinRegion } from './offline.ts';
import { pinnedSeries } from './pinned-series.ts';
import { csvFiles } from './wasm-series.ts';
import { parseSeries, type Area, type Series } from './series.ts';
import { renderShell } from './shell.ts';

const root = document.querySelector<HTMLDivElement>('#app');
if (!root) throw new Error('The app root is missing');

const sampleSeries = parseSeries(sample);
const shell = renderShell(root);

/** Everything on screen comes from this object; `render` draws it. */
const state: {
  area: Area;
  polygon?: [number, number][];
  series: Series;
  plane: Series['plane'];
  fromCore: boolean;
  actions: CalendarActions;
  bounds: AreaBoundsText;
  error: string;
} = {
  area: sampleSeries.area,
  series: sampleSeries,
  plane: 'count',
  fromCore: false,
  actions: {},
  bounds: boundsFromArea(sampleSeries.area),
  error: '',
};
const calendar = mountCalendar(shell.calendarHost, state.series);
const areaMap = createAreaMap(shell.mapHost, (selection) => emitAreaSelected(root, selection));
areaMap.showSelection({ area: state.area });
let firecal: Firecal | undefined;
let asked = 0;

shell.areaForm.addEventListener('submit', (event) => {
  event.preventDefault();
  state.bounds = readAreaBounds(shell.areaInputs);
  const result = parseAreaBounds(state.bounds);
  if (!result.area) {
    state.error = result.error;
    render();
    refuse(shell.areaForm);
    return;
  }
  emitAreaSelected(root, result.area);
});

root.addEventListener(AREA_SELECTED_EVENT, (event) => {
  const area = areaFromSelectionEvent(event);
  if (!area) return;
  state.polygon = polygonFromSelectionEvent(event);
  state.area = area;
  state.bounds = boundsFromArea(area);
  state.error = '';
  areaMap.showSelection({ area, ...(state.polygon ? { polygon: state.polygon } : {}) });
  render();
  void answerFromPinnedTiles();
});

shell.pinRegionButton.addEventListener('click', () => void pinSelectedRegion());

render();
void loadRuntime();
void showOverlapReport();
if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('/service-worker.js').catch((error: unknown) => {
    shell.pinStatus.textContent = `Offline use is unavailable: ${error instanceof Error ? error.message : String(error)}`;
  });
}

function render(): void {
  shell.areaInputs.west.value = state.bounds.west;
  shell.areaInputs.south.value = state.bounds.south;
  shell.areaInputs.east.value = state.bounds.east;
  shell.areaInputs.north.value = state.bounds.north;
  shell.areaStatus.textContent = state.error;
  const { west, south, east, north } = state.area;
  const shape = state.polygon ? 'Polygon in ' : '';
  shell.areaLabel.textContent = `${shape}${latitudeLabel(south)} to ${latitudeLabel(north)}, ${longitudeLabel(west)} to ${longitudeLabel(east)}`;
  shell.dataSource.textContent = state.fromCore ? 'Harmonized tiles, answered on this device' : 'Hand-made sample, not data';
  calendar.update(state.series, state.actions);
}

/** Moves what was refused, the way a wrong password shakes: the message says why. */
function refuse(element: HTMLElement): void {
  element.classList.remove('refused');
  void element.offsetWidth;
  element.classList.add('refused');
}

function readAreaBounds(inputs: Record<keyof AreaBoundsText, HTMLInputElement>): AreaBoundsText {
  return { west: inputs.west.value, south: inputs.south.value, east: inputs.east.value, north: inputs.north.value };
}

function boundsFromArea(area: Area): AreaBoundsText {
  return {
    west: degreesText(area.west),
    south: degreesText(area.south),
    east: degreesText(area.east),
    north: degreesText(area.north),
  };
}

async function loadRuntime(): Promise<void> {
  try {
    const response = await fetch('/firecal.wasm');
    if (!response.ok) throw new Error('Run `cargo xtask build-wasm` from the repository root first.');
    firecal = await loadFirecal(await response.arrayBuffer());
    shell.runtimeStatus.textContent = 'Core loaded. The calendar shows the hand-made sample until a region is pinned.';
    await answerFromPinnedTiles();
  } catch (error) {
    shell.runtimeStatus.textContent = error instanceof Error ? error.message : 'The core could not be loaded.';
  }
}

async function pinSelectedRegion(): Promise<void> {
  shell.pinRegionButton.disabled = true;
  shell.pinStatus.textContent = 'Saving the tiles of this region…';
  try {
    const report = await pinRegion(state.area);
    const absent = report.missing === 0
      ? ''
      : `; ${plural(report.missing, 'tile')} without fire ${report.missing === 1 ? 'is' : 'are'} not published`;
    shell.pinStatus.textContent = `Pinned ${plural(report.tiles, 'tile')}, ${formatBytes(report.bytes)}${absent}.`;
    await answerFromPinnedTiles();
  } catch (error) {
    shell.pinStatus.textContent = error instanceof Error ? error.message : 'The region could not be pinned.';
  } finally {
    shell.pinRegionButton.disabled = false;
  }
}

async function answerFromPinnedTiles(): Promise<void> {
  const question = ++asked;
  if (!firecal) return;
  try {
    const core = firecal;
    const about = { area: state.area, plane: state.plane, ...(state.polygon ? { polygon: state.polygon } : {}) };
    const answer = await pinnedSeries(core, about);
    if (question !== asked) return;
    if (answer === null) {
      state.series = sampleSeries;
      state.plane = 'count';
      state.fromCore = false;
      state.actions = {};
      render();
      shell.runtimeStatus.textContent = 'No pinned tiles here. The calendar shows the hand-made sample.';
      return;
    }
    state.series = answer.series;
    state.fromCore = true;
    state.actions = {
      choosePlane: (plane) => {
        state.plane = plane;
        void answerFromPinnedTiles();
      },
      csv: csvFiles(core, answer.query, state.polygon !== undefined),
    };
    render();
    shell.runtimeStatus.textContent = `From the core, on ${plural(answer.tiles, 'pinned tile')}, ${plural(answer.series.months.length, 'month')}.`;
  } catch (error) {
    if (question !== asked) return;
    shell.runtimeStatus.textContent = error instanceof Error ? error.message : 'The pinned tiles could not be read.';
  }
}

/** The harmonization figure, when the host publishes Cube's overlap report next to the tiles. */
async function showOverlapReport(): Promise<void> {
  try {
    const response = await fetch('/tiles/overlap.csv');
    const text = response.ok ? await response.text() : '';
    if (!text.startsWith('tile_row,')) return;
    mountOverlapFigure(shell.overlapHost, text);
    shell.harmonization.hidden = false;
  } catch {
    // No report and no network: the panel stays hidden, as on a host that does not publish one.
  }
}

function formatBytes(bytes: number): string {
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KiB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MiB`;
}

function plural(count: number, noun: string): string {
  return `${count} ${noun}${count === 1 ? '' : 's'}`;
}
