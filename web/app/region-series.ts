import type { AnalysisQuery, Firecal } from '../../wasm/load.ts';
import { historyRange } from './history-range.ts';
import { readRegionTiles } from './offline.ts';
import type { Area, Series } from './series.ts';
import { analysisQuery, seriesForArea } from './wasm-series.ts';

/** What to ask about: an area, the polygon inside it if one was drawn, and the plane. */
export interface RegionQuestion {
  area: Area;
  polygon?: [number, number][];
  plane: Series['plane'];
}

/** The core's answer, how many tiles it read and how many of those are pinned, and the query it ran. */
export interface RegionAnswer {
  series: Series;
  tiles: number;
  pinned: number;
  query: AnalysisQuery;
}

/**
 * Asks the core about an area, on the pinned tiles and, for the rest, the tiles the host publishes. `null` when no
 * tile covers the area; throws, with a message for the page, when the tiles cannot answer.
 */
export async function regionSeries(
  firecal: Firecal,
  question: RegionQuestion,
  storage: Pick<CacheStorage, 'open'> = globalThis.caches,
  fetchTile: typeof fetch = fetch,
): Promise<RegionAnswer | null> {
  const { tiles, pinned } = await readRegionTiles(question.area, storage, fetchTile);
  if (tiles.length === 0) return null;
  const options = {
    ...historyRange(tiles),
    tiles: tiles.map(({ row, col, bytes }) => ({ row, col, bytes })),
    plane: question.plane,
    ...(question.polygon === undefined ? {} : { polygon: question.polygon }),
  };
  return {
    series: seriesForArea(firecal, question.area, options),
    tiles: tiles.length,
    pinned,
    query: analysisQuery(question.area, options),
  };
}
