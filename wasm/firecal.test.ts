// Loads firecal.wasm in Node, with no page, and asks it the series of a hand-made tile.
// Run by `cargo xtask check` after it builds the module; FIRECAL_WASM overrides the module's path.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { loadFirecal } from "./load.ts";
import type { AnalysisQuery, SeriesQuery, TileFile } from "./load.ts";

const INVALID = 4294967295;
const wasmPath =
  process.env["FIRECAL_WASM"] ??
  fileURLToPath(new URL("../target/wasm32-unknown-unknown/debug/firecal_wasm.wasm", import.meta.url));
const firecal = await loadFirecal(readFileSync(wasmPath));

// Grid cell row 319, column 720 (latitude 10.0 to 10.25, longitude 0.0 to 0.25) is tile (7, 18) at
// position (39, 0); its east neighbour is position (39, 1). Two months from index 10 (November 2000).
//                    count     sigma     energy
//                    m0  m1    m0  m1    m0  m1
//  position (39,0)    5   7     3  12    40  50
//  position (39,1)    3   0     4   5     2   0
function fixtureTile(): Uint8Array {
  const cells = 1600;
  const planes = [new Uint16Array(2 * cells), new Uint16Array(2 * cells), new Uint16Array(2 * cells)];
  const put = (position: number, month: number, values: number[]): void =>
    values.forEach((value, plane) => {
      const target = planes[plane];
      if (target) target[month * cells + position] = value;
    });
  put(39 * 40, 0, [5, 3, 40]);
  put(39 * 40, 1, [7, 12, 50]);
  put(39 * 40 + 1, 0, [3, 4, 2]);
  put(39 * 40 + 1, 1, [0, 5, 0]);
  const bytes = new Uint8Array(16 + 3 * 2 * cells * 2);
  const view = new DataView(bytes.buffer);
  bytes.set([0x46, 0x43, 0x54, 0x31]); // FCT1
  view.setUint16(4, 2, true); // two months
  view.setUint16(6, 10, true); // first month index
  view.setUint8(8, 3); // three planes
  planes.forEach((plane, at) =>
    plane.forEach((value, cell) => view.setUint16(16 + (at * 2 * cells + cell) * 2, value, true)),
  );
  return bytes;
}

const tile: TileFile = { row: 7, col: 18, bytes: fixtureTile() };

function query(changes: Partial<SeriesQuery>): SeriesQuery {
  return {
    south: 10.0, north: 10.25, west: 0.0, east: 0.25,
    firstMonth: 10, months: 2, plane: 0, tiles: [tile],
    ...changes,
  };
}

test("one cell gives its count and sigma for each month", () => {
  const answer = firecal.series(query({}));
  assert.equal(answer.status, 0);
  assert.deepEqual([...answer.value], [5, 7]);
  assert.deepEqual([...answer.sigma], [3, 12]);
});

test("two cells add counts and add sigmas in quadrature", () => {
  const answer = firecal.series(query({ south: 10.1, north: 10.2, west: 0.1, east: 0.3 }));
  assert.deepEqual([...answer.value], [8, 7]);
  assert.deepEqual([...answer.sigma], [5, 13]); // sqrt(3^2 + 4^2), sqrt(12^2 + 5^2)
});

test("energy gives the energy and NaN sigmas", () => {
  const answer = firecal.series(query({ plane: 1, south: 10.1, north: 10.2, west: 0.1, east: 0.3 }));
  assert.deepEqual([...answer.value], [42, 50]);
  assert.ok([...answer.sigma].every(Number.isNaN));
});

test("a tile that is not in the table is zeros", () => {
  const answer = firecal.series(query({ tiles: [] }));
  assert.deepEqual([...answer.value], [0, 0]);
});

test("a question that cannot be answered returns its status", () => {
  assert.equal(firecal.series(query({ north: 10.0 })).status, 1); // empty area
  assert.equal(firecal.series(query({ north: 95 })).status, 2); // latitude beyond the pole
  assert.equal(firecal.series(query({ months: 0 })).status, 3); // no months
  assert.equal(firecal.series(query({ months: 3 })).status, 5); // month the tile lacks
  assert.equal(firecal.series(query({ tiles: [{ row: 7, col: 18, bytes: new Uint8Array(40) }] })).status, 4);
});

