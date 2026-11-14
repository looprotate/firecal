import type { Firecal } from '../../wasm/load.ts';
import { historyRange } from './history-range.ts';
import { readPinnedRegion } from './offline.ts';
import type { Area, Series } from './series.ts';
import { seriesForArea } from './wasm-series.ts';

/** The core's answer for an area from the tiles pinned on this device, and how many tiles it read. */
export interface PinnedAnswer {
  series: Series;
  tiles: number;
}

/**
 * Asks the core about `area` (or `polygon` inside it) using only pinned tiles: no network. `null` when no pinned
 * tile covers the area; throws, with a message for the page, when the tiles cannot answer.
 */
export async function pinnedSeries(
  firecal: Firecal,
  area: Area,
  polygon: [number, number][] | undefined,
  storage: Pick<CacheStorage, 'open'> = globalThis.caches,
): Promise<PinnedAnswer | null> {
  const cached = await readPinnedRegion(area, storage);
  if (cached.length === 0) return null;
  const range = historyRange(cached);
  const tiles = cached.map(({ row, col, bytes }) => ({ row, col, bytes }));
  const series = seriesForArea(firecal, area, { ...range, tiles, ...(polygon === undefined ? {} : { polygon }) });
  return { series, tiles: cached.length };
}
