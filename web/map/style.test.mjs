import assert from 'node:assert/strict';
import { test } from 'node:test';
import { MAP_COLOR_TOKENS, SELECTION, mapStyle } from './style.ts';

const colors = { land: '#000001', water: '#000002', border: '#000003', region: '#000004', label: '#000005', selection: '#000006', forest: '#000007', park: '#000008', farmland: '#000009', road: '#00000a', highway: '#00000b' };

test('every color in the basemap is one of the token colors it was given', () => {
  const style = JSON.stringify(mapStyle(colors));
  const used = new Set(style.match(/#[0-9a-f]{6}/gi));
  assert.deepEqual([...used].sort(), Object.values(colors).sort());
});

test('each map color is read from a token', () => {
  assert.deepEqual(Object.keys(MAP_COLOR_TOKENS).sort(), Object.keys(colors).sort());
  for (const name of Object.values(MAP_COLOR_TOKENS)) assert.match(name, /^--[a-z-]+$/);
});

test('the selection is drawn last, over everything, in its own source', () => {
  const style = mapStyle(colors);
  assert.equal(style.layers.at(-1)?.id, SELECTION);
  assert.ok(SELECTION in style.sources);
});

test('the style is written here, so the map draws its paper with no network', () => {
  const style = mapStyle(colors);
  assert.equal(style.layers[0]?.type, 'background');
});

 test('the selection fill starts transparent and uses the same geometry as the outline', () => {
  const style = mapStyle(colors);
  const fill = style.layers.find((layer) => layer.id === 'firecal-activity-fill');
  assert.equal(fill.source, SELECTION);
  assert.equal(fill.paint['fill-opacity'], 0);
  assert.ok(style.layers.findIndex((layer) => layer.id === fill.id) < style.layers.findIndex((layer) => layer.id === SELECTION));
});
