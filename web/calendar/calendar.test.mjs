import assert from 'node:assert/strict';
import { test } from 'node:test';
import stubSeries from '../stub/series.json' with { type: 'json' };
import { buildGrid, monthOfMonthIndex, yearOfMonthIndex } from './grid.ts';
import { latestPosition, nextPosition } from './keyboard.ts';
import { describeCell } from './labels.ts';
import { rampStep } from './ramp.ts';
import { renderCalendarSvg } from './render.ts';

test('month index 276 is January 2023 and 299 is December 2024', () => {
  assert.equal(yearOfMonthIndex(276), 2023);
  assert.equal(monthOfMonthIndex(276), 1);
  assert.equal(yearOfMonthIndex(299), 2024);
  assert.equal(monthOfMonthIndex(299), 12);
});

test('ramp step is 0 for no activity and 7 at the maximum', () => {
  assert.equal(rampStep(0, 100), 0);
  assert.equal(rampStep(100, 100), 7);
});

test('ramp step is never 0 for activity, however small', () => {
  assert.equal(rampStep(0.001, 1_000_000), 1);
});

test('ramp step of a quarter of the maximum is 4 on the square-root scale', () => {
  // sqrt(0.25) = 0.5; 7 * 0.5 = 3.5; rounded up is 4.
  assert.equal(rampStep(25, 100), 4);
});

test('ramp step never goes down as the value goes up', () => {
  let previous = 0;
  for (let value = 0; value <= 1000; value += 1) {
    const step = rampStep(value, 1000);
    assert.ok(step >= previous, `step fell at ${value}`);
    previous = step;
  }
});

test('ramp step with no maximum is 0, not NaN', () => {
  assert.equal(rampStep(5, 0), 0);
  assert.equal(rampStep(Number.NaN, 10), 0);
});

test('the stub fills two year rows of twelve months, 2023 above 2024', () => {
  const grid = buildGrid(stubSeries);
  assert.deepEqual(grid.years, [2023, 2024]);
  assert.equal(grid.rows.length, 2);
  assert.ok(grid.rows.every((row) => row.length === 12 && row.every((cell) => cell !== null)));
});

test('the stub places September 2023 (value 700) at row 0, column 8', () => {
  const cell = buildGrid(stubSeries).rows[0]?.[8];
  assert.equal(cell?.value, 700);
  assert.equal(cell?.sigma, 90);
  assert.equal(cell?.month, 9);
  assert.equal(cell?.year, 2023);
});

test('the stub peak, October 2024 at 1600, takes the top step and the quietest month the first', () => {
  const grid = buildGrid(stubSeries);
  assert.equal(grid.rows[1]?.[9]?.step, 7);
  assert.equal(grid.rows[1]?.[4]?.step, 1);
});

test('a series that starts in March leaves January and February empty', () => {
  const grid = buildGrid({ plane: 'count', first_month_index: 242, value: [5, 6], sigma: [1, 1] });
  assert.deepEqual(grid.years, [2020]);
  assert.equal(grid.rows[0]?.[0], null);
  assert.equal(grid.rows[0]?.[1], null);
  assert.equal(grid.rows[0]?.[2]?.value, 5);
});

test('an empty series makes an empty grid', () => {
  const grid = buildGrid({ plane: 'count', first_month_index: 0, value: [], sigma: [] });
  assert.deepEqual(grid, { years: [], rows: [] });
  assert.equal(latestPosition(grid), null);
});

test('a series that crosses a year boundary spans both years', () => {
  const grid = buildGrid({ plane: 'count', first_month_index: 287, value: [1, 2], sigma: [0, 0] });
  assert.deepEqual(grid.years, [2023, 2024]);
  assert.equal(grid.rows[0]?.[11]?.value, 1);
  assert.equal(grid.rows[1]?.[0]?.value, 2);
});

test('a count cell is read as hotspots with an estimated uncertainty', () => {
  const cell = buildGrid(stubSeries).rows[0]?.[8];
  assert.equal(describeCell(cell, 'count'), 'September 2023: 700 hotspots, uncertainty ±90 (estimated)');
});

