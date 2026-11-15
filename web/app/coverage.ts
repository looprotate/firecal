import type { Area } from './series.ts';

/** A tile of the 18 × 36 grid by its row and column. */
export interface TileCell {
  row: number;
  col: number;
}

/**
 * What the host publishes, from `tiles/coverage.json` (`docs/data-format.md`): the region's name, the tiles it has,
 * and an area worth opening first.
 */
export interface Coverage {
  region: string;
  tiles: TileCell[];
  example: Area & { name: string };
}

const TILE_NAME = /^t_(\d{1,2})_(\d{1,2})$/;

/** Reads `coverage.json`; throws, with a message for the page, when it is not one. */
export function parseCoverage(input: unknown): Coverage {
  if (typeof input !== 'object' || input === null) throw new Error('coverage.json is not an object');
  const { region, tiles, example } = input as Record<string, unknown>;
  if (typeof region !== 'string' || region === '') throw new Error('coverage.json has no region');
  if (!Array.isArray(tiles) || tiles.length === 0) throw new Error('coverage.json lists no tiles');
  const cells = tiles.map((name) => {
    const found = typeof name === 'string' ? TILE_NAME.exec(name) : null;
    const row = Number(found?.[1]);
    const col = Number(found?.[2]);
    if (!found || row > 17 || col > 35) throw new Error(`coverage.json has a tile that is not in the grid: ${String(name)}`);
    return { row, col };
  });
  if (typeof example !== 'object' || example === null) throw new Error('coverage.json has no example area');
  const { name, west, south, east, north } = example as Record<string, unknown>;
  const degrees = [west, south, east, north];
  if (typeof name !== 'string' || !degrees.every((value) => typeof value === 'number' && Number.isFinite(value))) {
    throw new Error('coverage.json has an example that is not a named area');
  }
  return { region, tiles: cells, example: { name, west: west as number, south: south as number, east: east as number, north: north as number } };
}

/** Whether any tile `area` touches is one the host has. */
export function covers(coverage: Coverage, area: Area): boolean {
  const firstRow = Math.floor((90 - area.north) / 10);
  const lastRow = Math.ceil((90 - area.south) / 10) - 1;
  const firstCol = Math.floor((area.west + 180) / 10);
  const lastCol = Math.ceil((area.east + 180) / 10) - 1;
  return coverage.tiles.some(({ row, col }) => row >= firstRow && row <= lastRow && col >= firstCol && col <= lastCol);
}

/**
 * The outline of the tiles, as the edges no two of them share, longitude first: one line around the region however
 * the tiles fall, for the map to draw.
 */
export function coverageEdges(tiles: readonly TileCell[]): [number, number][][] {
  const has = new Set(tiles.map(({ row, col }) => `${row},${col}`));
  const edges: [number, number][][] = [];
  for (const { row, col } of tiles) {
    const west = -180 + col * 10;
    const east = west + 10;
    const north = 90 - row * 10;
    const south = north - 10;
    if (!has.has(`${row - 1},${col}`)) edges.push([[west, north], [east, north]]);
    if (!has.has(`${row + 1},${col}`)) edges.push([[west, south], [east, south]]);
    if (!has.has(`${row},${col - 1}`)) edges.push([[west, south], [west, north]]);
    if (!has.has(`${row},${col + 1}`)) edges.push([[east, south], [east, north]]);
  }
  return edges;
}
