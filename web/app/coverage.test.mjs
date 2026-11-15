import assert from 'node:assert/strict';
import { test } from 'node:test';
import { coverageEdges, covers, parseCoverage } from './coverage.ts';

const published = {
  region: 'Australia',
  tiles: ['t_12_32', 't_12_33', 't_13_32'],
  example: { name: 'south-east Australia', west: 147, south: -38, east: 151, north: -34 },
};

test('the coverage file gives the region, its tiles by row and column, and a named example area', () => {
  const coverage = parseCoverage(published);
  assert.equal(coverage.region, 'Australia');
  assert.deepEqual(coverage.tiles, [{ row: 12, col: 32 }, { row: 12, col: 33 }, { row: 13, col: 32 }]);
  assert.equal(coverage.example.name, 'south-east Australia');
  assert.equal(coverage.example.west, 147);
});

test('a coverage file that is not one says what is wrong', () => {
  assert.throws(() => parseCoverage(null), /not an object/);
  assert.throws(() => parseCoverage({ ...published, tiles: [] }), /no tiles/);
  assert.throws(() => parseCoverage({ ...published, tiles: ['t_18_0'] }), /not in the grid: t_18_0/);
  assert.throws(() => parseCoverage({ ...published, tiles: ['tile.bin'] }), /not in the grid/);
  assert.throws(() => parseCoverage({ ...published, example: { name: 'x', west: 'east' } }), /not a named area/);
});

test('an area is covered when it touches one published tile, and not when it touches none', () => {
  const coverage = parseCoverage(published);
  // Tile 12, 32 is 30 to 40 south, 140 to 150 east.
  assert.ok(covers(coverage, { west: 147, south: -38, east: 151, north: -34 }));
  assert.ok(covers(coverage, { west: 149, south: -45, east: 152, north: -41 }), 'only 13, 32');
  assert.ok(!covers(coverage, { west: 20.2, south: -30.2, east: 30.3, north: -24.1 }), 'South Africa');
  assert.ok(!covers(coverage, { west: 150.5, south: -48, east: 159, north: -41 }), 'only 13, 33, not published');
});

test('the outline is the edges no two tiles share: an L of three tiles has eight', () => {
  const edges = coverageEdges(parseCoverage(published).tiles);
  assert.equal(edges.length, 8);
  // The edge between 12, 32 and 12, 33 (150 east, 30 to 40 south) is shared, so it is not drawn.
  assert.ok(!edges.some(([[x0, y0], [x1, y1]]) => x0 === 150 && x1 === 150 && y0 === -40 && y1 === -30));
  assert.deepEqual(coverageEdges([{ row: 0, col: 0 }]).length, 4);
});
