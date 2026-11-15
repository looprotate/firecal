import { CELL } from './layout.ts';
import { provisionalTriangle, unusualDiamond } from './marks.ts';
import { RAMP_STEPS } from './ramp.ts';
import { sketchRect } from './sketch.ts';

const SWATCH = 14;
const GAP = 2;

const glyph = (mark: string): string =>
  `<svg class="legend-glyph" viewBox="0 0 ${CELL} ${CELL}" width="${CELL}" height="${CELL}" aria-hidden="true">`
  + `<path class="sample ramp-3" d="${sketchRect(0, 0, CELL, CELL, 6, 900, CELL / 40)}"/>${mark}</svg>`;

/**
 * The key under the calendar, drawn from the same shapes and classes as the cells: the empty cell and the seven
 * steps from none to most, then a cell with each mark and what it means in words. HTML around small drawings, so
 * the words wrap and keep their size on a narrow screen.
 */
export function renderLegend(): string {
  const width = (RAMP_STEPS + 1) * (SWATCH + GAP) - GAP;
  const steps = Array.from({ length: RAMP_STEPS + 1 }, (_, step) =>
    `<path class="swatch ramp-${step}" d="${sketchRect(step * (SWATCH + GAP), 0, SWATCH, SWATCH, 3, 910 + step, 0.4)}"/>`,
  ).join('');
  return '<div class="calendar-legend" role="list" aria-label="Legend">'
    + `<span role="listitem">None <svg viewBox="0 0 ${width} ${SWATCH}" width="${width}" height="${SWATCH}" aria-hidden="true">${steps}</svg> Most</span>`
    + `<span role="listitem">${glyph(unusualDiamond(0, 0))}Unusual: above the band and every past year</span>`
    + `<span role="listitem">${glyph(provisionalTriangle(0, 0))}Provisional: near-real-time data</span>`
    + '</div>';
}
