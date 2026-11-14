import assert from 'node:assert/strict';
import { test } from 'node:test';
import { degreesText, latitudeLabel, longitudeLabel } from './degrees.ts';

test('a coordinate is written to a thousandth of a degree, the cell is a quarter', () => {
  assert.equal(degreesText(-58.070660542), '-58.071');
  assert.equal(degreesText(-62), '-62');
  assert.equal(degreesText(0.25), '0.25');
  assert.equal(degreesText(-0.0001), '0');
});

test('labels name the hemisphere instead of a sign', () => {
  assert.equal(latitudeLabel(-16.5), '16.5° S');
  assert.equal(latitudeLabel(10), '10° N');
  assert.equal(longitudeLabel(-58.0706), '58.071° W');
  assert.equal(longitudeLabel(0), '0° E');
});
