// firecal's mark drawn by the hand of the site: the ink arch traced and filled, then the terracotta circle, the way
// the site's logo draws itself (`site/js/scenes/logo.js`), small enough for the bar.
import { archPoints, circle, logoCircle, path, resample, rough } from '../../site/js/draw.js';

/** The mark's size against the logo's 440 units: 26 pixels wide, as the site's bar. */
const SCALE = 26 / 440;

const drawn = (points: number[][], seed: number, wobble: number): string =>
  path(rough(resample(points, true), seed, wobble, 0, 0), true);

/** The mark as SVG text, 26 by 34, base centre at the bottom. */
export function drawnLogo(): string {
  const { r, dy } = logoCircle(SCALE);
  const arch = drawn(archPoints(SCALE), 3, 0.5);
  const ring = drawn(circle(r).map(([x, y]) => [x, y + dy]), 5, 0.35);
  return '<svg class="drawn-logo" viewBox="-15 -36 30 38" width="26" height="34" aria-hidden="true">'
    + `<path class="logo-fill" d="${arch}"/><path class="logo-line" pathLength="1" d="${arch}"/>`
    + `<path class="logo-dot" d="${ring}"/></svg>`;
}
