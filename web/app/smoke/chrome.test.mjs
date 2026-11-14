import assert from 'node:assert/strict';
import { test } from 'node:test';
import { chromePath } from './chrome.mjs';

test('CHROME_PATH wins over the usual install places', () => {
  assert.equal(chromePath({ CHROME_PATH: '/opt/chrome' }, 'linux', () => true), '/opt/chrome');
});

test('the first usual install place that exists on this system is used', () => {
  const found = chromePath({}, 'linux', (path) => path === '/usr/bin/chromium');
  assert.equal(found, '/usr/bin/chromium');
});

test('no Chrome anywhere is a message that says what to do', () => {
  assert.throws(() => chromePath({}, 'win32', () => false), /install it or set CHROME_PATH/);
});
