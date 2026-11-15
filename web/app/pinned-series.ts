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
}

/** The core's answer from the tiles pinned on this device, how many tiles it read, and the query it ran. */
export interface PinnedAnswer {
  series: Series;
  tiles: number;
  query: AnalysisQuery;
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
  const options = {
    ...historyRange(cached),
    tiles: cached.map(({ row, col, bytes }) => ({ row, col, bytes })),
    plane: question.plane,
    ...(question.polygon === undefined ? {} : { polygon: question.polygon }),
  };
  return {
    series: seriesForArea(firecal, question.area, options),
    tiles: cached.length,
    query: analysisQuery(question.area, options),
  };
}
