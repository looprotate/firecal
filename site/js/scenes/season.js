// 2019 against the median of past years, on a log scale: a dot per month, its median as a tick, its band at two
// sigma as a blue line, and the diamond on the two months the core flags.
import { circle } from '../draw.js';
import { formatNumber, season2019 } from '../data.js';

const LEFT = 130;
const RIGHT = 950;
const BOTTOM = 640;
const DECADE = 130;
const y = (value) => BOTTOM - Math.log10(Math.max(value, 1)) * DECADE;
const x = (month) => 170 + month * 68;

export function build(scene) {
  [1, 10, 100, 1000, 10000].forEach((value, i) => {
    scene.line([[LEFT, y(value)], [RIGHT, y(value)]], { on: i * 1.5, dur: 6, w: 2, color: 'var(--line)', amp: 1, boil: 0.4 });
    scene.text(formatNumber(value), LEFT - 14, y(value) + 6, 'tiny end', { on: 2 + i * 1.5, color: 'var(--secondary)' });
  });
  scene.text('hotspots, log scale', LEFT, y(10000) - 26, 'tiny', { on: 8, color: 'var(--secondary)' });
  [...'JFMAMJJASOND'].forEach((letter, month) => scene.text(letter, x(month), BOTTOM + 40, 'tiny mid', { on: 4 + month * 0.4, color: 'var(--secondary)' }));

  season2019.forEach((month, i) => {
    const cx = x(i);
    scene.line([[cx - 18, y(month.median)], [cx + 18, y(month.median)]], { on: 12 + i * 1.2, dur: 3, w: 4, amp: 0.6, boil: 0.4 });
    const high = y(month.value + month.band);
    const low = y(month.value - month.band);
    scene.line([[cx, low], [cx, high]], { on: 26 + i * 2.2, dur: 3, w: 4, color: 'var(--uncertainty)', amp: 0.6, boil: 0.4 });
    scene.line([[cx - 7, high], [cx + 7, high]], { on: 27 + i * 2.2, dur: 2, w: 3, color: 'var(--uncertainty)', amp: 0.3 });
    scene.line([[cx - 7, low], [cx + 7, low]], { on: 27 + i * 2.2, dur: 2, w: 3, color: 'var(--uncertainty)', amp: 0.3 });
    scene.fill(circle(9), { x: cx, y: y(month.value), on: 28 + i * 2.2, dur: 3, color: 'var(--activity)', amp: 0.6, boil: 0.3 });
    if (month.unusual) {
      const top = high - 26;
      scene.fill([[0, -12], [12, 0], [0, 12], [-12, 0]], { x: cx, y: top, on: 66, color: 'var(--paper)', amp: 0.4, boil: 0.2 });
      scene.fill([[0, -9], [9, 0], [0, 9], [-9, 0]], { x: cx, y: top, on: 66.5, color: 'var(--ink)', amp: 0.4, boil: 0.2 });
    }
  });
  scene.text(`${formatNumber(7112)}`, x(11) - 26, y(7112) + 6, 'small end strong', { on: 58, color: 'var(--ink)' });
  scene.text('median 7', x(11) - 26, y(7) + 6, 'tiny end', { on: 60, color: 'var(--secondary)' });
}
