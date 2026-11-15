import assert from 'node:assert/strict';
import { test } from 'node:test';
import { renderToolbar, toolbarParts } from './toolbar.ts';

test('with both actions the toolbar has the two planes, the current one pressed, and both files', () => {
  const html = renderToolbar('energy', { planes: true, csv: true, cells: true });
  assert.match(html, /<button type="button" data-plane="count" aria-pressed="false">Hotspot count<\/button>/);
  assert.match(html, /<button type="button" data-plane="energy" aria-pressed="true">Radiative power<\/button>/);
  assert.match(html, /data-file="series"[^>]*>Monthly CSV ↓</);
  assert.match(html, /data-file="cells"[^>]*>Grid CSV ↓</);
});

test('a calendar with no actions has no toolbar, rather than buttons that do nothing', () => {
  assert.equal(renderToolbar('count', { planes: false, csv: false, cells: false }), '');
});

test('a polygon has no cells.csv, and the toolbar says why', () => {
  const html = renderToolbar('count', { planes: true, csv: true, cells: false });
  assert.doesNotMatch(html, /data-file="cells"/);
  assert.match(html, /cells\.csv is for rectangles/);
});

test('the parts follow the actions the page gives', () => {
  const bytes = () => new Uint8Array();
  assert.deepEqual(toolbarParts({}), { planes: false, csv: false, cells: false });
  assert.deepEqual(toolbarParts({ choosePlane() {}, csv: { series: bytes } }), { planes: true, csv: true, cells: false });
  assert.deepEqual(toolbarParts({ csv: { series: bytes, cells: bytes } }), { planes: false, csv: true, cells: true });
});
