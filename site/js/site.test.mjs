import assert from 'node:assert/strict';
import { test } from 'node:test';
import { rampStep as appRampStep } from '../../web/calendar/ramp.ts';
import { demoArea, offlineArea, parseMonths, rampStep, season2019 } from './data.js';
import { archPoints, cut, hash, path, resample, rough } from './draw.js';

test('a resampled square keeps its outline, its points evenly spaced, each with its distance along it', () => {
  const points = resample([[0, 0], [100, 0], [100, 100], [0, 100]], true);
  const gaps = points.slice(1).map(([x, y], i) => Math.hypot(x - points[i][0], y - points[i][1]));
  assert.ok(gaps.every((gap) => Math.abs(gap - gaps[0]) < 1e-9 || gap < gaps[0]), 'corners cut no gap longer than the step');
  assert.equal(points[0][2], 0);
  assert.ok(points.at(-1)[2] < 400, 'a closed outline does not repeat its start');
});

test('a cut from 0 to 1 runs the whole line, and from a quarter to a half starts and ends there', () => {
  const line = [[0, 0], [100, 0]];
  assert.deepEqual(cut(line, 0, 1), [[0, 0], [100, 0]]);
  assert.deepEqual(cut(line, 0.25, 0.5), [[25, 0], [50, 0]]);
});

test('the wobble is the same for the same seed and boil frame, and the boil changes it', () => {
  const base = resample([[0, 0], [200, 0]], false);
  assert.deepEqual(rough(base, 3, 2, 1, 7), rough(base, 3, 2, 1, 7));
  assert.notDeepEqual(rough(base, 3, 2, 1, 7), rough(base, 3, 2, 1, 8));
  assert.equal(hash(12), hash(12));
});

test('a path is closed with Z only when asked', () => {
  const square = [[0, 0], [10, 0], [10, 10]];
  assert.match(path(square, true), /Z$/);
  assert.doesNotMatch(path(square, false), /Z/);
});

test('the logo is 440 wide at size 1, standing on its base', () => {
  const xs = archPoints(1).map(([x]) => x);
  assert.equal(Math.max(...xs) - Math.min(...xs), 440);
  assert.equal(Math.max(...archPoints(1).map(([, y]) => y)), 0);
});

test('both areas run November 2000 to December 2019, and only the rises of late 2019 are flagged', () => {
  for (const area of [demoArea, offlineArea]) {
    assert.equal(area.length, 230);
    assert.deepEqual([area[0].row, area[0].col], [0, 10]);
    assert.deepEqual([area.at(-1).row, area.at(-1).col], [19, 11]);
    assert.deepEqual(area.filter((m) => m.unusual).map((m) => [m.row, m.col]), [[19, 10], [19, 11]]);
  }
  assert.equal(demoArea.at(-1).value, 7112);
  assert.deepEqual(parseMonths('5 9!').map((m) => m.unusual), [false, true]);
});

test('the 2019 season flags November and December, which rise past their band', () => {
  assert.deepEqual(season2019.flatMap((m, i) => (m.unusual ? [i] : [])), [10, 11]);
  for (const m of season2019.filter((s) => s.unusual)) assert.ok(m.value - m.median > m.band);
});

test('the site colours a cell exactly as the app does', () => {
  for (const max of [1, 7, 2377, 8470]) {
    for (let value = 0; value <= max; value += Math.max(1, Math.floor(max / 97))) {
      assert.equal(rampStep(value, max), appRampStep(value, max), `${value} of ${max}`);
    }
  }
});
