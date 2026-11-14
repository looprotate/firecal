import type { Grid } from './grid.ts';

/** A cell's place in the grid: row (year) and column (month, 0 to 11). */
export interface Position {
  row: number;
  column: number;
}

function hasCell(grid: Grid, at: Position): boolean {
  return (grid.rows[at.row]?.[at.column] ?? null) !== null;
}

/**
 * Where a key moves the focus: arrows by one cell, Home and End to the first and last month of the row,
 * PageUp and PageDown by a year. A move onto a missing month, or off the grid, stays where it is.
 */
export function nextPosition(grid: Grid, from: Position, key: string): Position {
  const move = (row: number, column: number): Position => {
    const target = { row, column };
    return hasCell(grid, target) ? target : from;
  };
  const columns = (grid.rows[from.row] ?? []).flatMap((cell, at) => (cell === null ? [] : [at]));
  switch (key) {
    case 'ArrowLeft':
      return move(from.row, from.column - 1);
    case 'ArrowRight':
      return move(from.row, from.column + 1);
    case 'ArrowUp':
    case 'PageUp':
      return move(from.row - 1, from.column);
    case 'ArrowDown':
    case 'PageDown':
      return move(from.row + 1, from.column);
    case 'Home':
      return move(from.row, columns[0] ?? from.column);
    case 'End':
      return move(from.row, columns[columns.length - 1] ?? from.column);
    default:
      return from;
  }
}

/** The newest month of the grid: where the focus enters the calendar. */
export function latestPosition(grid: Grid): Position | null {
  for (let row = grid.rows.length - 1; row >= 0; row -= 1) {
    for (let column = 11; column >= 0; column -= 1) {
      if (hasCell(grid, { row, column })) return { row, column };
    }
  }
  return null;
}