test("memory is given back: allocating again after a series still works", () => {
  for (let round = 0; round < 50; round += 1) firecal.series(query({}));
  assert.equal(firecal.series(query({})).status, 0);
});

test("the month exports answer through the same module", () => {
  assert.equal(firecal.exports.fc_month_index(2000, 11), 10);
  assert.equal(firecal.exports.fc_month_index(2020, 13), INVALID);
  assert.equal(firecal.exports.fc_month_span(10, 21), 12);
});

test("allocation refuses nothing, too much, and what it does not hold", () => {
  assert.equal(firecal.exports.fc_alloc(0), INVALID);
  assert.equal(firecal.exports.fc_alloc(2 ** 31), INVALID);
  assert.equal(firecal.exports.fc_free(123456), INVALID);
  const address = firecal.exports.fc_alloc(64);
  assert.notEqual(address, INVALID);
  assert.equal(firecal.exports.fc_free(address), 0);
  assert.equal(firecal.exports.fc_free(address), INVALID);
});

// A request block written by hand, so a field can be made wrong.
function rawRequest(words: Partial<Record<"tileCount" | "table" | "output" | "months" | "plane", number>>): number {
  const address = firecal.exports.fc_alloc(56);
  const view = new DataView(firecal.exports.memory.buffer);
  [0, 1, 0, 1].forEach((degrees, at) => view.setFloat64(address + at * 8, degrees, true));
  const fields = { first: 10, months: 1, plane: 0, tileCount: 0, table: 0, output: 0, ...words };
  [fields.first, fields.months, fields.plane, fields.tileCount, fields.table, fields.output].forEach((word, at) =>
    view.setUint32(address + 32 + at * 4, word, true),
  );
  return address;
}

test("addresses and lengths that cannot be read return INVALID and never trap", () => {
  const past = 2 ** 31;
  assert.equal(firecal.exports.fc_series(0, 56), INVALID);
  assert.equal(firecal.exports.fc_series(past, 56), INVALID);
  assert.equal(firecal.exports.fc_series(INVALID, 56), INVALID);
  const request = rawRequest({});
  assert.equal(firecal.exports.fc_series(request, 55), INVALID);
  assert.equal(firecal.exports.fc_series(request, INVALID), INVALID);
  assert.equal(firecal.exports.fc_series(rawRequest({ table: past, tileCount: 1 }), 56), INVALID);
  assert.equal(firecal.exports.fc_series(rawRequest({ tileCount: INVALID }), 56), INVALID);
  assert.equal(firecal.exports.fc_series(rawRequest({ output: past }), 56), INVALID);
  assert.equal(firecal.exports.fc_series(rawRequest({ months: INVALID }), 56), INVALID);
  assert.equal(firecal.exports.fc_series(rawRequest({ plane: 9 }), 56), INVALID);
});

test("a thousand random requests never trap", () => {
  let state = 0x2545f491n; // xorshift64, the same generator as firecal-testkit
  const next = (): number => {
    state ^= state >> 12n;
    state = (state ^ (state << 25n)) & 0xffffffffffffffffn;
    state ^= state >> 27n;
    return Number((state * 0x2545f4914f6cdd1dn) & 0xffffffffn);
  };
  const seed = 0x2545f491;
  const scratch = firecal.exports.fc_alloc(1600); // a request that succeeds writes here, never over the heap
  for (let round = 0; round < 1000; round += 1) {
    const output = next() % 2 === 0 ? scratch : 0x80000000 + next() / 2;
    const words = { tileCount: next() % 4, table: next(), output, months: next() % 100, plane: next() % 3 };
    const request = rawRequest(words);
    const status = firecal.exports.fc_series(request, 56);
    assert.equal(typeof status, "number", `seed ${seed}, round ${round}`);
    firecal.exports.fc_free(request);
  }
});

