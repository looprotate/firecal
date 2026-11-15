import { regionTiles } from './offline.ts';
import type { Area } from './series.ts';
export interface CoverageTile { row: number; col: number; firstMonth: number; months: number }
export interface Coverage { version: number; tiles: CoverageTile[] }
export function parseCoverage(value: unknown): Coverage {
  if (!value || typeof value !== 'object' || !('tiles' in value) || !Array.isArray(value.tiles)) throw new Error('The coverage catalog is unavailable.');
  const tiles = value.tiles as CoverageTile[];
  for (const tile of tiles) if (![tile.row, tile.col, tile.firstMonth, tile.months].every(Number.isInteger) || tile.row < 0 || tile.row >= 18 || tile.col < 0 || tile.col >= 36 || tile.firstMonth < 0 || tile.months <= 0) throw new Error('The coverage catalog is invalid.');
  if (new Set(tiles.map(t => `${t.row}/${t.col}`)).size !== tiles.length) throw new Error('The coverage catalog has duplicate tiles.');
  return { version: 1, tiles };
}
/** Unknown tiles are unknown, never zero-fire observations. */
export function areaCoverage(area: Area, coverage: Coverage): { first: number; last: number; tiles: number } | null {
  const needed = regionTiles(area);
  const found = needed.map(t => coverage.tiles.find(c => c.row === t.row && c.col === t.col));
  if (!needed.length || found.some(t => !t)) return null;
  const known = found as CoverageTile[];
  const first = Math.max(...known.map(t => t.firstMonth)), last = Math.min(...known.map(t => t.firstMonth + t.months - 1));
  return first <= last ? { first, last, tiles: needed.length } : null;
}
export async function loadCoverage(): Promise<Coverage> {
  const path = '/tiles/coverage.json';
  const cache = await caches.open('firecal-coverage-v1');
  try {
    const response = await fetch(path, { cache: 'no-cache' });
    if (!response.ok) throw new Error('Coverage could not be loaded.');
    const catalog = parseCoverage(await response.clone().json());
    await cache.put(path, response); return catalog;
  } catch (error) {
    const saved = await cache.match(path);
    if (saved) return parseCoverage(await saved.json());
    throw error;
  }
}
