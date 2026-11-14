import { RAMP_STEPS } from './ramp.ts';
import { CELL, provisionalTriangle } from './render.ts';

const SWATCH = 14;
const GAP = 2;
const HEIGHT = CELL + 2;
const MIDDLE = HEIGHT / 2;

/**
 * The key under the calendar, drawn from the same shapes and classes as the cells: the empty cell and the seven
 * steps from none to most, then a cell with each mark and what it means in words.
 */
export function renderLegendSvg(): string {
  const top = MIDDLE - SWATCH / 2;
  const steps = Array.from({ length: RAMP_STEPS + 1 }, (_, step) =>
    `<rect class="swatch ramp-${step}" x="${36 + step * (SWATCH + GAP)}" y="${top}" width="${SWATCH}" height="${SWATCH}" rx="3"/>`,
  ).join('');
  const most = 36 + (RAMP_STEPS + 1) * (SWATCH + GAP) + 4;
  const mark = most + 52;
  const width = mark + CELL + 8 + 200;
  return `<svg class="calendar-legend" role="img" aria-label="Legend: the darker the cell, the more activity; a triangle in the corner marks a provisional month" viewBox="0 0 ${width} ${HEIGHT}" width="${width}" height="${HEIGHT}">`
    + `<text class="axis" x="0" y="${MIDDLE + 4}">None</text>${steps}<text class="axis" x="${most}" y="${MIDDLE + 4}">Most</text>`
    + `<rect class="sample ramp-3" x="${mark}" y="1" width="${CELL}" height="${CELL}" rx="6"/>${provisionalTriangle(mark, 1)}`
    + `<text class="axis" x="${mark + CELL + 8}" y="${MIDDLE + 4}">Provisional: near-real-time data</text></svg>`;
}
