import '@watergis/maplibre-gl-terradraw/dist/maplibre-gl-terradraw.css';
import 'maplibre-gl/dist/maplibre-gl.css';
import '../tokens/tokens.css';
import './style.css';
import sample from '../stub/series.json';
import { mountCalendar } from '../calendar/index.ts';
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
import { pinRegion } from './offline.ts';
import { pinnedSeries } from './pinned-series.ts';
import { parseSeries } from './series.ts';
import { renderShell } from './shell.ts';
import type { Area } from './series.ts';

const root = document.querySelector<HTMLDivElement>('#app');
if (!root) throw new Error('The app root is missing');

const series = parseSeries(sample);
const shell = renderShell(root, series);
const state: { area: Area; polygon?: [number, number][]; series: typeof series; bounds: AreaBoundsText; error: string } = {
  area: series.area,
  series,
  bounds: boundsFromArea(series.area),
  error: '',
};
const calendar = mountCalendar(shell.calendarHost, state.series);
let firecal: Firecal | undefined;
let selection = 0;
const map = createAreaMap(shell.mapHost, (area) => emitAreaSelected(root, area));
map.once('load', () => {
  map.fitBounds([[state.area.west, state.area.south], [state.area.east, state.area.north]], { padding: 36, maxZoom: 8 });
});
map.on('error', (event) => {
  shell.runtimeStatus.textContent = `Map error: ${event.error.message}`;
});

shell.areaForm.addEventListener('submit', (event) => {
  event.preventDefault();
  state.bounds = readAreaBounds(shell.areaInputs);
  const result = parseAreaBounds(state.bounds);
  if (!result.area) {
    state.error = result.error;
    render();
    return;
  }
  emitAreaSelected(root, result.area);
});

root.addEventListener(AREA_SELECTED_EVENT, (event) => {
  const area = areaFromSelectionEvent(event);
  if (!area) return;
  state.polygon = polygonFromSelectionEvent(event);
  state.area = area;
  state.series = { ...series, area };
  state.bounds = boundsFromArea(area);
  state.error = '';
  render();
  void answerFromPinnedTiles();
  map.fitBounds([[area.west, area.south], [area.east, area.north]], { padding: 36, maxZoom: 8 });
});

shell.pinRegionButton.addEventListener('click', () => void pinSelectedRegion());

render();
void loadRuntime(shell.runtimeStatus);
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
  shell.mapBounds.textContent = `${state.area.west} / ${state.area.south} — ${state.area.east} / ${state.area.north}`;
  shell.mapCoordinates.textContent = `${state.area.west}° / ${state.area.south}°`;
  calendar.update(state.series);
}

function readAreaBounds(inputs: Record<keyof AreaBoundsText, HTMLInputElement>): AreaBoundsText {
  return {
    west: inputs.west.value,
    south: inputs.south.value,
    east: inputs.east.value,
    north: inputs.north.value,
  };
}

function boundsFromArea(area: Area): AreaBoundsText {
  return {
    west: String(area.west),
    south: String(area.south),
    east: String(area.east),
    north: String(area.north),
  };
}

async function loadRuntime(status: HTMLOutputElement): Promise<void> {
  try {
    const response = await fetch('/firecal.wasm');
    if (!response.ok) throw new Error('Run `cargo xtask build-wasm` from the repository root first.');
    firecal = await loadFirecal(await response.arrayBuffer());
    status.textContent = 'Core loaded. The calendar shows the hand-made sample until a region is pinned.';
    await answerFromPinnedTiles();
  } catch (error) {
    status.textContent = error instanceof Error ? error.message : 'The Firecal core could not be loaded.';
  }
}

async function pinSelectedRegion(): Promise<void> {
  shell.pinRegionButton.disabled = true;
  shell.pinStatus.textContent = 'Saving the tiles of this region…';
  try {
    const report = await pinRegion(state.area);
    const absent = report.missing === 0 ? '' : `; ${plural(report.missing, 'tile')} without fire ${report.missing === 1 ? 'is' : 'are'} not published`;
    shell.pinStatus.textContent = `Pinned ${plural(report.tiles, 'tile')}, ${formatBytes(report.bytes)}${absent}.`;
    await answerFromPinnedTiles();
  } catch (error) {
    shell.pinStatus.textContent = error instanceof Error ? error.message : 'The region could not be pinned.';
  } finally {
    shell.pinRegionButton.disabled = false;
  }
}

async function answerFromPinnedTiles(): Promise<void> {
  const asked = ++selection;
  if (!firecal) return;
  try {
    const answer = await pinnedSeries(firecal, state.area, state.polygon);
    if (asked !== selection) return;
    if (answer === null) {
      shell.runtimeStatus.textContent = 'No pinned tiles here. The calendar shows the hand-made sample.';
      return;
    }
    state.series = answer.series;
    render();
    shell.runtimeStatus.textContent = `From the core, on ${plural(answer.tiles, 'pinned tile')}, ${plural(answer.series.months.length, 'month')}.`;
  } catch (error) {
    if (asked !== selection) return;
    shell.runtimeStatus.textContent = error instanceof Error ? error.message : 'The pinned tiles could not be read.';
  }
}

function formatBytes(bytes: number): string {
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KiB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MiB`;
}

function plural(count: number, noun: string): string {
  return `${count} ${noun}${count === 1 ? '' : 's'}`;
}
