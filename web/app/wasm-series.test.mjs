import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';
import { loadFirecal } from '../../wasm/firecal.ts';
import { seriesForArea } from './wasm-series.ts';

function fixtureTile() {
  const firstMonth = 0;
  const months = 38; // January 2000 through February 2003.
  const cells = 1600;
  const planes = Array.from({ length: 3 }, () => new Uint16Array(months * cells));
  for (let month = 0; month < months; month += 1) {
    const year = Math.floor(month / 12);
    const position = 39 * 40;
    planes[0][month * cells + position] = 10 + year;
    planes[1][month * cells + position] = 1;
    planes[2][month * cells + position] = 20 + year;
  }
  const bytes = new Uint8Array(16 + 3 * months * cells * 2);
  const view = new DataView(bytes.buffer);
  bytes.set([0x46, 0x43, 0x54, 0x31]);
  view.setUint16(4, months, true);
  view.setUint16(6, firstMonth, true);
  view.setUint8(8, 3);
  planes.forEach((plane, index) => plane.forEach((value, cell) => {
    view.setUint16(16 + (index * months * cells + cell) * 2, value, true);
  }));
  return { row: 7, col: 18, bytes };
}

const wasmPath = fileURLToPath(new URL('../../target/wasm32-unknown-unknown/debug/firecal_wasm.wasm', import.meta.url));
const firecal = await loadFirecal(readFileSync(wasmPath));

test('the selected area produces one Series from the wasm history, comparison and analogs', () => {
  const area = { west: 0, south: 10, east: 0.25, north: 10.25 };
  const series = seriesForArea(firecal, area, {
    firstMonth: 0,
    months: 38,
    baselineFirst: 2001,
    baselineLast: 2002,
    seasonYear: 2003,
    seasonMonths: 2,
    tiles: [fixtureTile()],
  });

  assert.deepEqual(series.area, area);
  assert.equal(series.first_month_index, 0);
  assert.equal(series.value[36], 13);
  assert.equal(series.value[37], 13);
  assert.equal(series.sigma[36], 1);
  assert.equal(series.median[0], 11.5);
  assert.equal(series.median[1], 11.5);
  assert.equal(series.analogs.length, 2);
  assert.deepEqual(series.months.slice(-2), [36, 37]);
});
