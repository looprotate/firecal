import test from 'node:test';
import assert from 'node:assert/strict';
import { introPlacement } from './brand-motion.ts';

test('the logo begins centered above the wordmark and ends at the header destination', () => {
  const target={x:68,y:24,width:34,height:40};
  const p=introPlacement({width:1200,height:800},target);
  assert.equal(p.x+target.width*p.scale/2,600);
  assert.equal(p.y+target.height*p.scale/2,376);
  assert.equal(p.endX,68);assert.equal(p.endY,24);
});
test('the initial logo fits on a small mobile viewport', () => {
  const p=introPlacement({width:320,height:480},{x:26,y:18,width:34,height:40});
  assert.ok(p.x>=0 && p.y>=0);
  assert.ok(p.x+34*p.scale<=320 && p.y+40*p.scale<=480);
});
test('an unmeasurable destination does not produce a broken animation', () => {
  assert.equal(introPlacement({width:320,height:480},{x:0,y:0,width:0,height:0}),null);
});
