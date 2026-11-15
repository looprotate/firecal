// The logo drawing itself: the arch traced and filled with ink, then the terracotta circle under its top.
import { archPoints, circle, logoCircle, ring } from '../draw.js';

/** Adds the logo at size `k` (1 is 440 wide), base centre at the origin, drawn from `on` over 16 units. */
export function build(scene, k = 1, on = 0) {
  const { r, dy } = logoCircle(k);
  scene.line(archPoints(k), { on, dur: 9, closed: true, w: 'logo', amp: 2 * k, boil: 1.2 * k });
  scene.fill(archPoints(k), { on: on + 6, dur: 5, color: 'var(--ink)', amp: 2 * k, boil: 1.2 * k });
  scene.line(ring(r, 3), { y: dy, on: on + 5, dur: 6, w: 'logo', color: 'var(--activity)', amp: 1.5 * k, boil: k });
  scene.fill(circle(r), { y: dy, on: on + 10, dur: 4, color: 'var(--activity)', amp: 1.5 * k, boil: k });
}
