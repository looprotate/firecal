import assert from 'node:assert/strict';
import { test } from 'node:test';
import stubSeries from '../stub/series.json' with { type: 'json' };
import { describeAnalog, renderAnalogs } from './analogs.ts';
import { buildGrid } from './grid.ts';
import { renderCalendarSvg } from './render.ts';

const halfYear = {
  plane: 'count',
  first_month_index: 288, // January 2024 to June 2025
  value: [5, 6, 7, 8, 9, 10, 40, 60, 80, 30, 20, 10, 4, 5, 6, 7, 8, 9],
  sigma: Array.from({ length: 18 }, () => 1),
  analogs: [{ year: 2024, distance: 2 }],
};

test('the stub lists its one analog, 2023, with its distance, nearest first', () => {
  const html = renderAnalogs(stubSeries.analogs, buildGrid(stubSeries), 'count', null);
  assert.match(html, /<button type="button" data-year="2023" aria-pressed="false">2023<\/button>/);
  assert.match(html, /distance 12/);
  assert.match(html, /Years most like 2024/);
});

test('a chosen analog is pressed and its row is outlined in the calendar, once', () => {
  const grid = buildGrid(stubSeries);
  assert.match(renderAnalogs(stubSeries.analogs, grid, 'count', 2023), /data-year="2023" aria-pressed="true"/);
  const svg = renderCalendarSvg(grid, 'count', 2023);
  const rings = [...svg.matchAll(/<rect class="analog-ring" aria-hidden="true" x="([\d.]+)" y="([\d.]+)"/g)];
  assert.equal(rings.length, 1);
  assert.equal(Number(rings[0][2]), 20 - 3, 'around the first row, 2023');
  assert.doesNotMatch(renderCalendarSvg(grid, 'count', null), /analog-ring/);
});

test('for a season in progress, an analog says what happened in the months after it', () => {
  const grid = buildGrid(halfYear);
  assert.equal(
    describeAnalog({ year: 2024, distance: 2 }, grid, 'count'),
    '2024, distance 2. Then July 40, August 60, September 80, October 30, November 20, December 10 hotspots.',
  );
});

test('when the season is the whole year there is no "then" to show', () => {
  assert.equal(describeAnalog({ year: 2023, distance: 12 }, buildGrid(stubSeries), 'count'), '2023, distance 12.');
});

test('no analogs, no section', () => {
  assert.equal(renderAnalogs([], buildGrid(stubSeries), 'count', null), '');
});
