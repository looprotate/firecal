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

export interface CellsCsvAnswer {
  /** A `STATUS_*` of firecal.d.ts; `text` is empty unless it is `STATUS_OK` (0). */
  status: number;
  text: Uint8Array;
}

export interface Firecal {
  exports: FirecalExports;
  series(query: SeriesQuery): SeriesAnswer;
  /** The `cells.csv` of the area, as bytes. The buffer grows until the text fits. */
  cellsCsv(query: SeriesQuery): CellsCsvAnswer;
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
    fc_cells_csv: (request, length, buffer, room) => unsigned(raw.fc_cells_csv(request, length, buffer, room)),
  };
  return {
    exports,
    series: (query) => runSeries(exports, query),
    cellsCsv: (query) => runCellsCsv(exports, query),
  };
}

/** Allocates blocks of linear memory and gives them all back when `use` ends. */
function withBlocks<T>(exports: FirecalExports, use: (block: (length: number) => number) => T): T {
  const blocks: number[] = [];
  try {
    return use((length) => {
      if (length === 0) return 0;
      const address = exports.fc_alloc(length);
      if (address === INVALID) throw new Error(`firecal.wasm cannot allocate ${length} bytes`);
      blocks.push(address);
      return address;
    });
  } finally {
    for (const address of blocks) exports.fc_free(address);
  }
}

/** Writes the tiles, the tile table and the request block, and returns the request's address. */
function layOutRequest(
  exports: FirecalExports,
  block: (length: number) => number,
  query: SeriesQuery,
  output: number,
): number {
  const tileAddresses = query.tiles.map((tile) => {
    const address = block(tile.bytes.length);
    new Uint8Array(exports.memory.buffer).set(tile.bytes, address);
    return address;
  });
  const table = block(query.tiles.length * TILE_ENTRY_BYTES);
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
  [query.firstMonth, query.months, query.plane, query.tiles.length, table, output].forEach((word, at) =>
    view.setUint32(request + 32 + at * 4, word, true),
  );
  return request;
}

function runSeries(exports: FirecalExports, query: SeriesQuery): SeriesAnswer {
  return withBlocks(exports, (block) => {
    const output = block(query.months * 16);
    const request = layOutRequest(exports, block, query, output);
    const status = exports.fc_series(request, REQUEST_BYTES);
    const view = new DataView(exports.memory.buffer);
    const value = new Float64Array(status === 0 ? query.months : 0);
    const sigma = new Float64Array(status === 0 ? query.months : 0);
    for (let month = 0; month < value.length; month += 1) {
      value[month] = view.getFloat64(output + month * 8, true);
      sigma[month] = view.getFloat64(output + (query.months + month) * 8, true);
    }
    return { status, value, sigma };
  });
}

const FIRST_CSV_BUFFER_BYTES = 1 << 20;
const STATUS_OUTPUT_TOO_SMALL = 6;
const LARGEST_CSV_BUFFER_BYTES = 1 << 29;

function runCellsCsv(exports: FirecalExports, query: SeriesQuery): CellsCsvAnswer {
  for (let room = FIRST_CSV_BUFFER_BYTES; room <= LARGEST_CSV_BUFFER_BYTES; room *= 2) {
    const answer = withBlocks(exports, (block) => {
      const buffer = block(4 + room);
      const request = layOutRequest(exports, block, query, 0);
      const status = exports.fc_cells_csv(request, REQUEST_BYTES, buffer, 4 + room);
      if (status !== 0) return { status, text: new Uint8Array(0) };
      const length = new DataView(exports.memory.buffer).getUint32(buffer, true);
      return { status, text: new Uint8Array(exports.memory.buffer, buffer + 4, length).slice() };
    });
    if (answer.status !== STATUS_OUTPUT_TOO_SMALL) return answer;
  }
  return { status: STATUS_OUTPUT_TOO_SMALL, text: new Uint8Array(0) };
}