test("cells.csv is the header and a row per cell and month", () => {
  const answer = firecal.cellsCsv(query({ west: 0.0, east: 0.5 }));
  assert.equal(answer.status, 0);
  assert.equal(
    new TextDecoder().decode(answer.text),
    "year,month,cell_lat,cell_lon,value,sigma\n" +
      "2000,11,10.125,0.125,5,3\n" +
      "2000,11,10.125,0.375,3,4\n" +
      "2000,12,10.125,0.125,7,12\n" +
      "2000,12,10.125,0.375,0,5\n",
  );
});

test("cells.csv for the energy leaves the sigma empty", () => {
  const text = new TextDecoder().decode(firecal.cellsCsv(query({ plane: 1, months: 1 })).text);
  assert.equal(text, "year,month,cell_lat,cell_lon,value,sigma\n2000,11,10.125,0.125,40,\n");
});

test("cells.csv reports its status and refuses a buffer it cannot read", () => {
  assert.equal(firecal.cellsCsv(query({ north: 10.0 })).status, 1);
  const request = rawRequest({});
  assert.equal(firecal.exports.fc_cells_csv(request, 56, 0, 64), INVALID);
  assert.equal(firecal.exports.fc_cells_csv(request, 56, 2 ** 31, 64), INVALID);
  const buffer = firecal.exports.fc_alloc(8);
  assert.equal(firecal.exports.fc_cells_csv(request, 56, buffer, 3), INVALID);
  assert.equal(firecal.exports.fc_cells_csv(request, 55, buffer, 8), INVALID);
  assert.equal(firecal.exports.fc_cells_csv(request, 56, buffer, 8), 6, "four bytes of room cannot hold the header");
});

// A history of three years in tile (7, 18), January 2001 (index 12) to December 2003, for the cell at position
// (39, 0): row 319, column 720. Counts, with 100 in every month not listed, and sigma 2 throughout:
//
//          Jan  Feb
//   2001    10   20
//   2002    30   40
//   2003     5   22
//
// The cell east of it holds 1 every month with sigma 0. Medians over 2001 to 2002: January 20, February 30.
function historyTile(): Uint8Array {
  const cells = 1600;
  const count = new Uint16Array(36 * cells);
  const sigma = new Uint16Array(36 * cells);
  const listed = [[10, 20], [30, 40], [5, 22]];
  for (let month = 0; month < 36; month += 1) {
    count[month * cells + 39 * 40] = listed[Math.floor(month / 12)]?.[month % 12] ?? 100;
    count[month * cells + 39 * 40 + 1] = 1;
    sigma[month * cells + 39 * 40] = 2;
  }
  const bytes = new Uint8Array(16 + 3 * 36 * cells * 2);
  const view = new DataView(bytes.buffer);
  bytes.set([0x46, 0x43, 0x54, 0x31]);
  view.setUint16(4, 36, true);
  view.setUint16(6, 12, true);
  view.setUint8(8, 3);
  [count, sigma, new Uint16Array(36 * cells)].forEach((plane, at) =>
    plane.forEach((value, cell) => view.setUint16(16 + (at * 36 * cells + cell) * 2, value, true)),
  );
  return bytes;
}

function history(changes: Partial<AnalysisQuery>): AnalysisQuery {
  return {
    south: 10.0, north: 10.25, west: 0.0, east: 0.25,
    firstMonth: 12, months: 36, plane: 0, tiles: [{ row: 7, col: 18, bytes: historyTile() }],
    baselineFirst: 2001, baselineLast: 2002, seasonYear: 2003, seasonMonths: 2,
    ...changes,
  };
}

test("the area series of a rectangle and of a polygon over the same cell agree", () => {
  const byRectangle = firecal.areaSeries(history({}));
  const byPolygon = firecal.areaSeries(history({ polygon: [[10.0, 0.0], [10.25, 0.0], [10.25, 0.25], [10.0, 0.25]] }));
  assert.equal(byRectangle.status, 0);
  assert.deepEqual([...byRectangle.value.slice(0, 3)], [10, 20, 100]);
  assert.deepEqual([...byRectangle.sigma.slice(0, 2)], [2, 2]);
  assert.deepEqual(byPolygon, byRectangle);
});

