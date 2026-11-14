// Loads firecal.wasm in Node, with no page, and asks it the series of a hand-made tile.
// Run by `cargo xtask check` after it builds the module; FIRECAL_WASM overrides the module's path.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { loadFirecal } from "./firecal.ts";
import type { SeriesQuery, TileFile } from "./firecal.ts";

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
