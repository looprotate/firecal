import { CELL } from './layout.ts';
import { provisionalTriangle, unusualDiamond } from './marks.ts';
import { RAMP_STEPS } from './ramp.ts';

const SWATCH = 14;
const GAP = 2;

const glyph = (mark: string): string =>
  `<svg class="legend-glyph" viewBox="0 0 ${CELL} ${CELL}" width="${CELL}" height="${CELL}" aria-hidden="true">`
  + `<rect class="sample ramp-3" width="${CELL}" height="${CELL}" rx="6"/>${mark}</svg>`;

/**
 * The key under the calendar, drawn from the same shapes and classes as the cells: the empty cell and the seven
 * steps from none to most, then a cell with each mark and what it means in words. HTML around small drawings, so
 * the words wrap and keep their size on a narrow screen.
 */
export function renderLegend(): string {
  const width = (RAMP_STEPS + 1) * (SWATCH + GAP) - GAP;
  const steps = Array.from({ length: RAMP_STEPS + 1 }, (_, step) =>
    `<rect class="swatch ramp-${step}" x="${step * (SWATCH + GAP)}" y="0" width="${SWATCH}" height="${SWATCH}" rx="3"/>`,
  ).join('');
  return '<div class="calendar-legend" role="list" aria-label="Legend">'
    + `<span role="listitem">None <svg viewBox="0 0 ${width} ${SWATCH}" width="${width}" height="${SWATCH}" aria-hidden="true">${steps}</svg> Most</span>`
    + `<span role="listitem">${glyph(unusualDiamond(0, 0))}Unusual: outside the band and the past years</span>`
    + `<span role="listitem">${glyph(provisionalTriangle(0, 0))}Provisional: near-real-time data</span>`
    + '</div>';
}
