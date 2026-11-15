import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';
import { loadFirecal } from '../../wasm/load.ts';
import { pinnedSeries } from './pinned-series.ts';

const wasmPath = fileURLToPath(new URL('../../target/wasm32-unknown-unknown/debug/firecal_wasm.wasm', import.meta.url));
const firecal = await loadFirecal(readFileSync(wasmPath));

// Tile row 7, column 18 covers 10 to 20 north and 0 to 10 east. One cell, its south-west corner, has 10 plus the
// year since 2000 in every month of January 2000 to February 2003, sigma 1.
function fixtureTile() {
  const months = 38;
  const cells = 1600;
  const bytes = new Uint8Array(16 + 3 * months * cells * 2);
  const view = new DataView(bytes.buffer);
  bytes.set([0x46, 0x43, 0x54, 0x31]);
  view.setUint16(4, months, true);
  view.setUint8(8, 3);
  for (let month = 0; month < months; month += 1) {
    const at = month * cells + 39 * 40;
    view.setUint16(16 + at * 2, 10 + Math.floor(month / 12), true);
    view.setUint16(16 + (months * cells + at) * 2, 1, true);
  }
  return bytes;
}

function pinned(tiles) {
  return { open: async () => ({ match: async (path) => (tiles[path] ? new Response(tiles[path]) : undefined) }) };
}

test('a pinned region answers from the core, and a polygon over the same cell answers the same', async () => {
  const storage = pinned({ '/tiles/t_7_18.bin': fixtureTile() });
  const area = { west: 0, south: 10, east: 0.25, north: 10.25 };
  const square = [[10, 0], [10, 0.25], [10.25, 0.25], [10.25, 0]];

  const rectangle = await pinnedSeries(firecal, { area, plane: 'count' }, storage);
  const polygon = await pinnedSeries(firecal, { area, polygon: square, plane: 'count' }, storage);

  assert.equal(rectangle.tiles, 1);
  assert.equal(rectangle.series.value[36], 13, 'January 2003 is 10 + 3');
  assert.equal(rectangle.series.median[0], 11, 'the January median of 2000, 2001 and 2002: 10, 11, 12');
  assert.deepEqual(polygon.series.value, rectangle.series.value);
});

test('an area with no pinned tile has no answer, not a zero series', async () => {
  assert.equal(await pinnedSeries(firecal, { area: { west: 0, south: 10, east: 1, north: 11 }, plane: 'count' }, pinned({})), null);
});
