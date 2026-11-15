import { describeOverlap, overlapFacts } from './figure-words.ts';
import { formatNumber } from './labels.ts';
import type { OverlapRow } from './overlap.ts';
import { sketch, sketchLine } from './sketch.ts';

const PANEL = 240;
const MARGIN = { left: 56, top: 40, bottom: 44 };
const SPACE = 48;
const WIDTH = MARGIN.left + 2 * PANEL + SPACE + 16;
const HEIGHT = MARGIN.top + PANEL + MARGIN.bottom + 50;
/** The least room between two labels of an axis. */
const LABEL_ROOM = 28;

/** The counts the axes are marked at, on the square-root scale. */
const TICKS = [0, 100, 1000, 5000, 10000, 20000, 50000, 100000];

/**
 * Cube's overlap report as two panels on the same square-root axes, one point per tile: MODIS across, VIIRS up.
 * Before, raw VIIRS sits far above the diagonal because it sees many more small fires; after harmonization it
 * should sit on it. A square root keeps the empty tiles and spreads the small ones.
 */
export function renderOverlapSvg(rows: readonly OverlapRow[]): string {
  const largest = Math.max(1, ...rows.flatMap((row) => [row.modis, row.viirs, row.harmonized]));
  const scale = PANEL / Math.sqrt(largest);
  const at = (count: number): number => Math.sqrt(count) * scale;
  const ticks = TICKS.filter((tick) => tick <= largest).reduce<number[]>((kept, tick) => {
    const previous = kept.at(-1);
    return previous === undefined || at(tick) - at(previous) >= LABEL_ROOM ? [...kept, tick] : kept;
  }, []);
  const panel = (left: number, title: string, points: (row: OverlapRow) => string): string => {
    const bottom = MARGIN.top + PANEL;
    const grid = ticks.map((tick) =>
      `<text class="axis" x="${left - 6}" y="${bottom - at(tick) + 4}" text-anchor="end">${formatNumber(tick)}</text>`
      + (tick === 0 ? '' : `<text class="axis" x="${left + at(tick)}" y="${bottom + 16}" text-anchor="middle">${formatNumber(tick)}</text>`)).join('');
    return `<g data-origin="${left} ${bottom}"><text class="figure-title" x="${left}" y="${MARGIN.top - 14}">${title}</text>`
      + `<path class="frame" d="${sketch([[left, MARGIN.top], [left, bottom], [left + PANEL, bottom]], false, left, 1)}"/>`
      + `<path class="diagonal" d="${sketchLine([left, bottom], [left + PANEL, MARGIN.top], left + 1, 1)}"/>${grid}`
      + rows.map((row) => points(row).replace('{x}', `${left + at(row.modis)}`)).join('') + '</g>';
  };
  const bottom = MARGIN.top + PANEL;
  const before = panel(MARGIN.left, 'Before: raw VIIRS', (row) =>
    `<circle class="before" cx="{x}" cy="${bottom - at(row.viirs)}" r="4"/>`);
  const after = panel(MARGIN.left + PANEL + SPACE, 'After: harmonized VIIRS', (row) => row.training
    ? `<circle class="after activity" cx="{x}" cy="${bottom - at(row.harmonized)}" r="4"/>`
    : `<rect class="after held-out activity" x="{x}" y="${bottom - at(row.harmonized) - 3.5}" width="7" height="7" transform="translate(-3.5 0)"/>`);
  return `<svg class="overlap-figure" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="${describeOverlap(rows)}" viewBox="0 0 ${WIDTH} ${HEIGHT}" width="${WIDTH}" height="${HEIGHT}">`
    + `<style>${FIGURE_STYLE}</style>` + before + after
    + `<text class="axis" x="${MARGIN.left + PANEL + SPACE / 2}" y="${bottom + 36}" text-anchor="middle">MODIS hotspots per tile and month, across; VIIRS, up${rows.some((row) => !row.training) ? '. Squares: held out of the fit' : ''}</text>`
    + overlapFacts(rows).map((line, at) => `<text class="axis" x="0" y="${HEIGHT - 34 + at * 14}">${line}</text>`).join('')
    + '</svg>';
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
  '.overlap-figure .frame { fill: none; stroke: var(--text-strong); stroke-width: 2.5px; stroke-linecap: round; stroke-linejoin: round; }',
  '.overlap-figure .diagonal { fill: none; stroke: var(--disabled); stroke-width: 1.5px; stroke-linecap: round; stroke-dasharray: 2 5; }',
  '.overlap-figure .before { fill: var(--bg); stroke: var(--text-strong); stroke-width: 1.5px; }',
  '.overlap-figure .after { fill: var(--activity); stroke: var(--text-strong); stroke-width: 1px; }',
].join(' ');

/** The tokens a saved figure needs. */
export const FIGURE_TOKENS = ['--bg', '--text-strong', '--text-secondary', '--disabled', '--line', '--activity'];
