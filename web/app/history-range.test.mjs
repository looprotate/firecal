import assert from 'node:assert/strict';
import { test } from 'node:test';
import { historyRange } from './history-range.ts';

const tile = (firstMonth, months) => ({ firstMonth, months });

test('the range is the months every tile holds, and the season is the year of the last one', () => {
  // Month 0 is January 2000; month 37 is February 2003.
  assert.deepEqual(historyRange([tile(0, 38), tile(0, 38)]), {
    firstMonth: 0,
    months: 38,
    baselineFirst: 2000,
    baselineLast: 2002,
    seasonYear: 2003,
    seasonMonths: 2,
  });
});

test('tiles that start and end apart share only their common months', () => {
  const range = historyRange([tile(0, 40), tile(12, 26)]);
  assert.equal(range.firstMonth, 12);
  assert.equal(range.months, 26);
});

test('a baseline year counts only when the range holds all of it', () => {
  // From November 2000 (month 10) to March 2003 (month 38): 2000 is partial, so the baseline is 2001 to 2002.
  const range = historyRange([tile(10, 29)]);
  assert.equal(range.baselineFirst, 2001);
  assert.equal(range.baselineLast, 2002);
  assert.equal(range.seasonMonths, 3);
});

test('a season ending in December is the whole year', () => {
  assert.equal(historyRange([tile(0, 36)]).seasonMonths, 12);
  assert.equal(historyRange([tile(0, 36)]).seasonYear, 2002);
});

test('no tiles, tiles with no months in common, or no complete year before the season are refused', () => {
  assert.throws(() => historyRange([]), /No tiles cover this area/);
  assert.throws(() => historyRange([tile(0, 12), tile(24, 12)]), /no months in common/);
  assert.throws(() => historyRange([tile(318, 2)]), /complete year/);
});
