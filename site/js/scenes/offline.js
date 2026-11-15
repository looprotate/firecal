// The map of the region; the network goes off; a new area is drawn inside it; its calendar answers anyway.
import { arc, circle, rrect } from '../draw.js';
import { offlineArea } from '../data.js';
import { calendar } from './cells.js';

const MAP = [330, 250];
const geo = (lon, lat) => [50 + (lon - 146.5) * 112 - MAP[0], 80 + (-33.5 - lat) * 70 - MAP[1]];
const COAST = [[151.3, -33.5], [151.2, -34], [150.9, -34.6], [150.6, -35.2], [150.2, -35.8], [150.1, -36.4], [149.95, -37.1], [149.9, -37.5], [149.5, -37.75], [148.9, -37.8], [148.2, -37.85], [147.6, -37.9], [147.0, -38.1], [146.6, -38.5]];

export function build(scene) {
  const onMap = { x: MAP[0], y: MAP[1] };
  const coast = COAST.map(([lon, lat]) => geo(lon, lat));
  const [left, top] = [30 - MAP[0], 60 - MAP[1]];
  scene.fill(rrect(-300, -190, 600, 380, 10), { ...onMap, on: 0, color: 'var(--sea)', amp: 1.2, boil: 0.5 });
  scene.fill([...coast, [left + 6, 434 - MAP[1]], [left + 6, top + 6], [geo(151.35, -33.5)[0], top + 6]], { ...onMap, on: 3, color: 'var(--land)', amp: 1.2, boil: 0.5 });
  scene.line(coast, { ...onMap, on: 5, dur: 8, w: 2, color: 'var(--secondary)', amp: 0.8 });
  const [rx, ry] = geo(147, -34);
  const [rx2, ry2] = geo(151, -38);
  scene.line(rrect(rx, ry, rx2 - rx, ry2 - ry, 3), { ...onMap, closed: true, on: 12, dur: 8, w: 2, color: 'var(--secondary)', amp: 0.8 });

  [18, 36, 54].forEach((r, i) => scene.line(arc(0, 0, r, r, -Math.PI * 0.78, -Math.PI * 0.22, 10), { x: 820, y: 190, on: 24 + i * 1.5, dur: 3, w: 5, color: 'var(--secondary)', amp: 0.6 }));
  scene.fill(circle(6), { x: 820, y: 190, on: 24, color: 'var(--secondary)', amp: 0.3 });
  scene.line([[-50, -70], [50, 16]], { x: 820, y: 190, on: 33, dur: 4, w: 6, color: 'var(--attention)', amp: 0.8 });
  scene.text('Network off', 820, 246, 'small mid', { on: 35, color: 'var(--secondary)' });

  const fresh = [geo(148, -36), geo(150.5, -36), geo(150.5, -38), geo(148, -38)];
  scene.line(fresh, { ...onMap, closed: true, on: 42, dur: 16, w: 4, amp: 0.8 });
  [42, 49, 50, 55].forEach((on, i) => scene.fill(circle(7), { x: MAP[0] + fresh[i][0], y: MAP[1] + fresh[i][1], on, dur: 2, color: 'var(--ink)', amp: 0.4, boil: 0.2 }));

  scene.line([[650, 340], [690, 360], [722, 404]], { on: 60, dur: 4, w: 3, amp: 0.8 });
  scene.line([[708, 396], [722, 404], [724, 388]], { on: 63, dur: 2, w: 3, amp: 0.4 });
  const grid = calendar(scene, offlineArea, { x0: 740, y0: 370, pitch: 14, cell: 12, on: 64, wave: 1.2, flagsOn: 90 });
  const [x, y] = grid.centre(19, 11);
  scene.line(grid.ring(), { x, y, on: 92, dur: 5, w: 3, color: 'var(--attention)', amp: 0.6, boil: 0.4 });
  scene.text('answered with no network', 824, 700, 'small mid', { on: 88, color: 'var(--secondary)' });
}
