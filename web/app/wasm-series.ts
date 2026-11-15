import type { AnalysisQuery, Firecal, TileFile } from '../../wasm/load.ts';
import type { Area, Series } from './series.ts';

/** The months, baseline and season of a query, the tiles the area touches, and the plane to read. */
export interface WasmHistoryOptions {
  firstMonth: number;
  months: number;
  baselineFirst: number;
  baselineLast: number;
  seasonYear: number;
  seasonMonths: number;
  tiles: TileFile[];
  /** Count (with its sigma) or energy; count when absent. */
  plane?: Series['plane'];
  /** Corners as [latitude, longitude]; when present the core answers for the polygon, not the rectangle. */
  polygon?: [number, number][];
}

/** The most analog years the calendar asks the core for. */
const ANALOG_LIMIT = 64;
/** `PLANE_COUNT` and `PLANE_ENERGY` of `wasm/firecal.d.ts`. */
const PLANE_NUMBER: Record<Series['plane'], number> = { count: 0, energy: 1 };

/** The one request the core answers for an area: its series, comparison, analogs and CSV all read it. */
export function analysisQuery(area: Area, options: WasmHistoryOptions): AnalysisQuery {
  return {
    south: area.south,
    north: area.north,
    west: area.west,
    east: area.east,
    firstMonth: options.firstMonth,
    months: options.months,
    plane: PLANE_NUMBER[options.plane ?? 'count'],
    tiles: options.tiles,
    baselineFirst: options.baselineFirst,
    baselineLast: options.baselineLast,
    seasonYear: options.seasonYear,
    seasonMonths: options.seasonMonths,
    ...(options.polygon === undefined ? {} : { polygon: options.polygon }),
  };
}

/** Runs the history, comparison and analog queries of the core for a rectangle or polygon; returns one Series. */
export function seriesForArea(firecal: Firecal, area: Area, options: WasmHistoryOptions): Series {
  const query = analysisQuery(area, options);
  const values = firecal.areaSeries(query);
  assertOk(values.status, 'area series');
  const comparison = firecal.comparison(query);
  assertOk(comparison.status, 'comparison');
  const analogs = firecal.analogs(query, ANALOG_LIMIT);
  assertOk(analogs.status, 'analogs');

  const median = Array.from({ length: 12 }, () => Number.NaN);
  comparison.median.forEach((value, at) => {
    median[(options.firstMonth + at) % 12] = value;
  });
  return {
    area,
    plane: options.plane ?? 'count',
    first_month_index: options.firstMonth,
    months: Array.from({ length: options.months }, (_, at) => options.firstMonth + at),
    value: [...values.value],
    sigma: [...values.sigma],
    median,
    anomaly: [...comparison.anomaly],
    unusual: [...comparison.unusual].map((flag) => flag !== 0),
    provisional_from: null,
    analogs: analogs.analogs,
  };
}

/**
 * The core's `series.csv` and `cells.csv` for a query, made when asked. `cells.csv` takes a rectangle, so a polygon
 * has none rather than the cells of its bounding box.
 */
export function csvFiles(
  firecal: Firecal,
  query: AnalysisQuery,
  polygon: boolean,
): { series(): Uint8Array; cells?(): Uint8Array } {
  const series = (): Uint8Array => {
    const answer = firecal.seriesCsv(query);
    assertOk(answer.status, 'series.csv');
    return answer.text;
  };
  if (polygon) return { series };
  return {
    series,
    cells: () => {
      const answer = firecal.cellsCsv(query);
      assertOk(answer.status, 'cells.csv');
      return answer.text;
    },
  };
}

function assertOk(status: number, operation: string): void {
  if (status !== 0) throw new Error(`Firecal ${operation} failed with status ${status}`);
}
