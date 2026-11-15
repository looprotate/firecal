import { CELL } from './layout.ts';
import { diamondPoints, sketch } from './sketch.ts';

/** The provisional triangle's sides, as `--provisional-size` in `web/tokens/tokens.css`. */
export const PROVISIONAL_SIZE = 10;
/** Half the diagonal of the unusual diamond. */
const DIAMOND = 4.5;
/** How far a mark sits in from the cell's edges, so it stays inside the rounded corner. */
const INSET = 2;
/** The hand's wobble on a mark: small, so the corner it sits in stays clear. */
const WOBBLE = 0.35;

const delay = (ms: number | undefined): string => (ms === undefined ? '' : ` style="--d:${ms}ms"`);

/** The provisional triangle in the bottom left corner of the cell at `x`, `y`, popping in `ms` after the cells. */
export function provisionalTriangle(x: number, y: number, seed = 0, ms?: number): string {
  const left = x + INSET;
  const bottom = y + CELL - INSET;
  const d = sketch([[left, bottom - PROVISIONAL_SIZE], [left, bottom], [left + PROVISIONAL_SIZE, bottom]], true, seed + 0.5, WOBBLE);
  return `<path class="provisional" aria-hidden="true"${delay(ms)} d="${d}"/>`;
}

/** The unusual diamond in the top right corner of the cell at `x`, `y`, popping in `ms` after the cells. */
export function unusualDiamond(x: number, y: number, seed = 0, ms?: number): string {
  return diamondAt(x + CELL - INSET - DIAMOND, y + INSET + DIAMOND, seed, ms);
}

/** A diamond centered on `cx`, `cy`: the same mark wherever a month is flagged. */
export function diamondAt(cx: number, cy: number, seed = 0, ms?: number): string {
  return `<path class="unusual" aria-hidden="true"${delay(ms)} d="${sketch(diamondPoints(cx, cy, DIAMOND), true, seed + 0.25, WOBBLE)}"/>`;
}
