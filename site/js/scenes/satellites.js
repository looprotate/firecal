// Two satellites over the Earth drop their 2019 hotspots, 12.5 times apart; then VIIRS is harmonized onto MODIS.
import { arc, circle, clamp, hash, inOut, out3, rrect } from '../draw.js';
import { formatNumber, heldOut } from '../data.js';

const EX = 500;
const EY = 1950;
const EARTH = 1300;
const ORBIT = 1700;
const surface = (x) => EY - Math.sqrt(EARTH * EARTH - (x - EX) * (x - EX));

/** The Earth as an arc rising from below, with its land, from `on` until `off`. */
export function earth(scene, on, off = Infinity) {
  const top = arc(EX, EY, EARTH, EARTH, -Math.PI / 2 - 0.62, -Math.PI / 2 + 0.62, 64);
  const timing = { anim: 'rise', rise: 300, on, dur: 12, off, offDur: 8, amp: 2.5, boil: 0.8 };
  scene.fill([...top, [1200, 1100], [-200, 1100]], { ...timing, color: 'var(--land)' });
  scene.line(top, { ...timing, w: 4 });
}

export { surface };

function satellite(scene, at, on) {
  const o = { at, on, amp: 0.8, boil: 0.4 };
  scene.fill(rrect(-74, -9, 46, 18, 3), { ...o, on: on + 3, color: 'var(--panel)' });
  scene.fill(rrect(28, -9, 46, 18, 3), { ...o, on: on + 3, color: 'var(--panel)' });
  scene.fill(rrect(-21, -17, 42, 34, 6), { ...o, on: on + 2 });
  scene.line(rrect(-74, -9, 46, 18, 3), { ...o, closed: true, w: 2 });
  scene.line(rrect(28, -9, 46, 18, 3), { ...o, closed: true, w: 2 });
  scene.line(rrect(-21, -17, 42, 34, 6), { ...o, closed: true, w: 3 });
  for (const s of [[[-51, -9], [-51, 9]], [[51, -9], [51, 9]], [[-28, 0], [-21, 0]], [[21, 0], [28, 0]]]) {
    scene.line(s, { ...o, on: on + 4, dur: 2, w: 2, amp: 0.3 });
  }
  scene.line(arc(0, 17, 8, 7, 0, Math.PI, 6), { ...o, on: on + 4, dur: 2, w: 2, amp: 0.3 });
}

export function build(scene) {
  earth(scene, 0);
  const a0 = -Math.PI / 2 - 0.36;
  const a1 = -Math.PI / 2 + 0.36;
  for (let i = 0; i < 24; i += 1) {
    const a = a0 + ((a1 - a0) * i) / 24;
    scene.line(arc(EX, EY, ORBIT, ORBIT, a, a + ((a1 - a0) / 24) * 0.55, 3), { on: 5 + i * 0.5, dur: 3, w: 2, color: 'var(--secondary)', amp: 0.8, boil: 0.4 });
  }
  const orbitAt = (base) => (t) => {
    const a = -Math.PI / 2 + base + 0.0007 * t;
    return { x: EX + ORBIT * Math.cos(a), y: EY + ORBIT * Math.sin(a), rot: ((a + Math.PI / 2) * 180) / Math.PI };
  };
  const modis = orbitAt(-0.17);
  const viirs = orbitAt(0.15);
  const beam = (at, xa, xb, on) => scene.fill((t) => {
    const g = out3(clamp((t - on) / 8));
    const p = at(t);
    const ax = p.x;
    const ay = p.y + 22;
    const points = [[ax, ay]];
    for (let i = 0; i <= 10; i += 1) {
      const x = xa + ((xb - xa) * i) / 10;
      points.push([ax + (x - ax) * g, ay + (surface(x) - ay) * g]);
    }
    return points;
  }, { anim: 'fixed', on, dur: 8, color: 'var(--beam)', amp: 1.2, boil: 0.5 });
  beam(modis, 80, 350, 20);
  beam(viirs, 610, 900, 22);

  const dots = (xa, xb, count, seed) => Array.from({ length: count }, (_, i) => {
    const x = xa + 16 + (xb - xa - 32) * ((hash(seed + i * 3.1) + 1) / 2);
    const y = surface(x) + 14 + 70 * ((hash(seed + i * 7.7) + 1) / 2);
    return scene.fill(circle(5.5), { x, y, on: 28 + (24 * i) / count, dur: 3, color: 'var(--activity)', amp: 0.6, boil: 0.3 });
  });
  dots(80, 350, 8, 11);
  const kept = new Set([7, 23, 41, 58, 76, 92]);
  let leaving = 0;
  for (const [i, dot] of dots(610, 900, 100, 29).entries()) {
    if (kept.has(i)) continue;
    dot.off = 64 + leaving * 0.17;
    dot.offDur = 3;
    leaving += 1;
  }

  satellite(scene, modis, 11);
  satellite(scene, viirs, 13);
  const above = (at) => (t) => ({ x: at(t).x, y: at(t).y - 44 });
  scene.text('MODIS · since 2000', 0, 0, 'h3 mid', { at: above(modis), on: 14, color: 'var(--ink)' });
  scene.text('VIIRS · since 2012', 0, 0, 'h3 mid', { at: above(viirs), on: 16, color: 'var(--ink)' });

  const counting = (t, from, to, t0, t1) => from + (to - from) * inOut(clamp((t - t0) / (t1 - t0)));
  scene.text((t) => formatNumber(counting(t, 0, heldOut.modis, 28, 52)), 215, 600, 'number mid', { on: 27, color: 'var(--ink)' });
  scene.text('MODIS hotspots, 2019', 215, 628, 'small mid', { on: 28, color: 'var(--secondary)' });
  scene.text((t) => formatNumber(t < 64 ? counting(t, 0, heldOut.viirsRaw, 28, 52) : counting(t, heldOut.viirsRaw, heldOut.viirsHarmonized, 64, 82)), 755, 600, 'number mid', { on: 27, color: 'var(--ink)' });
  scene.text((t) => (t < 64 ? 'VIIRS hotspots, raw' : 'VIIRS, harmonized'), 755, 628, 'small mid', { on: 28, color: 'var(--secondary)' });
}
