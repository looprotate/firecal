// Loads firecal.wasm and wraps the series export over linear-memory buffers. The contract is firecal.d.ts.
// Plain TypeScript that Node can run without a build: no enums, no parameter properties.
import type { FirecalExports } from "./firecal.d.ts";

const REQUEST_BYTES = 56;
const ANALYSIS_REQUEST_BYTES = 80;
const TILE_ENTRY_BYTES = 16;
const VERTEX_BYTES = 16;
const ANALOGS_COUNT_BYTES = 4;
const ANALOG_BYTES = 12;
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

/** A series query over the whole history, with what the comparison and the analogs need. */
export interface AnalysisQuery extends SeriesQuery {
  /** First and last baseline year. */
  baselineFirst: number;
  baselineLast: number;
  /** The season in progress: January to month `seasonMonths` of `seasonYear`. */
  seasonYear: number;
  seasonMonths: number;
  /** Corners as [latitude, longitude]; when present the area is the polygon and the rectangle is ignored. */
  polygon?: [number, number][];
}

export interface ComparisonAnswer {
  /** A `STATUS_*` of firecal.d.ts; the arrays are empty unless it is `STATUS_OK` (0). */
  status: number;
  /** The median of each month's calendar month over the baseline; NaN when the baseline has none. */
  median: Float64Array;
  /** `(value - median) / max(median, 1)`; NaN when there is no median. */
  anomaly: Float64Array;
  /** 1 when the month clears the uncertainty band, else 0. */
  unusual: Uint8Array;
}

export interface Analog {
  year: number;
  distance: number;
}

