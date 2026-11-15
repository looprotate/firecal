import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import stubSeries from '../stub/series.json' with { type: 'json' };
import { buildGrid } from './grid.ts';
import { describeCell } from './labels.ts';
import { renderLegend } from './legend.ts';
import { PROVISIONAL_SIZE } from './marks.ts';
import { renderCalendarSvg } from './render.ts';

const tokens = readFileSync(new URL('../tokens/tokens.css', import.meta.url), 'utf8');

test('the months from provisional_from on are provisional, the ones before are not', () => {
  const grid = buildGrid(stubSeries);
  assert.equal(grid.rows[1]?.[9]?.provisional, false, 'October 2024');
  assert.equal(grid.rows[1]?.[10]?.provisional, true, 'November 2024');
  assert.equal(grid.rows[1]?.[11]?.provisional, true, 'December 2024');
});

test('a series with no provisional start has no provisional month', () => {
  const grid = buildGrid({ plane: 'count', first_month_index: 0, value: [1, 2], sigma: [1, 1] });
  assert.ok(grid.rows[0]?.every((cell) => cell === null || !cell.provisional));
});

test('a provisional month says so in words, so it does not rely on the mark', () => {
  const cell = buildGrid(stubSeries).rows[1]?.[11];
  assert.equal(describeCell(cell, 'count'), 'December 2024: 260 hotspots, uncertainty ±35 (estimated), provisional');
});

test('each provisional month gets one triangle, drawn in the bottom left corner inside its cell', () => {
  const svg = renderCalendarSvg(buildGrid(stubSeries), 'count');
  const triangles = [...svg.matchAll(/<path class="provisional" [^>]*d="([^"]+)"/g)];
  assert.equal(triangles.length, 2);
  // December 2024: column 11, row 1. Cells are 28 apart by 34, from x 40 and y 20. The hand moves the corners of the
  // triangle by under a unit, so its box is its size within one.
  const numbers = triangles[1][1].match(/-?[\d.]+/g).map(Number);
  const xs = numbers.filter((_, at) => at % 2 === 0);
  const ys = numbers.filter((_, at) => at % 2 === 1);
  const [left, right, top, bottom] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  const cellX = 40 + 11 * 34;
  const cellY = 20 + 34;
  assert.ok(Math.abs(right - left - PROVISIONAL_SIZE) < 1 && Math.abs(bottom - top - PROVISIONAL_SIZE) < 1);
  assert.ok(left > cellX && bottom < cellY + 28 && left < cellX + 6, 'inside the cell, against its bottom left');
});

test('the triangle is the size the tokens give it', () => {
  assert.equal(`${PROVISIONAL_SIZE}px`, /--provisional-size:\s*(\d+px);/.exec(tokens)?.[1]);
});

test('the legend shows the empty cell, the seven steps in order, and the provisional triangle, in words too', () => {
  const svg = renderLegend();
  const steps = [...svg.matchAll(/class="swatch ramp-(\d)"/g)].map((found) => Number(found[1]));
  assert.deepEqual(steps, [0, 1, 2, 3, 4, 5, 6, 7]);
  assert.match(svg, /class="provisional"/);
  assert.match(svg, />None </);
  assert.match(svg, /> Most</);
  assert.match(svg, />Provisional: near-real-time data</);
  assert.ok(!/#[0-9a-f]{3,6}|fill=|stroke=/i.test(svg), 'colors come from the tokens');
});
