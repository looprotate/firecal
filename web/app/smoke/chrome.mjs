// Headless Chrome driven over the DevTools protocol, with Node's own WebSocket: no browser library.
import { spawn } from 'node:child_process';
import { existsSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const KNOWN_PATHS = {
  darwin: ['/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'],
  linux: ['/usr/bin/google-chrome', '/usr/bin/google-chrome-stable', '/usr/bin/chromium', '/usr/bin/chromium-browser'],
  win32: [
    'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe',
    'C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe',
  ],
};

/** The Chrome to run: `CHROME_PATH` when set, else the usual install place on this system. */
export function chromePath(env = process.env, platform = process.platform, exists = existsSync) {
  if (env.CHROME_PATH) return env.CHROME_PATH;
  const found = (KNOWN_PATHS[platform] ?? []).find((path) => exists(path));
  if (!found) throw new Error('Chrome was not found: install it or set CHROME_PATH.');
  return found;
}

/** Starts headless Chrome on a throwaway profile and returns one page to drive, and a way to close it all. */
export async function openPage() {
  const profile = mkdtempSync(join(tmpdir(), 'firecal-smoke-'));
  const chrome = spawn(chromePath(), [
    '--headless=new', '--remote-debugging-port=0', `--user-data-dir=${profile}`,
    '--no-first-run', '--no-default-browser-check', 'about:blank',
  ], { stdio: ['ignore', 'ignore', 'pipe'] });
  const address = await new Promise((resolve, reject) => {
    let seen = '';
    chrome.stderr.on('data', (chunk) => {
      seen += chunk;
      const found = /DevTools listening on (ws:\/\/\S+)/.exec(seen);
      if (found) resolve(found[1]);
    });
    chrome.on('exit', (code) => reject(new Error(`Chrome exited with ${code} before it listened`)));
  });
  const socket = new WebSocket(address);
  await new Promise((resolve, reject) => {
    socket.addEventListener('open', resolve, { once: true });
    socket.addEventListener('error', reject, { once: true });
  });
  let next = 0;
  const waiting = new Map();
  socket.addEventListener('message', (event) => {
    const message = JSON.parse(String(event.data));
    const done = waiting.get(message.id);
    if (!done) return;
    waiting.delete(message.id);
    if (message.error) done.reject(new Error(message.error.message));
    else done.resolve(message.result);
  });
  const send = (method, params = {}, sessionId) => new Promise((resolve, reject) => {
    next += 1;
    waiting.set(next, { resolve, reject });
    socket.send(JSON.stringify({ id: next, method, params, ...(sessionId ? { sessionId } : {}) }));
  });

  const { targetId } = await send('Target.createTarget', { url: 'about:blank' });
  const { sessionId } = await send('Target.attachToTarget', { targetId, flatten: true });
  const page = {
    navigate: (url) => send('Page.navigate', { url }, sessionId),
    /** The value of a JavaScript expression in the page; a thrown error or rejected promise fails the test. */
    async evaluate(expression) {
      const { result, exceptionDetails } = await send(
        'Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true }, sessionId,
      );
      if (exceptionDetails) throw new Error(`In the page: ${exceptionDetails.exception?.description ?? exceptionDetails.text}`);
      return result.value;
    },
    /** Polls `expression` until it is truthy, then returns it; fails after `seconds`. */
    async waitFor(expression, seconds = 20) {
      const until = Date.now() + seconds * 1000;
      for (;;) {
        const value = await page.evaluate(expression).catch(() => undefined);
        if (value) return value;
        if (Date.now() > until) throw new Error(`Timed out waiting for: ${expression}`);
        await new Promise((resolve) => setTimeout(resolve, 200));
      }
    },
  };
  const close = async () => {
    socket.close();
    chrome.kill();
    await new Promise((resolve) => (chrome.exitCode === null ? chrome.on('exit', resolve) : resolve()));
    rmSync(profile, { recursive: true, force: true, maxRetries: 5, retryDelay: 200 });
  };
  return { page, close };
}
