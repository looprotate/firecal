// Hand-drawn outlines for the page: every panel, field and button gets a rounded rectangle traced by the hand of the
// site (`site/js/draw.js`), sized in real pixels and redrawn when the element changes size. A frame traces itself the
// first time it shows, then stays still.
import { path, resample, rough, rrect } from '../../site/js/draw.js';

const SVG = 'http://www.w3.org/2000/svg';
/** How long a new frame takes to trace, in milliseconds; the boil runs while it does. */
const TRACE_MS = 700;

/** How a kind of frame is drawn: its corner radius and the wobble of the hand, in pixels. */
interface FrameKind {
  radius: number;
  wobble: number;
}

const BUTTON: FrameKind = { radius: 7, wobble: 0.8 };
const KINDS: Record<string, FrameKind> = {
  panel: { radius: 12, wobble: 1.4 },
  map: { radius: 8, wobble: 1.1 },
  field: { radius: 6, wobble: 0.7 },
  button: BUTTON,
};

/** What gets a frame: whatever asks for one, and every button except the map's own controls. */
const FRAMED = '[data-frame], button:not(.maplibregl-ctrl button)';

/** A seed from what the element says, so a button drawn again after a redraw keeps its wobble. */
function seedOf(element: HTMLElement): number {
  let seed = 17;
  for (const char of `${element.dataset.frame ?? element.tagName}${element.textContent ?? ''}`) seed = (seed * 31 + (char.codePointAt(0) ?? 0)) % 100_003;
  return seed / 97;
}

/** Draws a frame on every element under `root` that needs one, now and whenever one is added. */
export function drawFrames(root: HTMLElement): void {
  const sizes = new ResizeObserver((entries) => {
    for (const entry of entries) outline(entry.target as HTMLElement);
  });
  const attach = (element: HTMLElement): void => {
    if (element.dataset.framed !== undefined) return;
    element.dataset.framed = '';
    const svg = document.createElementNS(SVG, 'svg');
    svg.setAttribute('class', 'frame-art');
    svg.setAttribute('aria-hidden', 'true');
    const shape = svg.appendChild(document.createElementNS(SVG, 'path'));
    shape.setAttribute('pathLength', '1');
    element.prepend(svg);
    element.classList.add('frame-drawing');
    window.setTimeout(() => element.classList.remove('frame-drawing'), TRACE_MS);
    sizes.observe(element);
  };
  const scan = (under: ParentNode): void => {
    if (under instanceof HTMLElement && under.matches(FRAMED)) attach(under);
    for (const element of under.querySelectorAll<HTMLElement>(FRAMED)) attach(element);
  };
  scan(root);
  new MutationObserver((changes) => {
    for (const change of changes) for (const node of change.addedNodes) if (node instanceof HTMLElement) scan(node);
  }).observe(root, { childList: true, subtree: true });
}

function outline(element: HTMLElement): void {
  const svg = element.querySelector(':scope > .frame-art');
  const shape = svg?.firstElementChild;
  const width = element.offsetWidth;
  const height = element.offsetHeight;
  if (!svg || !shape || width === 0 || height === 0) return;
  const kind = KINDS[element.dataset.frame ?? 'button'] ?? BUTTON;
  const inset = 1.5;
  const radius = Math.min(kind.radius, height / 2 - inset);
  const points = rrect(inset, inset, width - 2 * inset, height - 2 * inset, radius);
  svg.setAttribute('viewBox', `0 0 ${width} ${height}`);
  shape.setAttribute('d', path(rough(resample(points, true), seedOf(element), kind.wobble, 0, 0), true));
}
