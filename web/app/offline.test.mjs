import assert from 'node:assert/strict';
import { test } from 'node:test';
import { pinRegion, readRegionTiles, regionTiles } from './offline.ts';

test('a region lists only the 10-degree tiles it overlaps', () => {
  assert.deepEqual(regionTiles({ west: 0, south: 10, east: 20, north: 20 }), [
    { row: 7, col: 18, path: '/tiles/t_7_18.bin' },
    { row: 7, col: 19, path: '/tiles/t_7_19.bin' },
  ]);
});

test('pinning reports cached bytes and skips tile files absent from the published set', async () => {
  const saved = new Map();
  const cache = {
    match: async (path) => saved.get(path),
    put: async (path, response) => { saved.set(path, response); },
  };
  const storage = { open: async () => cache };
  const report = await pinRegion(
    { west: 0, south: 10, east: 20, north: 20 },
    storage,
    async (path) => path.endsWith('18.bin') ? new Response(fixtureTile()) : new Response('', { status: 404 }),
  );
  assert.deepEqual(report, { tiles: 1, bytes: fixtureTile().byteLength, missing: 1 });
});

test('a host that answers a missing tile with a page instead of a 404 does not get the page pinned', async () => {
  const saved = new Map();
  const cache = {
    match: async (path) => saved.get(path),
    put: async (path, response) => { saved.set(path, response); },
  };
  const page = () => new Response('<!doctype html><title>firecal</title>', { headers: { 'Content-Type': 'text/html' } });
  const report = await pinRegion(
    { west: 0, south: -10, east: 20, north: 0 },
    { open: async () => cache },
    async (path) => path.endsWith('t_9_18.bin') ? new Response(fixtureTile()) : page(),
  );
  assert.deepEqual(report, { tiles: 1, bytes: fixtureTile().byteLength, missing: 1 });
  assert.deepEqual([...saved.keys()], ['/tiles/t_9_18.bin']);
});

const offline = async () => { throw new TypeError('Failed to fetch'); };

test('reading a region returns its pinned tiles first, validated, with their month span', async () => {
  const bytes = fixtureTile();
  const saved = new Map([['/tiles/t_9_18.bin', new Response(bytes)]]);
  const cache = { match: async (path) => saved.get(path)?.clone() };
  const { tiles, pinned } = await readRegionTiles({ west: 0, south: -10, east: 10, north: 0 }, { open: async () => cache }, offline);

  assert.equal(pinned, 1);
  assert.equal(tiles.length, 1);
  assert.equal(tiles[0].row, 9);
  assert.equal(tiles[0].col, 18);
  assert.equal(tiles[0].firstMonth, 12);
  assert.equal(tiles[0].months, 2);
  assert.deepEqual(tiles[0].bytes, bytes);
});

test('a tile that is not pinned is read from the host, and not stored', async () => {
  const cache = { match: async () => undefined, put: async () => assert.fail('reading must not pin') };
  const { tiles, pinned } = await readRegionTiles(
    { west: 0, south: -10, east: 10, north: 0 },
    { open: async () => cache },
    async (path) => path === '/tiles/t_9_18.bin' ? new Response(fixtureTile()) : new Response('', { status: 404 }),
  );
  assert.equal(pinned, 0);
  assert.deepEqual(tiles.map(({ row, col }) => [row, col]), [[9, 18]]);
});

test('with no network and nothing pinned, a region has no tiles', async () => {
  const cache = { match: async () => undefined };
  const { tiles } = await readRegionTiles({ west: 0, south: -10, east: 10, north: 0 }, { open: async () => cache }, offline);
  assert.deepEqual(tiles, []);
});

test('a page served in place of a missing tile is not read as a tile', async () => {
  const cache = { match: async () => undefined };
  const page = async () => new Response('<!doctype html><title>firecal</title>', { headers: { 'Content-Type': 'text/html' } });
  const { tiles } = await readRegionTiles({ west: 0, south: -10, east: 10, north: 0 }, { open: async () => cache }, page);
  assert.deepEqual(tiles, []);
});

test('reading a region rejects a malformed pinned tile', async () => {
  const cache = { match: async () => new Response(new Uint8Array([1, 2, 3])) };

  await assert.rejects(
    readRegionTiles({ west: 0, south: 10, east: 10, north: 20 }, { open: async () => cache }, offline),
    /not a valid FCT1 tile/,
  );
});

function fixtureTile() {
  const months = 2;
  const bytes = new Uint8Array(16 + months * 1600 * 2 * 3);
  const view = new DataView(bytes.buffer);
  bytes.set([0x46, 0x43, 0x54, 0x31]);
  view.setUint16(4, months, true);
  view.setUint16(6, 12, true);
  view.setUint8(8, 3);
  return bytes;
}