test('an energy cell says megawatts and drops a NaN sigma', () => {
  const cell = { monthIndex: 0, year: 2000, month: 1, value: 12400, sigma: Number.NaN, step: 3 };
  assert.equal(describeCell(cell, 'energy'), 'January 2000: 12,400 MW of fire radiative power');
});

test('arrow keys move one cell and stop at the edge of the grid', () => {
  const grid = buildGrid(stubSeries);
  assert.deepEqual(nextPosition(grid, { row: 0, column: 3 }, 'ArrowRight'), { row: 0, column: 4 });
  assert.deepEqual(nextPosition(grid, { row: 0, column: 3 }, 'ArrowLeft'), { row: 0, column: 2 });
  assert.deepEqual(nextPosition(grid, { row: 0, column: 3 }, 'ArrowDown'), { row: 1, column: 3 });
  assert.deepEqual(nextPosition(grid, { row: 0, column: 0 }, 'ArrowLeft'), { row: 0, column: 0 });
  assert.deepEqual(nextPosition(grid, { row: 0, column: 5 }, 'ArrowUp'), { row: 0, column: 5 });
  assert.deepEqual(nextPosition(grid, { row: 1, column: 11 }, 'ArrowRight'), { row: 1, column: 11 });
});

test('Home and End go to the first and last month of the row', () => {
  const grid = buildGrid({ plane: 'count', first_month_index: 242, value: [5, 6, 7], sigma: [1, 1, 1] });
  assert.deepEqual(nextPosition(grid, { row: 0, column: 3 }, 'Home'), { row: 0, column: 2 });
  assert.deepEqual(nextPosition(grid, { row: 0, column: 2 }, 'End'), { row: 0, column: 4 });
});

test('the focus does not step onto a month the series lacks', () => {
  const grid = buildGrid({ plane: 'count', first_month_index: 242, value: [5, 6], sigma: [1, 1] });
  assert.deepEqual(nextPosition(grid, { row: 0, column: 2 }, 'ArrowLeft'), { row: 0, column: 2 });
});

test('the focus enters the calendar at the newest month', () => {
  assert.deepEqual(latestPosition(buildGrid(stubSeries)), { row: 1, column: 11 });
});

test('the SVG has 24 focusable cells for the stub and exactly one is in the tab order', () => {
  const svg = renderCalendarSvg(buildGrid(stubSeries), 'count');
  assert.equal(svg.match(/role="gridcell"/g)?.length, 24);
  assert.equal(svg.match(/tabindex="0"/g)?.length, 1);
  assert.equal(svg.match(/tabindex="-1"/g)?.length, 23);
});

test('every cell of the SVG has a label and a ramp class and no literal color', () => {
  const svg = renderCalendarSvg(buildGrid(stubSeries), 'count');
  assert.equal(svg.match(/aria-label="[A-Z][a-z]+ 20\d\d: [\d,]+ hotspots/g)?.length, 24);
  assert.ok(svg.includes('class="cell ramp-7"'));
  assert.ok(!/#[0-9a-f]{3,6}|fill=|stroke=/i.test(svg));
});

test('a missing month is drawn as an absent cell that cannot take focus', () => {
  const grid = buildGrid({ plane: 'count', first_month_index: 242, value: [5], sigma: [1] });
  const svg = renderCalendarSvg(grid, 'count');
  assert.equal(svg.match(/class="cell absent"/g)?.length, 11);
  assert.equal(svg.match(/tabindex/g)?.length, 1);
});

test('the SVG carries one hidden focus ring, drawn after the cells so it paints over them', () => {
  const svg = renderCalendarSvg(buildGrid(stubSeries), 'count');
  assert.equal(svg.match(/class="focus-ring"/g)?.length, 1);
  assert.ok(svg.indexOf('class="focus-ring"') > svg.lastIndexOf('role="gridcell"'));
  assert.ok(svg.includes('class="focus-ring" visibility="hidden"'));
});
