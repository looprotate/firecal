import test from 'node:test';
import assert from 'node:assert/strict';
import { monthlyShade } from './map-activity.ts';
const series = { first_month_index: 238, value: [0, 100], plane: 'count' };
test('the most recent real month uses the darkest calendar step', () => {
  assert.deepEqual(monthlyShade(series), { monthIndex: 239, value: 100, step: 7 });
});
test('zero activity is distinct from missing data', () => {
  assert.deepEqual(monthlyShade(series, 238), { monthIndex: 238, value: 0, step: 0 });
  assert.equal(monthlyShade(null), null);
  assert.equal(monthlyShade(series, 237), null);
  assert.equal(monthlyShade(series, 240), null);
});
test('month selection uses the same square-root presentation scale as the calendar', () => {
  assert.deepEqual(monthlyShade({ ...series, value: [25, 100] }, 238), { monthIndex: 238, value: 25, step: 4 });
});
