import '@watergis/maplibre-gl-terradraw/dist/maplibre-gl-terradraw.css';
import 'maplibre-gl/dist/maplibre-gl.css';
import '../tokens/tokens.css';
import './style.css';
import sample from '../stub/series.json';
import { createAreaMap } from '../map/map.ts';
import {
  AREA_SELECTED_EVENT,
  areaFromSelectionEvent,
  emitAreaSelected,
  parseAreaBounds,
  type AreaBoundsText,
} from './area-input.ts';
import { parseSeries } from './series.ts';
import { renderShell } from './shell.ts';
import type { Area } from './series.ts';

const root = document.querySelector<HTMLDivElement>('#app');
if (!root) throw new Error('The app root is missing');

const series = parseSeries(sample);
const shell = renderShell(root, series);
const state: { area: Area; bounds: AreaBoundsText; error: string } = {
  area: series.area,
  bounds: boundsFromArea(series.area),
  error: '',
};
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
  state.area = area;
  state.bounds = boundsFromArea(area);
  state.error = '';
  render();
  map.fitBounds([[area.west, area.south], [area.east, area.north]], { padding: 36, maxZoom: 8 });
});

render();
void loadRuntime(shell.runtimeStatus);

function render(): void {
  shell.areaInputs.west.value = state.bounds.west;
  shell.areaInputs.south.value = state.bounds.south;
  shell.areaInputs.east.value = state.bounds.east;
  shell.areaInputs.north.value = state.bounds.north;
  shell.areaStatus.textContent = state.error;
  shell.mapBounds.textContent = `${state.area.west} / ${state.area.south} — ${state.area.east} / ${state.area.north}`;
  shell.mapCoordinates.textContent = `${state.area.west}° / ${state.area.south}°`;
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
    await WebAssembly.instantiate(await response.arrayBuffer(), {});
    status.textContent = 'Firecal core loaded · sample Series active';
  } catch (error) {
    status.textContent = error instanceof Error ? error.message : 'The Firecal core could not be loaded.';
  }
}