test("a polygon selects the cells whose center it contains and refuses a bow tie", () => {
  const east = firecal.areaSeries(history({ polygon: [[10.0, 0.25], [10.25, 0.25], [10.25, 0.5], [10.0, 0.5]] }));
  assert.ok([...east.value].every((value) => value === 1));
  const bowTie = firecal.areaSeries(history({ polygon: [[10.0, 0.0], [10.25, 0.25], [10.25, 0.0], [10.0, 0.25]] }));
  assert.equal(bowTie.status, 9);
});

test("the comparison flags a month only when it clears the sensor band and the baseline range", () => {
  const answer = firecal.comparison(history({}));
  assert.equal(answer.status, 0);
  assert.deepEqual([...answer.median.slice(24, 27)], [20, 30, 100]);
  assert.equal(answer.anomaly[24], -0.75); // (5 - 20) / 20
  assert.equal(answer.anomaly[26], 0);
  // January 5: below the baseline range 10 to 30 and 15 from the median. February 22: 8 from the median, over
  // 2 * sigma = 4, but inside the range 20 to 40. March: the median.
  assert.deepEqual([...answer.unusual.slice(24, 27)], [1, 0, 0]);
});

test("a month with no baseline has no median and is not unusual", () => {
  const answer = firecal.comparison(history({ baselineFirst: 2010, baselineLast: 2011 }));
  assert.equal(answer.status, 0);
  assert.ok([...answer.median].every(Number.isNaN) && [...answer.anomaly].every(Number.isNaN));
  assert.ok([...answer.unusual].every((flag) => flag === 0));
});

test("the three analog years are the nearest past years, nearest first", () => {
  // Only two past years exist: 2001 at distance 2 and 2002 at distance 3.
  const answer = firecal.analogs(history({}), 3);
  assert.equal(answer.status, 0);
  assert.deepEqual(answer.analogs, [{ year: 2001, distance: 2 }, { year: 2002, distance: 3 }]);
  assert.deepEqual(firecal.analogs(history({}), 1).analogs, [{ year: 2001, distance: 2 }]);
});

test("analogs and the comparison report a bad baseline or season", () => {
  assert.equal(firecal.analogs(history({ seasonMonths: 13 }), 3).status, 8);
  assert.equal(firecal.analogs(history({ seasonYear: 2004 }), 3).status, 8);
  assert.equal(firecal.analogs(history({ baselineFirst: 2002, baselineLast: 2001 }), 3).status, 7);
  assert.equal(firecal.comparison(history({ baselineLast: 70000 })).status, 7);
});

test("series.csv is the series with its median, anomaly and flag", () => {
  const answer = firecal.seriesCsv(history({}));
  assert.equal(answer.status, 0);
  const lines = new TextDecoder().decode(answer.text).split("\n");
  assert.equal(lines[0], "year,month,value,sigma,median,anomaly,unusual");
  assert.equal(lines[25], "2003,1,5,2.000,20.0,-0.750,true");
  assert.equal(lines[27], "2003,3,100,2.000,100.0,0.000,false");
  assert.equal(lines.length, 38); // header, 36 rows, and the empty text after the last newline
});

test("the analysis exports refuse what they cannot read and never trap", () => {
  const past = 2 ** 31;
  const request = firecal.exports.fc_alloc(80);
  const buffer = firecal.exports.fc_alloc(64);
  for (const call of [
    firecal.exports.fc_area_series, firecal.exports.fc_comparison, firecal.exports.fc_analogs, firecal.exports.fc_series_csv,
  ]) {
    assert.equal(call(0, 80, buffer, 64), INVALID);
    assert.equal(call(past, 80, buffer, 64), INVALID);
    assert.equal(call(request, 79, buffer, 64), INVALID);
    assert.equal(call(request, INVALID, buffer, 64), INVALID);
  }
  // A zeroed block has an area with no extent.
  assert.equal(firecal.exports.fc_area_series(request, 80, buffer, 64), 1);
});
