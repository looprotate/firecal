import test from 'node:test';
import assert from 'node:assert/strict';
import { translateText, LANGUAGES } from './translations.ts';

test('English is the base and all five requested languages are available', () => {
  assert.deepEqual(LANGUAGES, ['en', 'es', 'de', 'fr', 'zh']);
  assert.equal(translateText('Choose your area', 'en'), 'Choose your area');
});
test('each language translates the area control without changing coordinates', () => {
  const expected = { es: 'Elige tu zona', de: 'Gebiet auswählen', fr: 'Choisissez votre zone', zh: '选择区域' };
  for (const [language, text] of Object.entries(expected)) {
    assert.equal(translateText('Choose your area', language), text);
    assert.equal(translateText('147, -38, 151, -34', language), '147, -38, 151, -34');
  }
});
test('dynamic calendar descriptions keep values, uncertainty and flags in Spanish', () => {
  assert.equal(translateText('December 2019: 7,112 hotspots, uncertainty ±14 (estimated), unusual', 'es'),
    'Diciembre 2019: 7,112 puntos calientes, incertidumbre ±14 (estimada), inusual');
});
test('machine-readable file names and satellite names remain unchanged', () => {
  for (const language of LANGUAGES) assert.equal(translateText('MODIS + VIIRS · series.csv · cells.csv', language), 'MODIS + VIIRS · series.csv · cells.csv');
});
test('a phrase is translated once, including text around scientific numbers', () => {
  assert.equal(translateText('2019 against the median of past years', 'fr'), '2019 par rapport à la médiane des années précédentes');
  assert.equal(translateText('From the core, on 2 pinned tiles, 230 months.', 'es'), 'Calculado en este dispositivo con 2 bloques guardados, 230 meses.');
});
