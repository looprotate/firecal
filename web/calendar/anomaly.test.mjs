import assert from 'node:assert/strict';
import { test } from 'node:test';
import stubSeries from '../stub/series.json' with { type: 'json' };
import { buildGrid } from './grid.ts';
import { describeCell } from './labels.ts';
import { renderLegend } from './legend.ts';
import { renderCalendarSvg } from './render.ts';
import { describeSeasonMonth, renderStripSvg } from './strip.ts';

test('exactly the stub months the core flags as unusual carry the diamond: August to November 2024', () => {
  const svg = renderCalendarSvg(buildGrid(stubSeries), 'count');
  const diamonds = [...svg.matchAll(/<path class="unusual" aria-hidden="true" d="M([\d.]+) /g)].map((found) => Number(found[1]));
  // Diamonds sit at x = 40 + column * 34 + 22, top right of their cell.
  assert.deepEqual(diamonds.map((x) => (x - 62) / 34), [7, 8, 9, 10]);
});

test('an unusual month says so in its label, and a month that is not does not', () => {
  const grid = buildGrid(stubSeries);
  assert.match(describeCell(grid.rows[1]?.[8], 'count'), /, unusual$/);
  assert.doesNotMatch(describeCell(grid.rows[1]?.[6], 'count'), /unusual/);
});

test('a series without the comparison has no diamonds', () => {
  const svg = renderCalendarSvg(buildGrid({ plane: 'count', first_month_index: 0, value: [1], sigma: [1] }), 'count');
  assert.doesNotMatch(svg, /class="unusual"/);
});

test('the strip shows the twelve months of the season, flags the four unusual ones, and draws a sigma band for each', () => {
  const svg = renderStripSvg(buildGrid(stubSeries), 'count');
  assert.equal(svg.match(/class="anomaly-bar activity"/g)?.length, 12);
  assert.equal(svg.match(/class="sigma-band uncertainty"/g)?.length, 12);
  assert.equal(svg.match(/class="unusual"/g)?.length, 4);
  assert.match(svg, /aria-label="2024 against the median of past years"/);
  assert.ok(!/#[0-9a-f]{3,6}|fill=|stroke=/i.test(svg), 'colors come from the tokens');
});

test('a month of the strip is read with its median, its change and its estimated band at two sigma', () => {
  const grid = buildGrid(stubSeries);
  assert.equal(
    describeSeasonMonth(grid.rows[1]?.[8], 'count'),
    'September 2024: 1,400 hotspots against a median of 700, +100%, ±300 at two sigma (estimated). Unusual.',
  );
  assert.equal(
    describeSeasonMonth(grid.rows[1]?.[6], 'count'),
    'July 2024: 90 hotspots against a median of 60, +50%, ±30 at two sigma (estimated). Within the usual range.',
  );
});

test('the legend names the diamond in words', () => {
  assert.match(renderLegend(), />Unusual: outside the band and the past years</);
  assert.match(renderLegend(), /class="unusual"/);
});

test('a month with no sigma, as on the energy plane, is not called usual: the core cannot judge it', () => {
  const cell = { ...buildGrid(stubSeries).rows[1]?.[6], sigma: Number.NaN, unusual: false };
  assert.equal(
    describeSeasonMonth(cell, 'energy'),
    'July 2024: 90 MW against a median of 60, +50%. No uncertainty to judge it by.',
  );
});

test('a change of thousands of percent is written with its thousands separated', () => {
  const cell = { ...buildGrid(stubSeries).rows[1]?.[8], value: 7112, median: 7, anomaly: 1015, sigma: 88 };
  assert.match(describeSeasonMonth(cell, 'count'), /against a median of 7, \+101,500%, ±176 at two sigma/);
});
