import test from 'node:test';
import assert from 'node:assert/strict';
import { CONTINENTS, continentArea, renderContinentPicker } from './continents.ts';

test('six distinct countries and capitals cover the requested inhabited continents, excluding Central America', () => {
  assert.deepEqual(CONTINENTS.map(({country,capital})=>[country,capital]), [
    ['Canada','Ottawa'],['Brazil','Brasília'],['China','Beijing'],['Russia','Moscow'],['Algeria','Algiers'],['Australia','Canberra'],
  ]);
  assert.equal(new Set(CONTINENTS.map(place=>place.capital)).size,6);
  assert.ok(!CONTINENTS.some(place=>/central/i.test(place.id)));
});
test('every viewing area contains its capital at the center and remains within valid coordinates', () => {
  for (const place of CONTINENTS) {
    const area=continentArea(place.id);
    assert.ok(Math.abs((area.west+area.east)/2-place.longitude)<1e-8);
    assert.ok(Math.abs((area.south+area.north)/2-place.latitude)<1e-8);
    assert.equal(Math.round(area.east-area.west),4);
    assert.equal(Math.round(area.north-area.south),4);
    assert.ok(area.west>=-180 && area.east<=180 && area.south>=-90 && area.north<=90);
  }
  assert.equal(continentArea('central-america'),null);
  assert.equal(continentArea('antarctica'),null);
});
test('Antarctica is explicitly unavailable because it has no national capital', () => {
  assert.match(renderContinentPicker(), /value="antarctica" disabled/);
});

test('all six dropdown choices show their continent, country and capital together', () => {
  const html=renderContinentPicker();
  assert.match(html, /<select id="continent-select">/);
  assert.equal((html.match(/<option /g) ?? []).length,8);
  for (const place of CONTINENTS) {
    assert.match(html,new RegExp(`value="${place.id}"`));
    assert.ok(html.includes(place.country));
    assert.ok(html.includes(place.capital));
  }
});
