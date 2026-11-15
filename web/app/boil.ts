// The boil of the site and the film, only while something draws: a noise that moves what it covers by a pixel or
// two, eight times a second, and stops when nothing on the page is drawing.

/** The boil's frame rate, as in the film. */
const FRAMES_PER_SECOND = 8;
/** What is drawing: a calendar popping in or out, or a frame tracing itself. */
const DRAWING = '.entering, .leaving, .frame-drawing';

/** The filter the page's stylesheet puts on whatever is drawing. Its noise gets a new seed each frame. */
export const BOIL_FILTER = `<svg class="boil-defs" width="0" height="0" aria-hidden="true"><filter id="boil">`
  + '<feTurbulence type="fractalNoise" baseFrequency="0.04" numOctaves="2" seed="1"/>'
  + '<feDisplacementMap in="SourceGraphic" scale="2.6" xChannelSelector="R" yChannelSelector="G"/></filter></svg>';

/** Steps the noise of the filter under `root` while something on the page is drawing. */
export function startBoil(root: ParentNode): void {
  const noise = root.querySelector('#boil feTurbulence');
  if (noise === null || window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  let frame = 1;
  window.setInterval(() => {
    if (document.querySelector(DRAWING) === null) return;
    frame = (frame % 7) + 1;
    noise.setAttribute('seed', String(frame));
  }, 1000 / FRAMES_PER_SECOND);
}
