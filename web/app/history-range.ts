/** The months a tile holds, from its FCT1 header. */
export interface TileMonths {
  firstMonth: number;
  months: number;
}

/** The months, baseline years and season in progress that one query over pinned tiles asks the core for. */
export interface HistoryRange {
  firstMonth: number;
  months: number;
  baselineFirst: number;
  baselineLast: number;
  seasonYear: number;
  seasonMonths: number;
}

const FIRST_YEAR = 2000;
/** The most baseline years `wasm/firecal.d.ts` accepts. */
const MOST_BASELINE_YEARS = 64;

/**
 * The months every pinned tile holds, the season in progress (January to the last month), and as baseline the
 * complete years before it. Throws, with a message for the page, when there is nothing to compare.
 */
export function historyRange(tiles: readonly TileMonths[]): HistoryRange {
  if (tiles.length === 0) throw new Error('No pinned tiles cover this area.');
  const firstMonth = Math.max(...tiles.map((tile) => tile.firstMonth));
  const lastMonth = Math.min(...tiles.map((tile) => tile.firstMonth + tile.months - 1));
  if (lastMonth < firstMonth) throw new Error('The pinned tiles have no months in common.');

  const seasonYear = FIRST_YEAR + Math.floor(lastMonth / 12);
  const baselineLast = seasonYear - 1;
  const baselineFirst = Math.max(FIRST_YEAR + Math.ceil(firstMonth / 12), seasonYear - MOST_BASELINE_YEARS);
  if (baselineFirst > baselineLast) {
    throw new Error('The pinned tiles need at least one complete year before the latest season.');
  }
  return {
    firstMonth,
    months: lastMonth - firstMonth + 1,
    baselineFirst,
    baselineLast,
    seasonYear,
    seasonMonths: (lastMonth % 12) + 1,
  };
}
