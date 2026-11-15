import { FIGURE_TOKENS, renderOverlapSvg, standaloneSvg } from './figure.ts';
import { parseOverlapCsv } from './overlap.ts';
import { download } from './toolbar.ts';

/**
 * Draws the before and after harmonization figure from the text of Cube's overlap report into `container`, with
 * a button that saves it as an SVG for the pitch. A file that is not the report is said so, in place of a figure.
 */
export function mountOverlapFigure(container: HTMLElement, reportText: string): void {
  let svg: string;
  try {
    svg = renderOverlapSvg(parseOverlapCsv(reportText));
  } catch (error) {
    container.textContent = error instanceof Error ? error.message : 'The overlap report could not be read.';
    return;
  }
  const figure = document.createElement('figure');
  figure.className = 'overlap';
  figure.innerHTML = svg;
  const save = document.createElement('button');
  save.type = 'button';
  save.className = 'overlap-save';
  save.textContent = 'Save as SVG';
  save.addEventListener('click', () => {
    const style = getComputedStyle(document.documentElement);
    const tokens = Object.fromEntries(FIGURE_TOKENS.map((name) => [name, style.getPropertyValue(name).trim()]));
    download('harmonization-before-after.svg', new TextEncoder().encode(standaloneSvg(svg, tokens)), 'image/svg+xml');
  });
  container.replaceChildren(figure, save);
}
