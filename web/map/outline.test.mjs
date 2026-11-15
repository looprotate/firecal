import assert from 'node:assert/strict';
import { test } from 'node:test';
import { ring, selectionOutline } from './outline.ts';

function fakeMap() {
  const drawn = [];
  const framed = [];
  return {
    drawn,
    framed,
    getSource: () => ({ setData: (data) => drawn.push(data.geometry.coordinates[0]) }),
    fitBounds: (bounds) => framed.push(bounds),
  };
}

const sample = { area: { west: -62, south: -20, east: -58, north: -16 } };
const australia = { area: { west: 147, south: -38, east: 151, north: -34 } };

test('an area chosen while the map loads is the one shown when it has loaded, not the first one', () => {
  const map = fakeMap();
  const outline = selectionOutline(map);
  outline.show(sample);
  outline.show(australia);
  assert.deepEqual(map.framed, [], 'nothing is drawn before the map has loaded');
  outline.ready();
  assert.deepEqual(map.framed, [[[147, -38], [151, -34]]]);
  assert.equal(map.drawn.length, 1);
});

test('once the map has loaded, every new area is drawn and framed at once', () => {
  const map = fakeMap();
  const outline = selectionOutline(map);
  outline.ready();
  outline.show(sample);
  outline.show(australia);
  assert.equal(map.framed.length, 2);
  assert.deepEqual(map.framed[1], [[147, -38], [151, -34]]);
});

test('a rectangle is a closed ring, longitude first; a polygon keeps its corners', () => {
  assert.deepEqual(ring(australia), [[147, -38], [151, -38], [151, -34], [147, -34], [147, -38]]);
  assert.deepEqual(ring({ area: australia.area, polygon: [[-38, 148], [-36, 148], [-36, 150]] }), [[148, -38], [148, -36], [150, -36], [148, -38]]);
});
