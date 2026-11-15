import test from 'node:test';
import assert from 'node:assert/strict';
import { installBrandIntro } from './brand-intro.ts';

function browser(reduced = false) {
  const originals = new Map();
  const animations = [];
  const delays = [];
  const layers = [];
  const listeners = new Map();
  const create = () => ({
    children: [], style: {}, parentNode: null,
    setAttribute() {}, addEventListener(name, callback) { listeners.set(name, callback); },
    append(...children) { this.children.push(...children); for (const child of children) child.parentNode = this; },
    remove() { this.removed = true; },
    animate() { return { finished: Promise.resolve(), cancel() {} }; },
  });
  const home = { insertBefore(child) { child.parentNode = this; } };
  const logo = { ...create(), style: { cssText: '' }, parentNode: home, nextSibling: null, complete: true, naturalWidth: 100,
    decode: () => Promise.resolve(), getBoundingClientRect: () => ({ x: 190, y: 24, width: 34, height: 40 }),
    animate(frames, options) { animations.push({ frames, options }); return { finished: Promise.resolve(), cancel() {} }; },
  };
  const root = { inert: false, querySelector: selector => selector === '#brand-logo' ? logo : null };
  const values = {
    innerWidth: 1200, innerHeight: 800,
    matchMedia: () => ({ matches: reduced, addEventListener() {}, removeEventListener() {} }),
    sessionStorage: { seen: null, getItem() { return this.seen; }, setItem(_key, value) { this.seen = value; } },
    window: { scrollY: 0, addEventListener() {}, removeEventListener() {} },
    document: { createElement: create, documentElement: { lang: 'en' }, body: { style: { overflow: '' }, append(layer) { layers.push(layer); } }, activeElement: null, addEventListener() {}, removeEventListener() {} },
    setTimeout: (_callback, delay) => { delays.push(delay); return 1; }, clearTimeout() {},
  };
  for (const [key, value] of Object.entries(values)) { originals.set(key, Object.getOwnPropertyDescriptor(globalThis, key)); Object.defineProperty(globalThis, key, { value, writable: true, configurable: true }); }
  return { root, logo, home, animations, delays, layers, restore() { for (const [key, descriptor] of originals) { if (descriptor) Object.defineProperty(globalThis, key, descriptor); else delete globalThis[key]; } } };
}

test('entry plays on repeated page visits even when a session has already seen it', async () => {
  const b = browser();
  try {
    globalThis.sessionStorage.seen = 'seen';
    await installBrandIntro(b.root);
    await installBrandIntro(b.root);
    assert.equal(b.animations.length, 2);
    assert.equal(b.logo.parentNode, b.home);
    assert.equal(b.root.inert, false);
    assert.ok(b.layers.every(layer => layer.removed));
  } finally { b.restore(); }
});
test('the asset gets a realistic load window and the logo finishes at the measured right-side slot', async () => {
  const b = browser();
  try {
    await installBrandIntro(b.root);
    assert.ok(b.delays[0] >= 3000);
    assert.equal(b.animations[0].frames.at(-1).transform, 'translate(190px, 24px) scale(1)');
  } finally { b.restore(); }
});
test('reduced motion keeps the settled logo without a blocking intro', async () => {
  const b = browser(true);
  try { await installBrandIntro(b.root); assert.equal(b.animations.length, 0); assert.equal(b.layers.length, 0); assert.equal(b.root.inert, false); }
  finally { b.restore(); }
});
