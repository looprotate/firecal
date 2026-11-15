import type { Area } from './series.ts';

/** The Cache Storage cache that holds the tiles of pinned regions; the service worker reads it too. */
export const REGION_CACHE = 'firecal-regions-v1';

/** Cells per tile and planes per tile, from the FCT1 layout in docs/data-format.md. */
const TILE_CELLS = 1600;
const TILE_PLANES = 3;
const TILE_HEADER_BYTES = 16;

/** A 10-degree tile by its row and column in the 18 x 36 grid, and the path it is served at. */
export interface RegionTile {
  row: number;
  col: number;
  path: string;
}

/** What pinning a region stored: tiles found, their bytes, and tiles the published set does not have. */
export interface PinReport {
  tiles: number;
  bytes: number;
  missing: number;
}

/** A tile's bytes, pinned or read from the host, with the months its header says it holds. */
export interface CachedTile extends RegionTile {
  bytes: Uint8Array;
  firstMonth: number;
  months: number;
}

/** Lists the 10-degree tile files with positive-area overlap with an area. */
export function regionTiles(area: Area): RegionTile[] {
  const firstRow = Math.max(0, Math.floor((90 - area.north) / 10));
  const lastRow = Math.min(17, Math.ceil((90 - area.south) / 10) - 1);
  const firstCol = Math.max(0, Math.floor((area.west + 180) / 10));
  const lastCol = Math.min(35, Math.ceil((area.east + 180) / 10) - 1);
  const tiles: RegionTile[] = [];
  for (let row = firstRow; row <= lastRow; row += 1) {
    for (let col = firstCol; col <= lastCol; col += 1) {
      tiles.push({ row, col, path: `/tiles/t_${row}_${col}.bin` });
    }
  }
  return tiles;
}

/**
 * Caches every published tile overlapping an area and reports the stored byte cost. A tile the host does not
 * have (a 404, or a page served in its place) is counted as missing: by the tile contract it holds no fire.
 */
export async function pinRegion(
  area: Area,
  storage: Pick<CacheStorage, 'open'> = globalThis.caches,
  fetchTile: typeof fetch = fetch,
): Promise<PinReport> {
  const cache = await storage.open(REGION_CACHE);
  const report: PinReport = { tiles: 0, bytes: 0, missing: 0 };
  for (const tile of regionTiles(area)) {
    const cached = await cache.match(tile.path);
    const response = cached ?? (await fetchTile(tile.path));
    if (!cached && response.status !== 404 && !response.ok) {
      throw new Error(`Could not pin ${tile.path}: HTTP ${response.status}`);
    }
    const bytes = response.ok ? new Uint8Array(await response.arrayBuffer()) : new Uint8Array();
    if (tileMonths(bytes) === null) {
      report.missing += 1;
      continue;
    }
    if (!cached) await cache.put(tile.path, new Response(bytes));
    report.bytes += bytes.byteLength;
    report.tiles += 1;
  }
  return report;
}

/** The months an FCT1 tile holds, from its header, or `null` when the bytes are not a whole three-plane tile. */
export function tileMonths(bytes: Uint8Array): { firstMonth: number; months: number } | null {
  if (bytes.length < TILE_HEADER_BYTES || bytes[0] !== 0x46 || bytes[1] !== 0x43 || bytes[2] !== 0x54 || bytes[3] !== 0x31) {
    return null;
  }
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const months = view.getUint16(4, true);
  const firstMonth = view.getUint16(6, true);
  const whole = TILE_HEADER_BYTES + months * TILE_CELLS * 2 * TILE_PLANES;
  if (months === 0 || bytes[8] !== TILE_PLANES || bytes.length !== whole) return null;
  return { firstMonth, months };
}

/** The tiles of an area this device can read, and how many of them are pinned. */
export interface RegionTiles {
  tiles: CachedTile[];
  pinned: number;
}

/**
 * Reads every tile that overlaps an area: a pinned tile from the cache, any other from the host, without storing
 * it. A tile the host does not have, or cannot send because there is no network, is left out.
 */
export async function readRegionTiles(
  area: Area,
  storage: Pick<CacheStorage, 'open'> = globalThis.caches,
  fetchTile: typeof fetch = fetch,
): Promise<RegionTiles> {
  const cache = await storage.open(REGION_CACHE);
  const result: RegionTiles = { tiles: [], pinned: 0 };
  for (const tile of regionTiles(area)) {
    const cached = await cache.match(tile.path);
    if (cached) {
      const bytes = new Uint8Array(await cached.arrayBuffer());
      const held = tileMonths(bytes);
      if (held === null) throw new Error(`Cached tile ${tile.path} is not a valid FCT1 tile.`);
      result.tiles.push({ ...tile, bytes, ...held });
      result.pinned += 1;
      continue;
    }
    const response = await fetchTile(tile.path).catch(() => null);
    if (!response?.ok) continue;
    const bytes = new Uint8Array(await response.arrayBuffer());
    const held = tileMonths(bytes);
    if (held !== null) result.tiles.push({ ...tile, bytes, ...held });
  }
  return result;
}
