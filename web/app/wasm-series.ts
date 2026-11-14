import type { AnalysisQuery, Firecal, TileFile } from '../../wasm/firecal.ts';
import type { Area, Series } from './series.ts';

/** The months, baseline and season of a query, and the tiles the area touches. */
export interface WasmHistoryOptions {
  firstMonth: number;
  months: number;
  baselineFirst: number;
  baselineLast: number;
  seasonYear: number;
  seasonMonths: number;
  tiles: TileFile[];
}

/** The most analog years the calendar asks the core for. */
const ANALOG_LIMIT = 64;

/** Runs the history, comparison and analog queries of the core for an area and returns one Series. */
export function seriesForArea(firecal: Firecal, area: Area, options: WasmHistoryOptions): Series {
  const query: AnalysisQuery = {
    south: area.south,
    north: area.north,
    west: area.west,
    east: area.east,
    firstMonth: options.firstMonth,
    months: options.months,
    plane: 0,
    tiles: options.tiles,
    baselineFirst: options.baselineFirst,
    baselineLast: options.baselineLast,
    seasonYear: options.seasonYear,
    seasonMonths: options.seasonMonths,
  };
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
    plane: 'count',
    first_month_index: options.firstMonth,
    months: Array.from({ length: options.months }, (_, at) => options.firstMonth + at),
    value: [...values.value],
    sigma: [...values.sigma],
    median,
    anomaly: [...comparison.anomaly],
    unusual: [...comparison.unusual].map((flag) => flag !== 0),
    analogs: analogs.analogs,
  };
}

function assertOk(status: number, operation: string): void {
  if (status !== 0) throw new Error(`Firecal ${operation} failed with status ${status}`);
}
