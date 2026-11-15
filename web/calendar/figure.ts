import { MONTH_NAMES, formatNumber } from './labels.ts';
import type { OverlapRow } from './overlap.ts';

const PANEL = 240;
const MARGIN = { left: 56, top: 40, bottom: 44 };
const SPACE = 48;
const WIDTH = MARGIN.left + 2 * PANEL + SPACE + 16;
const HEIGHT = MARGIN.top + PANEL + MARGIN.bottom + 36;

/** The counts the axes are marked at, on the square-root scale. */
const TICKS = [0, 100, 1000, 5000, 10000, 20000, 50000];

/**
 * Cube's overlap report as two panels on the same square-root axes, one point per tile: MODIS across, VIIRS up.
 * Before, raw VIIRS sits far above the diagonal because it sees many more small fires; after harmonization it
 * should sit on it. A square root keeps the empty tiles and spreads the small ones.
 */
export function renderOverlapSvg(rows: readonly OverlapRow[]): string {
  const largest = Math.max(1, ...rows.flatMap((row) => [row.modis, row.viirs, row.harmonized]));
  const scale = PANEL / Math.sqrt(largest);
  const at = (count: number): number => Math.sqrt(count) * scale;
  const ticks = TICKS.filter((tick) => tick <= largest);
  const panel = (left: number, title: string, points: (row: OverlapRow) => string): string => {
    const bottom = MARGIN.top + PANEL;
    const grid = ticks.map((tick) =>
      `<text class="axis" x="${left - 6}" y="${bottom - at(tick) + 4}" text-anchor="end">${formatNumber(tick)}</text>`
      + (tick === 0 ? '' : `<text class="axis" x="${left + at(tick)}" y="${bottom + 16}" text-anchor="middle">${formatNumber(tick)}</text>`)).join('');
    return `<g data-origin="${left} ${bottom}"><text class="figure-title" x="${left}" y="${MARGIN.top - 14}">${title}</text>`
      + `<path class="frame" d="M${left} ${MARGIN.top}V${bottom}H${left + PANEL}"/>`
      + `<path class="diagonal" d="M${left} ${bottom}L${left + PANEL} ${MARGIN.top}"/>${grid}`
      + rows.map((row) => points(row).replace('{x}', `${left + at(row.modis)}`)).join('') + '</g>';
  };
  const bottom = MARGIN.top + PANEL;
  const before = panel(MARGIN.left, 'Before: raw VIIRS', (row) =>
    `<circle class="before" cx="{x}" cy="${bottom - at(row.viirs)}" r="4"/>`);
  const after = panel(MARGIN.left + PANEL + SPACE, 'After: harmonized VIIRS', (row) =>
    `<circle class="after activity" cx="{x}" cy="${bottom - at(row.harmonized)}" r="4"/>`);
  return `<svg class="overlap-figure" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="${describeOverlap(rows)}" viewBox="0 0 ${WIDTH} ${HEIGHT}" width="${WIDTH}" height="${HEIGHT}">`
    + `<style>${FIGURE_STYLE}</style>` + before + after
    + `<text class="axis" x="${MARGIN.left + PANEL + SPACE / 2}" y="${bottom + 36}" text-anchor="middle">MODIS hotspots per tile, across; VIIRS, up</text>`
    + overlapFacts(rows).map((line, at) => `<text class="axis" x="0" y="${HEIGHT - 20 + at * 14}">${line}</text>`).join('')
    + '</svg>';
}

/** What the figure shows, in words: the month, the tiles, the totals, and whether the agreement means anything. */
export function describeOverlap(rows: readonly OverlapRow[]): string {
  return overlapFacts(rows).join(' ');
}

/** The figure's two lines of words: the month, tiles and totals; then what the agreement means. */
function overlapFacts(rows: readonly OverlapRow[]): string[] {
  const first = rows[0];
  if (first === undefined) return ['No overlap months.'];
  const month = `${MONTH_NAMES[first.monthIndex % 12] ?? ''} ${2000 + Math.floor(first.monthIndex / 12)}`;
  const sum = (pick: (row: OverlapRow) => number): string => formatNumber(rows.reduce((total, row) => total + pick(row), 0));
  const fit = rows.every((row) => row.training)
    ? 'fit month: the agreement is by construction, not a validation'
    : 'includes months held out of the fit';
  const facts = `${month}, ${rows.length} tiles. MODIS ${sum((row) => row.modis)}, raw VIIRS ${sum((row) => row.viirs)}, harmonized ${sum((row) => row.harmonized)}.`;
  return [facts, `${fit.charAt(0).toUpperCase()}${fit.slice(1)}.`];
}

/** The figure as a file that keeps its colors outside the page: the token values, written into the SVG. */
export function standaloneSvg(svg: string, tokens: Record<string, string>): string {
  const values = Object.entries(tokens).map(([name, value]) => `${name}: ${value};`).join(' ');
  return svg.replace(/^<svg([^>]*)>/, (open) => `${open}<style>:root, svg { ${values} }</style>`);
}

/** The figure's look, inside the SVG, so the page and a saved file draw it the same way from the tokens. */
const FIGURE_STYLE = [
  '.overlap-figure { font-family: system-ui, -apple-system, "Segoe UI", sans-serif; background: var(--bg); }',
  '.overlap-figure .axis { fill: var(--text-secondary); font-size: 11px; }',
  '.overlap-figure .figure-title { fill: var(--text-strong); font-size: 14px; font-weight: 600; }',
  '.overlap-figure .frame { fill: none; stroke: var(--disabled); stroke-width: 1px; }',
  '.overlap-figure .diagonal { fill: none; stroke: var(--line); stroke-width: 1px; }',
  '.overlap-figure .before { fill: var(--bg); stroke: var(--text-strong); stroke-width: 1.5px; }',
  '.overlap-figure .after { fill: var(--activity); stroke: var(--text-strong); stroke-width: 1px; }',
].join(' ');

/** The tokens a saved figure needs. */
export const FIGURE_TOKENS = ['--bg', '--text-strong', '--text-secondary', '--disabled', '--line', '--activity'];
