import assert from 'node:assert/strict';
import { test } from 'node:test';
import { REGION_CACHE } from './offline.ts';
import { buildName, serviceWorkerSource } from './service-worker.ts';

test('the worker is valid JavaScript that stores the shell it is given and reads the region cache', () => {
  const source = serviceWorkerSource(['/', '/assets/index-abc.js', '/firecal.wasm']);
  assert.doesNotThrow(() => new Function(source));
  assert.ok(source.includes('["/","/assets/index-abc.js","/firecal.wasm"]'));
  assert.ok(source.includes(`const REGION_CACHE = '${REGION_CACHE}';`));
  // A host that sends `Vary: Origin` must not hide stored assets from the page's crossorigin module requests.
  assert.equal(source.match(/\.match\(/g).length, source.match(/ignoreVary: true/g).length);
});

test('a different build gets a different shell cache, and the same build the same one', () => {
  assert.equal(buildName(['/', '/assets/a.js']), buildName(['/', '/assets/a.js']));
  assert.notEqual(buildName(['/', '/assets/a.js']), buildName(['/', '/assets/b.js']));
});
