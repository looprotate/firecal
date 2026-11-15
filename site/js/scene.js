// One drawing: every line, fill and label with the moment it starts drawing and the moment it starts leaving, on
// a timeline from 0 to 100 that the page's scroll (or the clock, for the logo) moves along.
import { clamp, cut, in3, inOut, out3, path, resample, rough, spring } from './draw.js';

const SVG = 'http://www.w3.org/2000/svg';
const RISE = 14;

export class Scene {
  constructor(svg) {
    this.svg = svg;
    this.items = [];
    this.shapes = svg.appendChild(document.createElementNS(SVG, 'g'));
    this.labels = svg.appendChild(document.createElementNS(SVG, 'g'));
  }

  add(options) {
    const item = { on: 0, dur: 5, off: Infinity, offDur: 4, x: 0, y: 0, amp: 1.6, boil: 0.7, closed: false, ...options };
    item.seed = this.items.length * 1.37 + 3 + (this.svg.dataset.seed ? Number(this.svg.dataset.seed) : 0);
    if (Array.isArray(item.points)) item.base = resample(item.points, item.closed);
    const parent = item.kind === 'text' ? this.labels : this.shapes;
    item.node = parent.appendChild(document.createElementNS(SVG, item.kind === 'text' ? 'text' : 'path'));
    item.node.style.display = 'none';
    if (item.kind === 'line') {
      item.node.setAttribute('class', `stroke w${item.w ?? 3}`);
      item.node.style.stroke = item.color ?? 'var(--ink)';
    } else if (item.kind === 'fill') {
      item.node.setAttribute('class', 'fill');
      item.node.style.fill = item.color ?? 'var(--paper)';
    } else {
      item.node.setAttribute('class', `label ${item.cls ?? ''}`);
      item.node.style.fill = item.color ?? 'var(--text)';
    }
    this.items.push(item);
    return item;
  }

  line(points, options = {}) {
    return this.add({ kind: 'line', anim: 'draw', points, ...options });
  }

  fill(points, options = {}) {
    return this.add({ kind: 'fill', anim: 'pop', points, closed: true, dur: 4, ...options });
  }

  text(content, x, y, cls, options = {}) {
    return this.add({ kind: 'text', anim: 'fade', content, x, y, cls, dur: 4, offDur: 3, ...options });
  }

  /** Draws the scene at moment `t` of its timeline and boil frame `frame`; `still` skips the boil. */
  render(t, frame, still = false) {
    for (const item of this.items) draw(item, t, still ? 0 : frame, still);
  }
}

function show(item, visible) {
  if (item.shown !== visible) {
    item.node.style.display = visible ? '' : 'none';
    item.shown = visible;
  }
}

function set(item, name, value) {
  const key = `last_${name}`;
  if (item[key] !== value) {
    item.node.setAttribute(name, value);
    item[key] = value;
  }
}

function draw(item, t, frame, still) {
  if (t < item.on || t >= item.off + item.offDur) return show(item, false);
  const d = clamp((t - item.on) / item.dur);
  const q = clamp((t - item.off) / item.offDur);
  const at = item.at ? item.at(t) : { x: item.x, y: item.y, rot: 0 };
  if (item.kind === 'text') {
    const o = out3(d) * (1 - in3(q));
    if (o < 0.02) return show(item, false);
    show(item, true);
    const content = typeof item.content === 'function' ? item.content(t) : item.content;
    if (item.lastContent !== content) {
      item.node.textContent = content;
      item.lastContent = content;
    }
    set(item, 'x', String(Math.round(at.x * 10) / 10));
    set(item, 'y', String(Math.round(at.y * 10) / 10));
    set(item, 'opacity', String(Math.round(o * 1000) / 1000));
    set(item, 'transform', `translate(0 ${Math.round((1 - out3(d)) * RISE * 10) / 10})`);
    return undefined;
  }
  let scale = 1;
  let dy = 0;
  if (item.anim === 'pop') {
    scale = spring(d) * (1 - in3(q));
    if (scale < 0.02) return show(item, false);
  }
  if (item.anim === 'rise') dy = (1 - spring(d)) * item.rise + in3(q) * item.rise;
  const base = item.base ?? resample(item.points(t), item.closed);
  const points = rough(base, item.seed, still ? item.amp : item.amp, still ? 0 : item.boil, frame);
  let shape;
  if (item.kind === 'line' && item.anim === 'draw') {
    const a = inOut(q);
    const b = inOut(d);
    if (b - a < 0.003) return show(item, false);
    shape = path(cut(item.closed ? [...points, points[0]] : points, a, b), false);
  } else {
    shape = path(points, item.closed);
  }
  show(item, true);
  set(item, 'd', shape);
  const r = (v) => Math.round(v * 10) / 10;
  set(item, 'transform', `translate(${r(at.x)} ${r(at.y + dy)}) rotate(${r(at.rot ?? 0)}) scale(${Math.round(scale * 1000) / 1000})`);
  return undefined;
}
