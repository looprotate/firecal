import { REGION_CACHE } from './offline.ts';

/** A short, stable name for a list of files, so a new build gets a new shell cache. */
export function buildName(files: readonly string[]): string {
  let hash = 5381;
  for (const character of files.join('\n')) hash = ((hash * 33) ^ character.charCodeAt(0)) >>> 0;
  return hash.toString(36);
}

/**
 * The service worker's source. It stores `shell` (the page, its assets and `firecal.wasm`) when it installs, so the
 * second visit works with no network; serves pinned tiles from the region cache; and falls back to the stored page
 * when a navigation fails.
 */
export function serviceWorkerSource(shell: readonly string[]): string {
  return `
const SHELL_CACHE = 'firecal-shell-${buildName(shell)}';
const REGION_CACHE = '${REGION_CACHE}';
const SHELL = ${JSON.stringify(shell)};

self.addEventListener('install', (event) => {
  event.waitUntil((async () => {
    const cache = await caches.open(SHELL_CACHE);
    await Promise.all(SHELL.map(async (path) => {
      const response = await fetch(path, { cache: 'reload' });
      if (response.ok) await cache.put(path, response);
    }));
    await self.skipWaiting();
  })());
});

self.addEventListener('activate', (event) => {
  event.waitUntil((async () => {
    const keys = await caches.keys();
    await Promise.all(keys.filter((key) => key.startsWith('firecal-shell-') && key !== SHELL_CACHE).map((key) => caches.delete(key)));
    await self.clients.claim();
  })());
});

self.addEventListener('fetch', (event) => {
  const request = event.request;
  const url = new URL(request.url);
  if (request.method !== 'GET' || url.origin !== self.location.origin) return;
  event.respondWith((async () => {
    if (url.pathname.startsWith('/tiles/')) {
      const pinned = await (await caches.open(REGION_CACHE)).match(request, { ignoreVary: true });
      return pinned ?? fetch(request);
    }
    const shell = await caches.open(SHELL_CACHE);
    if (request.mode === 'navigate') {
      try {
        return await fetch(request);
      } catch {
        return (await shell.match('/', { ignoreSearch: true, ignoreVary: true })) ?? Response.error();
      }
    }
    return (await shell.match(request, { ignoreSearch: true, ignoreVary: true })) ?? fetch(request);
  })());
});
`;
}
