// How firecal is built, as six boxes joined by arrows: the archive, ingest, harmonize, the tiles, the one Rust core
// and the browser, each with what was measured there (pitch/numbers.md).
import { arc, circle, rrect } from '../draw.js';

const W = 250;
const H = 150;
const NODES = [
  { x: 170, y: 190, title: 'NASA FIRMS', note: '16.6 million hotspot rows', icon: sheets },
  { x: 500, y: 190, title: 'Ingest', note: 'every row in 7.3 s, on a laptop', icon: funnel },
  { x: 830, y: 190, title: 'Harmonize', note: '230 months in 0.23 s, with sigma', icon: bars },
  { x: 830, y: 530, title: 'Static tiles', note: '2.1 MiB for a 10° tile', icon: tiles },
  { x: 500, y: 530, title: 'One Rust core', note: 'browser = native, 8,968 calls', icon: twins },
  { x: 170, y: 530, title: 'Your browser', note: '0.75 ms a query, no server', icon: browser },
];

function sheets(scene, x, y, on) {
  [[-22, -14], [-11, -7], [0, 0]].forEach(([dx, dy], i) => {
    scene.fill(rrect(-30, -22, 60, 44, 5), { x: x + dx, y: y + dy, on: on + i, color: 'var(--paper)', amp: 0.6 });
    scene.line(rrect(-30, -22, 60, 44, 5), { x: x + dx, y: y + dy, on: on + i, dur: 3, closed: true, w: 2, amp: 0.6 });
  });
  [-8, 0, 8].forEach((dy, i) => scene.line([[-16, dy], [16, dy]], { x, y, on: on + 3 + i * 0.5, dur: 2, w: 2, color: 'var(--secondary)', amp: 0.3 }));
}

function funnel(scene, x, y, on) {
  scene.line([[-36, -24], [36, -24], [8, 6], [8, 26], [-8, 26], [-8, 6]], { x, y, on, dur: 5, closed: true, w: 3, amp: 0.8 });
  [[-20, -36], [0, -40], [18, -35]].forEach(([dx, dy], i) => scene.fill(circle(4), { x: x + dx, y: y + dy, on: on + 2 + i, color: 'var(--activity)', amp: 0.3 }));
}

function bars(scene, x, y, on) {
  scene.fill(rrect(-30, -6, 20, 32, 3), { x, y, on, color: 'var(--activity)', amp: 0.5 });
  scene.fill(rrect(10, -8, 20, 34, 3), { x, y, on: on + 1, color: 'var(--uncertainty)', amp: 0.5 });
  scene.line([[20, -14], [20, -34]], { x, y, on: on + 2, dur: 2, w: 3, color: 'var(--uncertainty)', amp: 0.3 });
  scene.line([[-38, 26], [38, 26]], { x, y, on, dur: 3, w: 3, amp: 0.4 });
}

function tiles(scene, x, y, on) {
  for (let r = 0; r < 3; r += 1) {
    for (let c = 0; c < 3; c += 1) {
      scene.fill(rrect(-8, -8, 16, 16, 3), { x: x - 22 + c * 22, y: y - 22 + r * 22, on: on + (r * 3 + c) * 0.3, color: `var(--ramp-${[1, 2, 4, 1, 3, 7, 2, 5, 1][r * 3 + c]})`, amp: 0.4 });
    }
  }
}

function twins(scene, x, y, on) {
  scene.line(arc(-14, 0, 24, 24, 0, Math.PI * 2, 18), { x, y, on, dur: 4, w: 3, amp: 0.6 });
  scene.line(arc(14, 0, 24, 24, 0, Math.PI * 2, 18), { x, y, on: on + 1, dur: 4, w: 3, color: 'var(--activity)', amp: 0.6 });
}

function browser(scene, x, y, on) {
  scene.fill(rrect(-38, -26, 76, 52, 6), { x, y, on, color: 'var(--paper)', amp: 0.6 });
  scene.line(rrect(-38, -26, 76, 52, 6), { x, y, on, dur: 4, closed: true, w: 3, amp: 0.6 });
  scene.line([[-38, -14], [38, -14]], { x, y, on: on + 2, dur: 2, w: 2, amp: 0.3 });
  for (let r = 0; r < 2; r += 1) {
    for (let c = 0; c < 4; c += 1) {
      scene.fill(rrect(-5, -4, 10, 8, 2), { x: x - 21 + c * 14, y: y - 2 + r * 13, on: on + 3 + (r * 4 + c) * 0.2, color: `var(--ramp-${[1, 3, 2, 6, 1, 4, 7, 2][r * 4 + c]})`, amp: 0.3 });
    }
  }
}

function arrow(scene, from, to, on) {
  const [x1, y1] = from;
  const [x2, y2] = to;
  const bend = [(x1 + x2) / 2 + (y2 - y1) * 0.12, (y1 + y2) / 2 - (x2 - x1) * 0.08];
  scene.line([from, bend, to], { on, dur: 4, w: 3, amp: 0.8 });
  const angle = Math.atan2(y2 - bend[1], x2 - bend[0]);
  const wing = (turn) => [x2 - 16 * Math.cos(angle + turn), y2 - 16 * Math.sin(angle + turn)];
  scene.line([wing(0.5), to, wing(-0.5)], { on: on + 3, dur: 2, w: 3, amp: 0.4 });
}

export function build(scene) {
  NODES.forEach((node, i) => {
    const on = 4 + i * 13;
    scene.fill(rrect(-W / 2, -H / 2, W, H, 10), { x: node.x, y: node.y, on, color: 'var(--paper)', amp: 1.2, boil: 0.5 });
    scene.line(rrect(-W / 2, -H / 2, W, H, 10), { x: node.x, y: node.y, on, dur: 5, closed: true, w: 4, amp: 1.2, boil: 0.5 });
    node.icon(scene, node.x, node.y - 22, on + 3);
    scene.text(node.title, node.x, node.y + 50, 'h3 mid', { on: on + 4, color: 'var(--ink)' });
    scene.text(node.note, node.x, node.y + H / 2 + 32, 'small mid', { on: on + 6, color: 'var(--secondary)' });
  });
  const ends = [
    [[300, 190], [370, 190]],
    [[630, 190], [700, 190]],
    [[830, 312], [830, 440]],
    [[700, 530], [630, 530]],
    [[370, 530], [300, 530]],
  ];
  ends.forEach(([from, to], i) => arrow(scene, from, to, 12 + i * 13));
}
