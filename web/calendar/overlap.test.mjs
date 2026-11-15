import assert from 'node:assert/strict';
import { test } from 'node:test';
import { renderOverlapSvg, standaloneSvg } from './figure.ts';
import { parseOverlapCsv } from './overlap.ts';

const CSV = `tile_row,tile_col,year,month,month_index,modis_satellite,viirs_satellite,split,raw_modis_count,raw_viirs_count,harmonized_viirs_count
8,10,2026,7,318,0,2,training,100,900,100.000000
8,11,2026,7,318,0,2,training,0,0,0.000000
9,10,2026,7,318,0,2,training,400,4000,399.999999
`;

test('the overlap report is read by its header, one row per tile and month', () => {
  const rows = parseOverlapCsv(CSV);
  assert.equal(rows.length, 3);
  assert.deepEqual(rows[0], { tileRow: 8, tileCol: 10, monthIndex: 318, training: true, modis: 100, viirs: 900, harmonized: 100 });
});

test('a file that is not the overlap report is refused, not drawn wrong', () => {
  assert.throws(() => parseOverlapCsv('year,month,value\n2026,7,1\n'), /overlap report/);
  assert.throws(() => parseOverlapCsv(CSV.replace('900', 'nine hundred')), /row 1/);
});

test('the figure puts every tile in both panels, before as rings and after as dots, with one diagonal each', () => {
  const svg = renderOverlapSvg(parseOverlapCsv(CSV));
  assert.equal(svg.match(/class="before"/g)?.length, 3);
  assert.equal(svg.match(/class="after activity"/g)?.length, 3);
  assert.equal(svg.match(/class="diagonal"/g)?.length, 2);
  assert.match(svg, /Before: raw VIIRS/);
  assert.match(svg, /After: harmonized VIIRS/);
});

test('on a square-root scale, a tile with nine times the MODIS count sits three times as high', () => {
  const svg = renderOverlapSvg(parseOverlapCsv(CSV));
  const before = [...svg.matchAll(/<circle class="before" cx="([\d.]+)" cy="([\d.]+)"/g)].map((m) => [Number(m[1]), Number(m[2])]);
  const [origin] = [...svg.matchAll(/data-origin="([\d.]+) ([\d.]+)"/g)].map((m) => [Number(m[1]), Number(m[2])]);
  // Tile 8/10: MODIS 100 and VIIRS 900, so VIIRS is three times as far up as MODIS is across.
  const across = before[0][0] - origin[0];
  const up = origin[1] - before[0][1];
  assert.ok(Math.abs(up - 3 * across) < 0.01, `${up} is not three times ${across}`);
});

test('a fit month says that its agreement is by construction, not a validation', () => {
  assert.match(renderOverlapSvg(parseOverlapCsv(CSV)), /Fit month: the agreement is by construction, not a validation/);
});

test('the image saved for the pitch carries its colors, since the tokens are not there', () => {
  const svg = standaloneSvg(renderOverlapSvg(parseOverlapCsv(CSV)), { '--activity': '#d77656', '--text': '#232321' });
  assert.match(svg, /^<svg[^>]*xmlns="http:\/\/www.w3.org\/2000\/svg"/);
  assert.match(svg, /<style>:root, svg \{ --activity: #d77656; --text: #232321; \}<\/style>/);
});
