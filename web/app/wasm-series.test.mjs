import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';
import { loadFirecal } from '../../wasm/load.ts';
import { analysisQuery, csvFiles, seriesForArea } from './wasm-series.ts';

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

  const polygonSeries = seriesForArea(firecal, area, {
    firstMonth: 0,
    months: 38,
    baselineFirst: 2001,
    baselineLast: 2002,
    seasonYear: 2003,
    seasonMonths: 2,
    tiles: [fixtureTile()],
    polygon: [[10, 0], [10, 0.25], [10.25, 0.25], [10.25, 0]],
  });
  assert.deepEqual(polygonSeries.value, series.value);
});

const options = {
  firstMonth: 0, months: 38, baselineFirst: 2001, baselineLast: 2002, seasonYear: 2003, seasonMonths: 2,
};

test('switching to energy changes the plane the core reads and nothing else', () => {
  const area = { west: 0, south: 10, east: 0.25, north: 10.25 };
  const tiles = [fixtureTile()];
  const count = analysisQuery(area, { ...options, tiles, plane: 'count' });
  const energy = analysisQuery(area, { ...options, tiles, plane: 'energy' });
  assert.equal(count.plane, 0);
  assert.equal(energy.plane, 1);
  assert.deepEqual({ ...energy, plane: 0 }, count);
});

test('the energy series is the energy plane, in its own unit and without a sigma', () => {
  const area = { west: 0, south: 10, east: 0.25, north: 10.25 };
  const series = seriesForArea(firecal, area, { ...options, tiles: [fixtureTile()], plane: 'energy' });
  assert.equal(series.plane, 'energy');
  assert.equal(series.value[36], 23, 'January 2003 is 20 + 3 megawatts');
  assert.ok(Number.isNaN(series.sigma[36]));
});

test('the CSV files are the core text of the same query, with the headers of docs/data-format.md', () => {
  const area = { west: 0, south: 10, east: 0.25, north: 10.25 };
  const query = analysisQuery(area, { ...options, tiles: [fixtureTile()], plane: 'count' });
  const files = csvFiles(firecal, query, false);
  const decode = (bytes) => new TextDecoder().decode(bytes);
  assert.equal(decode(files.series()).split('\n')[0], 'year,month,value,sigma,median,anomaly,unusual');
  assert.equal(decode(files.cells()).split('\n')[0], 'year,month,cell_lat,cell_lon,value,sigma');
  assert.equal(csvFiles(firecal, query, true).cells, undefined, 'a polygon has no cells.csv');
});

test('the core is asked for the three nearest analog years, nearest first', () => {
  const months = 86; // January 2000 to February 2007: seven complete baseline years.
  const cells = 1600;
  const bytes = new Uint8Array(16 + 3 * months * cells * 2);
  const view = new DataView(bytes.buffer);
  bytes.set([0x46, 0x43, 0x54, 0x31]);
  view.setUint16(4, months, true);
  view.setUint8(8, 3);
  for (let month = 0; month < months; month += 1) {
    const at = month * cells + 39 * 40;
    view.setUint16(16 + at * 2, 10 + ((month * 7) % 13), true);
    view.setUint16(16 + (months * cells + at) * 2, 1, true);
  }
  const series = seriesForArea(firecal, { west: 0, south: 10, east: 0.25, north: 10.25 }, {
    firstMonth: 0, months, baselineFirst: 2000, baselineLast: 2006, seasonYear: 2007, seasonMonths: 2,
    tiles: [{ row: 7, col: 18, bytes }],
  });
  assert.equal(series.analogs.length, 3);
  assert.ok(series.analogs[0].distance <= series.analogs[1].distance && series.analogs[1].distance <= series.analogs[2].distance);
});
