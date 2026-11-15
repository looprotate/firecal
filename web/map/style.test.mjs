import assert from 'node:assert/strict';
import { test } from 'node:test';
import { MAP_COLOR_SOURCES, SELECTION, mapStyle, mixWithPaper } from './style.ts';

const colors = { land: '#000001', water: '#000002', border: '#000003', region: '#000004', label: '#000005', selection: '#000006', coverage: '#000007' };

test('every color in the basemap is one of the token colors it was given', () => {
  const style = JSON.stringify(mapStyle(colors));
  const used = new Set(style.match(/#[0-9a-f]{6}/gi));
  assert.deepEqual([...used].sort(), Object.values(colors).sort());
});

test('each map color is read from a token, mixed with the paper by a share between none and all', () => {
  assert.deepEqual(Object.keys(MAP_COLOR_SOURCES).sort(), Object.keys(colors).sort());
  for (const { token, share } of Object.values(MAP_COLOR_SOURCES)) {
    assert.match(token, /^--[a-z-]+$/);
    assert.ok(share > 0 && share <= 1);
  }
});

test('mixing with the paper is color-mix in srgb: per channel, rounded', () => {
  assert.equal(mixWithPaper('#000000', '#ffffff', 0.5), '#808080');
  assert.equal(mixWithPaper('#d77656', '#faf9f5', 1), '#d77656');
  // The site's --land: 40 % of --healthy #788c5d in the paper #faf9f5.
  assert.equal(mixWithPaper('#788c5d', '#faf9f5', 0.4), '#c6cdb8');
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
