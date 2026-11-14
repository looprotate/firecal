import type { Area, Series } from './series.ts';

interface TileFile {
  row: number;
  col: number;
  bytes: Uint8Array;
}

interface AnalysisQuery {
  south: number;
  north: number;
  west: number;
  east: number;
  firstMonth: number;
  months: number;
  plane: number;
  tiles: TileFile[];
  baselineFirst: number;
  baselineLast: number;
  seasonYear: number;
  seasonMonths: number;
}

interface Firecal {
  areaSeries(query: AnalysisQuery): { status: number; value: Float64Array; sigma: Float64Array };
  comparison(query: AnalysisQuery): {
    status: number;
    median: Float64Array;
    anomaly: Float64Array;
    unusual: Uint8Array;
  };
  analogs(query: AnalysisQuery, count: number): { status: number; analogs: Series['analogs'] };
}

export interface WasmHistoryOptions {
  firstMonth: number;
  months: number;
  baselineFirst: number;
  baselineLast: number;
  seasonYear: number;
  seasonMonths: number;
    tiles: TileFile[];
}

/** Runs the same history, comparison and analog queries for a newly selected rectangle. */
export function seriesForArea(
  firecal: Firecal,
  area: Area,
  options: WasmHistoryOptions,
): Series {
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
  const analogs = firecal.analogs(query, 64);
  assertOk(analogs.status, 'analogs');

  const median = Array.from({ length: 12 }, () => Number.NaN);
  for (let at = 0; at < comparison.median.length; at += 1) {
    const month = (options.firstMonth + at) % 12;
    median[month] = comparison.median[at] ?? Number.NaN;
  }
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
