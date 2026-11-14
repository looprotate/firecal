import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

const css = readFileSync(new URL('./tokens.css', import.meta.url), 'utf8');

function token(name) {
  const found = new RegExp(`--${name}:\\s*(#[0-9a-f]{6});`).exec(css);
  assert.ok(found, `--${name} is missing from tokens.css`);
  return found[1];
}

const ramp = [1, 2, 3, 4, 5, 6, 7].map((step) => token(`ramp-${step}`));

function toLinear(hex) {
  return [1, 3, 5].map((at) => {
    const c = parseInt(hex.slice(at, at + 2), 16) / 255;
    return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
  });
}

function luminance(hex) {
  const [r, g, b] = toLinear(hex);
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

function contrast(a, b) {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (hi + 0.05) / (lo + 0.05);
}

function oklabOfLinear([r, g, b]) {
  const l = Math.cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b);
  const m = Math.cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b);
  const s = Math.cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b);
  return [
    0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s,
    1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s,
    0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s,
  ];
}

function distance(a, b) {
  return Math.hypot(...a.map((value, at) => value - b[at]));
}

// Machado, Oliveira and Fernandes 2009, severity 1.0, applied to linear RGB.
const BLINDNESS = {
  protanopia: [
    [0.152286, 1.052583, -0.204868],
    [0.114503, 0.786281, 0.099216],
    [-0.003882, -0.048116, 1.051998],
  ],
  deuteranopia: [
    [0.367322, 0.860646, -0.227968],
    [0.280085, 0.672501, 0.047413],
    [-0.01182, 0.04294, 0.968881],
  ],
  tritanopia: [
    [1.255528, -0.076749, -0.178779],
    [-0.078411, 0.930809, 0.147602],
    [0.004733, 0.691367, 0.3039],
  ],
};

function simulate(matrix, linear) {
  return matrix.map((row) => row.reduce((sum, weight, at) => sum + weight * linear[at], 0));
}

test('each step stands further from the page than the one before, so grayscale keeps the order', () => {
  for (let at = 1; at < ramp.length; at += 1) {
    const ratio = contrast(ramp[at], token('bg')) / contrast(ramp[at - 1], token('bg'));
    assert.ok(ratio >= 1.2, `step ${at + 1} is only ${ratio.toFixed(2)} times step ${at} in contrast with the page`);
  }
});

test('every step stands out from the page: 3:1, WCAG 1.4.11', () => {
  ramp.forEach((color, at) => {
    const ratio = contrast(color, token('bg'));
    assert.ok(ratio >= 3, `step ${at + 1} is ${ratio.toFixed(2)}:1 against the page`);
  });
});

test('neighbouring steps stay apart under each kind of colour blindness', () => {
  // 0.04 in OKLab is twice the smallest difference an eye notices.
  for (const [kind, matrix] of Object.entries(BLINDNESS)) {
    const seen = ramp.map((color) => oklabOfLinear(simulate(matrix, toLinear(color))));
    for (let at = 1; at < seen.length; at += 1) {
      const apart = distance(seen[at], seen[at - 1]);
      assert.ok(apart >= 0.04, `${kind}: steps ${at} and ${at + 1} are ${apart.toFixed(3)} apart`);
    }
  }
});

test('an empty cell is not the first step', () => {
  assert.ok(contrast(token('ramp-0'), ramp[0]) >= 1.5);
});

test('the diamond shows on every step', () => {
  ramp.forEach((color, at) => {
    const edge = contrast(token('mark-edge'), token('mark-fill'));
    assert.ok(edge >= 7, `the diamond's own edge is ${edge.toFixed(2)}:1 against its fill`);
    const outer = Math.max(contrast(token('mark-fill'), color), contrast(token('mark-edge'), color));
    assert.ok(outer >= 3, `step ${at + 1}: the diamond is ${outer.toFixed(2)}:1 against the cell`);
  });
});

test('the provisional triangle shows on every step and on an empty cell, without color', () => {
  const size = /--provisional-size:\s*(\d+)px;/.exec(css);
  assert.ok(size, '--provisional-size is missing from tokens.css');
  assert.ok(Number(size[1]) >= 8, `the triangle is ${size[1]}px along each side, under 8px`);
  const edge = contrast(token('provisional-edge'), token('provisional-fill'));
  assert.ok(edge >= 7, `the triangle's own edge is ${edge.toFixed(2)}:1 against its fill`);
  [token('ramp-0'), ...ramp].forEach((color, at) => {
    const outer = Math.max(contrast(token('provisional-fill'), color), contrast(token('provisional-edge'), color));
    assert.ok(outer >= 3, `step ${at}: the triangle is ${outer.toFixed(2)}:1 against the cell`);
  });
});

test('text meets WCAG AA on the page and on a surface', () => {
  for (const name of ['text', 'text-strong', 'text-secondary', 'error', 'added']) {
    for (const ground of ['bg', 'bg-deep', 'surface']) {
      const ratio = contrast(token(name), token(ground));
      assert.ok(ratio >= 4.5, `--${name} on --${ground} is ${ratio.toFixed(2)}:1`);
    }
  }
});

test('each colour that means something stands out from the page: 3:1, WCAG 1.4.11', () => {
  for (const name of ['activity', 'uncertainty', 'attention', 'healthy']) {
    const ratio = contrast(token(name), token('bg'));
    assert.ok(ratio >= 3, `--${name} is ${ratio.toFixed(2)}:1 against the page`);
  }
});

test('activity is the first step of the ramp, and the last step is the ink of the text', () => {
  assert.equal(ramp[0], token('activity'));
  assert.equal(ramp[ramp.length - 1], token('text-strong'));
});

test('the logo is drawn only in token colours', () => {
  const logo = readFileSync(new URL('./logo.svg', import.meta.url), 'utf8');
  const used = [...logo.matchAll(/fill="(#[0-9a-f]{6})"/g)].map((found) => found[1]);
  assert.deepEqual(used, [token('text-strong'), token('activity')]);
});
