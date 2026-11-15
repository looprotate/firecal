// Any area, any season: the Earth again, with a calendar standing wherever someone draws one.
import { rrect } from '../draw.js';
import { earth, surface } from './satellites.js';

export function build(scene) {
  earth(scene, 0);
  [130, 340, 500, 660, 870].forEach((px, i) => {
    const sy = surface(px);
    const on = 18 + i * 6;
    const [ix, iy] = [px, sy - 94];
    scene.line([[px, sy], [px, sy - 64]], { on, dur: 4, w: 3, amp: 0.6 });
    scene.fill(rrect(-36, -30, 72, 60, 6), { x: ix, y: iy, on: on + 3, amp: 0.8 });
    scene.fill(rrect(-36, -30, 72, 16, 6), { x: ix, y: iy, on: on + 4, color: 'var(--fire)', amp: 0.6 });
    [[0, 0], [1, 0], [2, 0], [0, 1], [1, 1], [2, 1]].forEach(([c, r], j) => {
      scene.fill(rrect(-7, -5, 14, 10, 2), { x: ix - 19 + c * 19, y: iy + 3 + r * 15, on: on + 5 + j * 0.4, color: `var(--ramp-${[1, 3, 2, 5, 7, 4][(j + i) % 6]})`, amp: 0.4, boil: 0.2 });
    });
    scene.line(rrect(-36, -30, 72, 60, 6), { x: ix, y: iy, closed: true, on: on + 2, dur: 5, w: 3, amp: 0.8 });
  });
}
