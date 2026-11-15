// The calendar of south-east Australia, cell by cell; December 2019 rung; then the three years most like 2019.
import { analogs, demoArea, formatNumber } from '../data.js';
import { calendar } from './cells.js';

const X0 = 120;
const Y0 = 80;
const PITCH = 32;
const CELL = 28;

export function build(scene) {
  const grid = calendar(scene, demoArea, { x0: X0, y0: Y0, pitch: PITCH, cell: CELL, on: 4, wave: 2, labels: true, flagsOn: 46 });
  const [x, y] = grid.centre(19, 11);
  scene.line(grid.ring(), { x, y, on: 48, dur: 6, w: 4, color: 'var(--attention)', amp: 0.8, boil: 0.5 });
  scene.line([[x + 26, y - 14], [x + 52, y - 40], [x + 78, y - 58]], { on: 52, dur: 3, w: 3, color: 'var(--attention)', amp: 0.8 });
  scene.text(`${formatNumber(7112)} ± 88`, x + 88, y - 66, 'number', { on: 53, color: 'var(--ink)' });
  scene.text('hotspots, December 2019', x + 90, y - 38, 'small', { on: 54, color: 'var(--secondary)' });
  scene.text('a median December: 7', x + 90, y - 12, 'small', { on: 55, color: 'var(--secondary)' });

  analogs.forEach(([year], rank) => {
    const row = year - 2000;
    const on = 66 + rank * 3;
    scene.line(grid.rowOutline(row), { on, dur: 5, w: 3, closed: true, amp: 0.8, boil: 0.5 });
    scene.text(`${rank + 1}`, X0 + 12 * PITCH + 34, Y0 + row * PITCH + 21, 'small mid strong', { on: on + 3, color: 'var(--ink)' });
  });
}
