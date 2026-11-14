import assert from 'node:assert/strict';
import test from 'node:test';
import {
  AREA_SELECTED_EVENT,
  areaFromSelectionEvent,
  areaFromRectangleCoordinates,
  emitAreaSelected,
  parseAreaBounds,
  polygonFromSelectionEvent,
  selectionFromPolygonCoordinates,
} from './area-input.ts';

test('coordinate entry parses signed decimal bounds into an Area', () => {
  assert.deepEqual(
    parseAreaBounds({ west: '-121.5', south: '-10', east: '-120', north: '5.25' }),
    { area: { west: -121.5, south: -10, east: -120, north: 5.25 }, error: null },
  );
});

test('coordinate entry refuses empty and nonnumeric bounds', () => {
  for (const fields of [
    { west: '', south: '-10', east: '-120', north: '5' },
    { west: 'west', south: '-10', east: '-120', north: '5' },
  ]) {
    assert.equal(parseAreaBounds(fields).area, null);
    assert.ok(parseAreaBounds(fields).error);
  }
});

test('coordinate entry refuses bounds outside the globe', () => {
  assert.equal(parseAreaBounds({ west: '-181', south: '-10', east: '-120', north: '5' }).area, null);
  assert.equal(parseAreaBounds({ west: '-121', south: '-91', east: '-120', north: '5' }).area, null);
  assert.equal(parseAreaBounds({ west: '-121', south: '-10', east: '181', north: '5' }).area, null);
  assert.equal(parseAreaBounds({ west: '-121', south: '-10', east: '-120', north: '91' }).area, null);
});

test('coordinate entry refuses empty, reversed and antimeridian-crossing bounds', () => {
  for (const fields of [
    { west: '10', south: '10', east: '10', north: '20' },
    { west: '20', south: '10', east: '10', north: '20' },
    { west: '10', south: '20', east: '20', north: '10' },
    { west: '170', south: '-10', east: '-170', north: '10' },
  ]) {
    assert.equal(parseAreaBounds(fields).area, null);
  }
});

test('a finished rectangle ring becomes its exact Area bounds', () => {
  const ring = [[-121, -10], [-120, -10], [-120, 5], [-121, 5], [-121, -10]];
  assert.deepEqual(areaFromRectangleCoordinates(ring), {
    west: -121,
    south: -10,
    east: -120,
    north: 5,
  });
});

test('typing the bounds and drawing them produce the same Area', () => {
  const typed = parseAreaBounds({ west: '-121', south: '-10', east: '-120', north: '5' });
  const drawn = areaFromRectangleCoordinates([
    [-121, -10], [-120, -10], [-120, 5], [-121, 5], [-121, -10],
  ]);
  assert.deepEqual(typed.area, drawn);
});

test('area selection emits one shared event with the validated Area', () => {
  const target = new EventTarget();
  const emitted = [];
  target.addEventListener(AREA_SELECTED_EVENT, (event) => emitted.push(event.detail));
  const area = { west: -121, south: -10, east: -120, north: 5 };

  emitAreaSelected(target, area);

  assert.deepEqual(emitted, [{ area }]);
  assert.deepEqual(areaFromSelectionEvent(new CustomEvent(AREA_SELECTED_EVENT, { detail: area })), area);
  assert.equal(areaFromSelectionEvent(new Event(AREA_SELECTED_EVENT)), null);
});

test('a finished polygon keeps its ring for the core and derives bounds for the map', () => {
  const selection = selectionFromPolygonCoordinates([[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]]);
  assert.deepEqual(selection, {
    area: { west: 0, south: 0, east: 1, north: 1 },
    polygon: [[0, 0], [0, 1], [1, 1], [1, 0]],
  });
  assert.equal(selectionFromPolygonCoordinates([[0, 0], [1, 1], [0, 0]]), null);
});

test('the shared area event preserves polygon corners in core order', () => {
  const target = new EventTarget();
  let selection;
  target.addEventListener(AREA_SELECTED_EVENT, (event) => { selection = event; });
  const input = selectionFromPolygonCoordinates([[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]]);
  emitAreaSelected(target, input);
  assert.deepEqual(areaFromSelectionEvent(selection), input.area);
  assert.deepEqual(polygonFromSelectionEvent(selection), input.polygon);
});

test('rectangle conversion refuses triangles and zero-area rings', () => {
  assert.equal(areaFromRectangleCoordinates([[-121, -10], [-120, -10], [-120, 5], [-121, -10]]), null);
  assert.equal(areaFromRectangleCoordinates([[-121, -10], [-121, -10], [-121, -10], [-121, -10], [-121, -10]]), null);
});
