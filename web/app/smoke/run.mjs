// The web smoke test (issue #22): the built app in headless Chrome, a fixed area over a fixture tile, the calendar
// cell checked; then the host stopped, the page reloaded, and the same check with no network.
// Run with `npm run smoke` after `cargo xtask build-wasm` or `cargo xtask check` (it reads the debug wasm build).
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { openPage } from './chrome.mjs';
import { serve } from './server.mjs';

const MONTHS = 38; // January 2000 to February 2003.
const CELLS = 1600;

/** Tile row 7, column 18 (10 to 20 north, 0 to 10 east). Its south-west cell holds 10 plus the year since 2000. */
function fixtureTile() {
  const bytes = new Uint8Array(16 + 3 * MONTHS * CELLS * 2);
  const view = new DataView(bytes.buffer);
  bytes.set([0x46, 0x43, 0x54, 0x31]);
  view.setUint16(4, MONTHS, true);
  view.setUint8(8, 3);
  for (let month = 0; month < MONTHS; month += 1) {
    const at = month * CELLS + 39 * 40;
    view.setUint16(16 + at * 2, 10 + Math.floor(month / 12), true);
    view.setUint16(16 + (MONTHS * CELLS + at) * 2, 1, true);
  }
  return bytes;
}

const EXPECTED = 'January 2003: 13 hotspots, uncertainty ±1 (estimated)';
const dist = fileURLToPath(new URL('../../dist', import.meta.url));
const wasm = readFileSync(new URL('../../../target/wasm32-unknown-unknown/debug/firecal_wasm.wasm', import.meta.url));

const useArea = (west, south, east, north) => `(() => {
  const form = document.querySelector('#area-form');
  const set = (name, value) => { form.querySelector('[name=' + name + ']').value = String(value); };
  set('west', ${west}); set('south', ${south}); set('east', ${east}); set('north', ${north});
  form.querySelector('button[type=submit]').click();
  return true;
})()`;
const januaryCell = `document.querySelector('.calendar-grid [aria-label^="January 2003"]')?.getAttribute('aria-label')`;

const host = await serve(dist, { '/tiles/t_7_18.bin': fixtureTile(), '/firecal.wasm': wasm });
const { page, close } = await openPage();
try {
  await page.navigate(host.origin);
  await page.waitFor(`document.querySelector('#runtime-status')?.textContent.includes('hand-made sample')`);
  await page.waitFor(`navigator.serviceWorker.ready.then(() => Boolean(navigator.serviceWorker.controller))`);

  await page.evaluate(useArea(0, 10, 0.25, 10.25));
  await page.evaluate(`document.querySelector('#pin-region').click()`);
  assert.equal(await page.waitFor(januaryCell), EXPECTED, 'online, after pinning');

  await host.stop();
  await page.navigate(host.origin);
  await page.waitFor(`document.querySelector('#runtime-status')?.textContent.includes('hand-made sample')`);
  const network = await page.evaluate(`fetch('/not-stored-' + Date.now()).then(() => 'up', () => 'down')`);
  assert.equal(network, 'down', 'the host is stopped');
  await page.evaluate(useArea(0, 10, 0.2, 10.2));
  assert.equal(await page.waitFor(januaryCell), EXPECTED, 'offline, a new area inside the pinned region');
  console.log('smoke: the app answered online and with the network off');
} finally {
  await close();
}
