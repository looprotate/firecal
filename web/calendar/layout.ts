/** The side of a cell, in SVG units. */
export const CELL = 28;
/** The space between cells. */
export const GAP = 6;
/** Room for the year labels on the left. */
export const LEFT = 40;
/** Room for the month letters on top. */
export const TOP = 20;

/** The left edge of the cells of a month column, 0 to 11. */
export function columnX(column: number): number {
  return LEFT + column * (CELL + GAP);
}

/** The width of twelve columns and the year labels. */
export const GRID_WIDTH = LEFT + 12 * (CELL + GAP) - GAP;
