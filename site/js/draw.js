// The hand-drawn look: shapes resampled evenly, a wobble fixed per shape (the hand) plus one that changes eight
// times a second (the boil), smooth curves through the points, and lines that trace themselves on and off.

const STEP_MOST = 14;

/** A number in -1..1 that depends only on `n`. */
export function hash(n) {
  const x = Math.sin(n * 127.1 + 311.7) * 43758.5453;
  return (x - Math.floor(x)) * 2 - 1;
}

/** Smooth noise along `u`, the same for the same `seed` and `u`. */
export function noise(seed, u) {
  const i = Math.floor(u);
  const f = u - i;
  const a = hash(seed * 7.13 + i * 1.618);
  const b = hash(seed * 7.13 + (i + 1) * 1.618);
  return a + (b - a) * f * f * (3 - 2 * f);
}

export const clamp = (x) => (x < 0 ? 0 : x > 1 ? 1 : x);
export const out3 = (x) => 1 - (1 - x) ** 3;
export const in3 = (x) => x * x * x;
export const inOut = (x) => (x < 0.5 ? 4 * x * x * x : 1 - (-2 * x + 2) ** 3 / 2);
/** A short spring: overshoots about 8% and settles at 1. */
export const spring = (x) => (x >= 1 ? 1 : 1 + 2.4 * (x - 1) ** 3 + 1.4 * (x - 1) ** 2);

/** Points spaced evenly along a polyline (closed when `closed`), each with its distance along the outline. */
export function resample(points, closed) {
  const q = closed ? [...points, points[0]] : points;
  const along = [0];
  for (let i = 1; i < q.length; i += 1) along.push(along[i - 1] + Math.hypot(q[i][0] - q[i - 1][0], q[i][1] - q[i - 1][1]));
  const total = along.at(-1);
  if (!(total > 0)) return points.map(([x, y]) => [x, y, 0]);
  const n = Math.max(2, Math.ceil(total / Math.min(STEP_MOST, Math.max(2.5, total / 20))));
  const result = [];
  let j = 1;
  for (let k = 0; k <= (closed ? n - 1 : n); k += 1) {
    const s = (total * k) / n;
    while (j < along.length - 1 && along[j] < s) j += 1;
    const f = (s - along[j - 1]) / (along[j] - along[j - 1] || 1);
    result.push([q[j - 1][0] + (q[j][0] - q[j - 1][0]) * f, q[j - 1][1] + (q[j][1] - q[j - 1][1]) * f, s]);
  }
  return result;
}

/** The resampled points moved by the hand's wobble (`amp`) and the boil of frame `frame` (`boil`). */
export function rough(base, seed, amp, boil, frame) {
  return base.map(([x, y, s]) => [
    x + noise(seed, s / 70) * amp + noise(seed + 40 + frame * 3.3, s / 38) * boil,
    y + noise(seed + 90, s / 70) * amp + noise(seed + 130 + frame * 3.3, s / 38) * boil,
  ]);
}

const r1 = (v) => Math.round(v * 10) / 10;

/** An SVG path of smooth cubic curves through every point. */
export function path(points, closed) {
  const n = points.length;
  if (n < 2) return '';
  const at = (i) => (closed ? points[(i + n) % n] : points[Math.max(0, Math.min(n - 1, i))]);
  let d = `M${r1(points[0][0])} ${r1(points[0][1])}`;
  for (let i = 0; i < (closed ? n : n - 1); i += 1) {
    const [a, b, c, e] = [at(i - 1), at(i), at(i + 1), at(i + 2)];
    d += `C${r1(b[0] + (c[0] - a[0]) / 6)} ${r1(b[1] + (c[1] - a[1]) / 6)} ${r1(c[0] - (e[0] - b[0]) / 6)} ${r1(c[1] - (e[1] - b[1]) / 6)} ${r1(c[0])} ${r1(c[1])}`;
  }
  return closed ? `${d}Z` : d;
}

/** The part of a polyline from fraction `a` to fraction `b` of its length. */
export function cut(points, a, b) {
  const along = [0];
  for (let i = 1; i < points.length; i += 1) along.push(along[i - 1] + Math.hypot(points[i][0] - points[i - 1][0], points[i][1] - points[i - 1][1]));
  const total = along.at(-1);
  const at = (s) => {
    let j = 1;
    while (j < points.length - 1 && along[j] < s) j += 1;
    const f = (s - along[j - 1]) / (along[j] - along[j - 1] || 1);
    return [points[j - 1][0] + (points[j][0] - points[j - 1][0]) * f, points[j - 1][1] + (points[j][1] - points[j - 1][1]) * f];
  };
  const result = [at(a * total)];
  for (let i = 0; i < points.length; i += 1) if (along[i] > a * total && along[i] < b * total) result.push(points[i]);
  result.push(at(b * total));
  return result;
}

export const arc = (cx, cy, rx, ry, a0, a1, n) =>
  Array.from({ length: n + 1 }, (_, i) => {
    const a = a0 + ((a1 - a0) * i) / n;
    return [cx + rx * Math.cos(a), cy + ry * Math.sin(a)];
  });

export const circle = (r) => arc(0, 0, r, r, 0, Math.PI * 2, 16).slice(0, 16);

/** A circle drawn the way a hand rings something: it starts anywhere and overshoots its start. */
export const ring = (r, seed, squash = 0.92) => {
  const a0 = hash(seed) * Math.PI;
  return Array.from({ length: 49 }, (_, i) => {
    const k = i / 48;
    const a = a0 + (Math.PI * 2 + 0.6) * k;
    const rr = r * (1.04 - 0.1 * k);
    return [rr * Math.cos(a), rr * Math.sin(a) * squash];
  });
};

export function rrect(x, y, w, h, r) {
  const points = [];
  const corner = (cx, cy, a0) => {
    for (let i = 0; i <= 4; i += 1) {
      const a = a0 + (Math.PI / 2) * (i / 4);
      points.push([cx + r * Math.cos(a), cy + r * Math.sin(a)]);
    }
  };
  corner(x + w - r, y + r, -Math.PI / 2);
  corner(x + w - r, y + h - r, 0);
  corner(x + r, y + h - r, Math.PI / 2);
  corner(x + r, y + r, Math.PI);
  return points;
}

/** firecal's logo as `xtask tokens` writes it: an arch 440 wide with sides 360 high, times `k`, base centre at 0. */
export const archPoints = (k) => [
  [-220 * k, 0],
  [-220 * k, -360 * k],
  ...arc(0, -360 * k, 220 * k, 220 * k, Math.PI, Math.PI * 2, 16).slice(1, -1),
  [220 * k, -360 * k],
  [220 * k, 0],
];

/** The logo's circle: radius 130 and centre 340 above the base, times `k`. */
export const logoCircle = (k) => ({ r: 130 * k, dy: -340 * k });
