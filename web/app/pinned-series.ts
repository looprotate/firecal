import { resolvePeriod, windowSeries, type MonthRange } from './period.ts';
import type { AnalysisQuery, Firecal } from '../../wasm/load.ts';
import { historyRange } from './history-range.ts';
import { readPinnedRegion } from './offline.ts';
import type { Area, Series } from './series.ts';
import { analysisQuery, seriesForArea } from './wasm-series.ts';

/** What to ask about: an area, the polygon inside it if one was drawn, and the plane. */
export interface PinnedQuestion {
  area: Area;
  polygon?: [number, number][];
  plane: Series['plane'];
  period?: MonthRange;
  baselineFirst?: number;
}

/** The core's answer from the tiles pinned on this device, how many tiles it read, and the query it ran. */
export interface PinnedAnswer {
  series: Series;
  tiles: number;
  query: AnalysisQuery;
  available: MonthRange;
  period: MonthRange;
  adjusted: boolean;
}

/**
 * Asks the core about an area using only pinned tiles: no network. `null` when no pinned tile covers the area;
 * throws, with a message for the page, when the tiles cannot answer.
 */
export async function pinnedSeries(
  firecal: Firecal,
  question: PinnedQuestion,
  storage: Pick<CacheStorage, 'open'> = globalThis.caches,
): Promise<PinnedAnswer | null> {
  const cached = await readPinnedRegion(question.area, storage);
  if (cached.length === 0) return null;
  const history = historyRange(cached);
  const available = { first: history.firstMonth, last: history.firstMonth + history.months - 1 };
  const period = resolvePeriod(question.period, available.first, available.last);
  const year = 2000 + Math.floor(period.last / 12);
  const baselineFirst = Math.max(history.baselineFirst, question.baselineFirst ?? history.baselineFirst);
  const hasBaseline = baselineFirst < year;
  const options = {
    ...history,
    months: period.last - history.firstMonth + 1,
    seasonYear: year,
    seasonMonths: period.last % 12 + 1,
    baselineFirst: hasBaseline ? baselineFirst : year + 1,
    baselineLast: hasBaseline ? year - 1 : year + 1,
    tiles: cached.map(({ row, col, bytes }) => ({ row, col, bytes })),
    plane: question.plane,
    ...(question.polygon === undefined ? {} : { polygon: question.polygon }),
  };
  return {
    series: windowSeries(seriesForArea(firecal, question.area, options), period),
    available, period, adjusted: period.adjusted,
    tiles: cached.length,
    query: analysisQuery(question.area, options),
  };
}
