import assert from 'node:assert/strict';
import { test } from 'node:test';
import stubSeries from '../stub/series.json' with { type: 'json' };
import { parseSeries } from './index.ts';

test('the hand-made Series fixture keeps its monthly values and area', () => {
  const series = parseSeries(stubSeries);

  assert.equal(series.first_month_index, 276);
  assert.equal(series.months.length, 24);
  assert.equal(series.value[8], 700);
  assert.deepEqual(series.area, { west: -62, south: -20, east: -58, north: -16 });
});

test('a Series whose monthly arrays disagree is rejected', () => {
  const invalid = structuredClone(stubSeries);
  invalid.sigma.pop();

  assert.throws(() => parseSeries(invalid), /sigma.*months/i);
});

test('a Series with a gap in its month indexes is rejected', () => {
  const invalid = structuredClone(stubSeries);
  invalid.months[1] += 1;

  assert.throws(() => parseSeries(invalid), /consecutive/i);
});

test('provisional months start where the Series says, and a missing or null start means none', () => {
  assert.equal(parseSeries(stubSeries).provisional_from, 298);
  const none = structuredClone(stubSeries);
  none.provisional_from = null;
  assert.equal(parseSeries(none).provisional_from, null);
  delete none.provisional_from;
  assert.equal(parseSeries(none).provisional_from, null);
});

test('a provisional start outside the months is rejected', () => {
  const invalid = structuredClone(stubSeries);
  invalid.provisional_from = 400;
  assert.throws(() => parseSeries(invalid), /provisional_from/);
});
