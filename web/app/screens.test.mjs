import test from 'node:test';
import assert from 'node:assert/strict';
import { SCREEN_PATHS, screenForUrl } from './screens.ts';
test('each reference screen has a distinct direct entry path', () => {
 assert.deepEqual(Object.keys(SCREEN_PATHS), ['home','explorer','approach','data','about','support']);
 assert.equal(new Set(Object.values(SCREEN_PATHS)).size, 6);
 for (const [screen,path] of Object.entries(SCREEN_PATHS)) assert.equal(screenForUrl(path), screen);
});
test('old shared query links open the working explorer rather than hiding their results on home', () => {
 assert.equal(screenForUrl('/', '?v=1&box=147,-38,151,-34'), 'explorer');
 assert.equal(screenForUrl('/index.html', '?v=1'), 'explorer');
 assert.equal(screenForUrl('/index.html'), 'home');
 assert.equal(screenForUrl('/about.html', '?v=1'), 'about');
});
test('unknown paths and unversioned query strings cannot select an unexpected screen', () => {
 assert.equal(screenForUrl('/unknown.html'), 'home');
 assert.equal(screenForUrl('/', '?v=2'), 'home');
 assert.equal(screenForUrl('/', '?screen=support'), 'home');
});
