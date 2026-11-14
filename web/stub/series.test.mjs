import assert from 'node:assert/strict';
import { test } from 'node:test';
import stub from './series.json' with { type: 'json' };

const BASELINE_YEAR = 2023;
const yearOf = (month) => 2000 + Math.floor(month / 12);

test('the median of each calendar month is its value in the one baseline year, 2023', () => {
  stub.months.forEach((month, at) => {
    if (yearOf(month) === BASELINE_YEAR) assert.equal(stub.median[month % 12], stub.value[at]);
  });
});

test('a month is unusual only when it clears two sigma and lies outside the baseline range', () => {
  stub.months.forEach((month, at) => {
    const median = stub.median[month % 12];
    const clears = Math.abs(stub.value[at] - median) > 2 * stub.sigma[at];
    const outside = stub.value[at] !== median; // the range of one baseline year is its value
    const expected = yearOf(month) !== BASELINE_YEAR && clears && outside;
    assert.equal(stub.unusual[at], expected, `month ${month}`);
  });
});

test('the anomaly is (value - median) / max(median, 1), to six decimals', () => {
  stub.months.forEach((month, at) => {
    const median = stub.median[month % 12];
    assert.ok(Math.abs(stub.anomaly[at] - (stub.value[at] - median) / Math.max(median, 1)) < 5e-7, `month ${month}`);
  });
});

test('every analog year is a past year the series holds', () => {
  const years = new Set(stub.months.map(yearOf));
  const season = yearOf(stub.months.at(-1));
  for (const analog of stub.analogs) {
    assert.ok(years.has(analog.year) && analog.year < season, `${analog.year}`);
  }
});

test('provisional months start inside the series', () => {
  assert.ok(stub.months.includes(stub.provisional_from));
});
