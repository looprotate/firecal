import { areaCoverage, type Coverage } from './coverage.ts';
import { pinRegion, readPinnedRegion } from './offline.ts';
import { pinnedSeries, type PinnedQuestion, type PinnedAnswer } from './pinned-series.ts';
import type { Firecal } from '../../wasm/load.ts';

/** Only complete published coverage can produce a series; missing areas do not become zeros. */
export async function queryRegion(core: Firecal, coverage: Coverage, question: PinnedQuestion): Promise<PinnedAnswer | null> {
  const known = areaCoverage(question.area, coverage);
  if (!known) return null;
  let cached = await readPinnedRegion(question.area);
  if (cached.length !== known.tiles) {
    if (!navigator.onLine) throw new Error('This region is not saved on this device. Connect once to download it.');
    const report = await pinRegion(question.area);
    if (report.missing > 0) throw new Error('The region download is incomplete. Please try again.');
    cached = await readPinnedRegion(question.area);
  }
  if (cached.length !== known.tiles) throw new Error('The region download is incomplete. Please try again.');
  return pinnedSeries(core, question);
}
