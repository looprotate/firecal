// Loads firecal.wasm and wraps the series export over linear-memory buffers. The contract is firecal.d.ts.
// Plain TypeScript that Node can run without a build: no enums, no parameter properties.
import type { FirecalExports } from "./firecal.d.ts";

const REQUEST_BYTES = 56;
const TILE_ENTRY_BYTES = 16;
const INVALID = 4294967295;

export interface TileFile {
  row: number;
  col: number;
  bytes: Uint8Array;
}

export interface SeriesQuery {
  south: number;
  north: number;
  west: number;
  east: number;
  firstMonth: number;
  months: number;
  /** 0 for the count with its sigma, 1 for the energy. */
  plane: number;
  tiles: TileFile[];
}

export interface SeriesAnswer {
  /** A `STATUS_*` of firecal.d.ts; `value` and `sigma` are empty unless it is `STATUS_OK` (0). */
  status: number;
  value: Float64Array;
  sigma: Float64Array;
}

export interface Firecal {
  exports: FirecalExports;
  series(query: SeriesQuery): SeriesAnswer;
}

export async function loadFirecal(wasm: BufferSource): Promise<Firecal> {
  const { instance } = await WebAssembly.instantiate(wasm, {});
  const raw = instance.exports as unknown as FirecalExports;
  // JavaScript reads a `u32` result of a wasm function as a signed number: `>>> 0` makes it unsigned again.
  const unsigned = (value: number): number => value >>> 0;
  const exports: FirecalExports = {
    memory: raw.memory,
    fc_month_index: (year, month) => unsigned(raw.fc_month_index(year, month)),
    fc_month_span: (first, last) => unsigned(raw.fc_month_span(first, last)),
    fc_alloc: (length) => unsigned(raw.fc_alloc(length)),
    fc_free: (address) => unsigned(raw.fc_free(address)),
    fc_series: (address, length) => unsigned(raw.fc_series(address, length)),
  };
  return { exports, series: (query) => runSeries(exports, query) };
}

function runSeries(exports: FirecalExports, query: SeriesQuery): SeriesAnswer {
  const blocks: number[] = [];
  const block = (length: number): number => {
    if (length === 0) return 0;
    const address = exports.fc_alloc(length);
    if (address === INVALID) throw new Error(`firecal.wasm cannot allocate ${length} bytes`);
    blocks.push(address);
    return address;
  };
  try {
    const bytes = (): Uint8Array => new Uint8Array(exports.memory.buffer);
    const tileAddresses = query.tiles.map((tile) => {
      const address = block(tile.bytes.length);
      bytes().set(tile.bytes, address);
      return address;
    });
    const table = block(query.tiles.length * TILE_ENTRY_BYTES);
    const output = block(query.months * 16);
    const request = block(REQUEST_BYTES);
    const view = new DataView(exports.memory.buffer);
    query.tiles.forEach((tile, at) => {
      const entry = table + at * TILE_ENTRY_BYTES;
      view.setUint32(entry, tile.row, true);
      view.setUint32(entry + 4, tile.col, true);
      view.setUint32(entry + 8, tileAddresses[at] ?? 0, true);
      view.setUint32(entry + 12, tile.bytes.length, true);
    });
    [query.south, query.north, query.west, query.east].forEach((degrees, at) =>
      view.setFloat64(request + at * 8, degrees, true),
    );
    [query.firstMonth, query.months, query.plane, query.tiles.length, table, output].forEach(
      (word, at) => view.setUint32(request + 32 + at * 4, word, true),
    );
    const status = exports.fc_series(request, REQUEST_BYTES);
    const value = new Float64Array(status === 0 ? query.months : 0);
    const sigma = new Float64Array(status === 0 ? query.months : 0);
    for (let month = 0; month < value.length; month += 1) {
      value[month] = view.getFloat64(output + month * 8, true);
      sigma[month] = view.getFloat64(output + (query.months + month) * 8, true);
    }
    return { status, value, sigma };
  } finally {
    for (const address of blocks) exports.fc_free(address);
  }
}
