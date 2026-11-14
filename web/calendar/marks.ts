import { CELL } from './layout.ts';

/** The provisional triangle's sides, as `--provisional-size` in `web/tokens/tokens.css`. */
export const PROVISIONAL_SIZE = 10;
/** Half the diagonal of the unusual diamond. */
const DIAMOND = 4;
/** How far a mark sits in from the cell's edges, so it stays inside the rounded corner. */
const INSET = 2;

/** The provisional triangle in the bottom left corner of the cell at `x`, `y`. */
export function provisionalTriangle(x: number, y: number): string {
  const left = x + INSET;
  const bottom = y + CELL - INSET;
  return `<path class="provisional" aria-hidden="true" d="M${left} ${bottom - PROVISIONAL_SIZE}V${bottom}H${left + PROVISIONAL_SIZE}Z"/>`;
}

/** The unusual diamond in the top right corner of the cell at `x`, `y`. */
export function unusualDiamond(x: number, y: number): string {
  return diamondAt(x + CELL - INSET - DIAMOND, y + INSET + DIAMOND);
}

/** A diamond centered on `cx`, `cy`: the same mark wherever a month is flagged. */
export function diamondAt(cx: number, cy: number): string {
  return `<path class="unusual" aria-hidden="true" d="M${cx} ${cy - DIAMOND}L${cx + DIAMOND} ${cy}L${cx} ${cy + DIAMOND}L${cx - DIAMOND} ${cy}Z"/>`;
}