export interface AnalogsAnswer {
  /** A `STATUS_*` of firecal.d.ts; `analogs` is empty unless it is `STATUS_OK` (0). */
  status: number;
  /** The nearest past years, nearest first: at most `count`. */
  analogs: Analog[];
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
  /** The series of a rectangle or polygon: the same answer as `series`, by the analysis request. */
  areaSeries(query: AnalysisQuery): SeriesAnswer;
  /** How each month compares with the baseline median. */
  comparison(query: AnalysisQuery): ComparisonAnswer;
  /** The `count` past years nearest to the season in progress (at most 64). */
  analogs(query: AnalysisQuery, count: number): AnalogsAnswer;
  /** The `series.csv` of the history, as bytes. The buffer grows until the text fits. */
  seriesCsv(query: AnalysisQuery): CellsCsvAnswer;
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
    fc_area_series: (request, length, buffer, room) => unsigned(raw.fc_area_series(request, length, buffer, room)),
    fc_comparison: (request, length, buffer, room) => unsigned(raw.fc_comparison(request, length, buffer, room)),
    fc_analogs: (request, length, buffer, room) => unsigned(raw.fc_analogs(request, length, buffer, room)),
    fc_series_csv: (request, length, buffer, room) => unsigned(raw.fc_series_csv(request, length, buffer, room)),
  };
  return {
    exports,
    series: (query) => runSeries(exports, query),
    cellsCsv: (query) => growingText(exports, query, (request, buffer, room) =>
      exports.fc_cells_csv(request, REQUEST_BYTES, buffer, room)),
    areaSeries: (query) => runAreaSeries(exports, query),
    comparison: (query) => runComparison(exports, query),
    analogs: (query, count) => runAnalogs(exports, query, count),
    seriesCsv: (query) => growingText(exports, query, (request, buffer, room) =>
      exports.fc_series_csv(request, ANALYSIS_REQUEST_BYTES, buffer, room)),
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
  query: SeriesQuery | AnalysisQuery,
  output: number,
): number {
  const analysis = "baselineFirst" in query ? query : undefined;
  const corners = analysis?.polygon ?? [];
  const tileAddresses = query.tiles.map((tile) => {
    const address = block(tile.bytes.length);
    new Uint8Array(exports.memory.buffer).set(tile.bytes, address);
    return address;
  });
  const table = block(query.tiles.length * TILE_ENTRY_BYTES);
  const vertices = block(corners.length * VERTEX_BYTES);
  const request = block(analysis ? ANALYSIS_REQUEST_BYTES : REQUEST_BYTES);
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
  corners.forEach(([latitude, longitude], at) => {
    view.setFloat64(vertices + at * VERTEX_BYTES, latitude, true);
    view.setFloat64(vertices + at * VERTEX_BYTES + 8, longitude, true);
  });
  if (analysis) {
    [analysis.baselineFirst, analysis.baselineLast, analysis.seasonYear, analysis.seasonMonths, corners.length, vertices].forEach(
      (word, at) => view.setUint32(request + 56 + at * 4, word, true),
    );
  }
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

/** Asks `call(request, buffer, room)` for text in a buffer that doubles until the text fits. */
function growingText(
  exports: FirecalExports,
  query: SeriesQuery | AnalysisQuery,
  call: (request: number, buffer: number, room: number) => number,
): CellsCsvAnswer {
  for (let room = FIRST_CSV_BUFFER_BYTES; room <= LARGEST_CSV_BUFFER_BYTES; room *= 2) {
    const answer = withBlocks(exports, (block) => {
      const buffer = block(4 + room);
      const request = layOutRequest(exports, block, query, 0);
      const status = call(request, buffer, 4 + room);
      if (status !== 0) return { status, text: new Uint8Array(0) };
      const length = new DataView(exports.memory.buffer).getUint32(buffer, true);
      return { status, text: new Uint8Array(exports.memory.buffer, buffer + 4, length).slice() };
    });
    if (answer.status !== STATUS_OUTPUT_TOO_SMALL) return answer;
  }
  return { status: STATUS_OUTPUT_TOO_SMALL, text: new Uint8Array(0) };
}

function runAreaSeries(exports: FirecalExports, query: AnalysisQuery): SeriesAnswer {
  return withBlocks(exports, (block) => {
    const room = query.months * 16;
    const buffer = block(room);
    const request = layOutRequest(exports, block, query, 0);
    const status = exports.fc_area_series(request, ANALYSIS_REQUEST_BYTES, buffer, room);
    const view = new DataView(exports.memory.buffer);
    const value = new Float64Array(status === 0 ? query.months : 0);
    const sigma = new Float64Array(status === 0 ? query.months : 0);
    for (let month = 0; month < value.length; month += 1) {
      value[month] = view.getFloat64(buffer + month * 8, true);
      sigma[month] = view.getFloat64(buffer + (query.months + month) * 8, true);
    }
    return { status, value, sigma };
  });
}

function runComparison(exports: FirecalExports, query: AnalysisQuery): ComparisonAnswer {
  return withBlocks(exports, (block) => {
    const room = query.months * 17;
    const buffer = block(room);
    const request = layOutRequest(exports, block, query, 0);
    const status = exports.fc_comparison(request, ANALYSIS_REQUEST_BYTES, buffer, room);
    const months = status === 0 ? query.months : 0;
    const view = new DataView(exports.memory.buffer);
    const median = new Float64Array(months);
    const anomaly = new Float64Array(months);
    for (let month = 0; month < months; month += 1) {
      median[month] = view.getFloat64(buffer + month * 8, true);
      anomaly[month] = view.getFloat64(buffer + (months + month) * 8, true);
    }
    const unusual = new Uint8Array(exports.memory.buffer, buffer + months * 16, months).slice();
    return { status, median, anomaly, unusual };
  });
}

function runAnalogs(exports: FirecalExports, query: AnalysisQuery, count: number): AnalogsAnswer {
  return withBlocks(exports, (block) => {
    const room = ANALOGS_COUNT_BYTES + count * ANALOG_BYTES;
    const buffer = block(room);
    const request = layOutRequest(exports, block, query, 0);
    const status = exports.fc_analogs(request, ANALYSIS_REQUEST_BYTES, buffer, room);
    const analogs: Analog[] = [];
    if (status === 0) {
      const view = new DataView(exports.memory.buffer);
      const found = view.getUint32(buffer, true);
      for (let at = 0; at < found; at += 1) {
        const slot = buffer + ANALOGS_COUNT_BYTES + at * ANALOG_BYTES;
        analogs.push({ year: view.getUint32(slot, true), distance: view.getFloat64(slot + 4, true) });
      }
    }
    return { status, analogs };
  });
}
