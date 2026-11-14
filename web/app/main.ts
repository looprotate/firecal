import '@watergis/maplibre-gl-terradraw/dist/maplibre-gl-terradraw.css';
import 'maplibre-gl/dist/maplibre-gl.css';
import '../tokens/tokens.css';
import './style.css';
import sample from '../stub/series.json';
import { createAreaMap } from '../map/map.ts';
import { parseSeries } from './series.ts';
import { renderShell } from './shell.ts';

const root = document.querySelector<HTMLDivElement>('#app');
if (!root) throw new Error('The app root is missing');

const series = parseSeries(sample);
const shell = renderShell(root, series);
const map = createAreaMap(shell.mapHost);
map.on('error', (event) => {
  shell.runtimeStatus.textContent = `Map error: ${event.error.message}`;
});

void loadRuntime(shell.runtimeStatus);

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
