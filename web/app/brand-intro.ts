import { introPlacement } from './brand-motion.ts';
import { translateText, type Language } from './translations.ts';

/** A short, skippable introduction that animates the actual header image on every page entry. */
export async function installBrandIntro(root: HTMLElement): Promise<void> {
  const logo = root.querySelector<HTMLImageElement>('#brand-logo');
  const reduced = matchMedia('(prefers-reduced-motion: reduce)');
  if (!logo || reduced.matches || typeof logo.animate !== 'function') return;
  const placement = introPlacement({ width: innerWidth, height: innerHeight }, logo.getBoundingClientRect());
  if (!placement) return;
  const layer = document.createElement('div');
  layer.className = 'brand-intro';
  layer.setAttribute('aria-label', 'firecal');
  const wash = document.createElement('div');
  wash.className = 'brand-intro-background';
  const title = document.createElement('p');
  title.className = 'brand-intro-wordmark';
  title.textContent = 'firecal';
  const skip = document.createElement('button');
  skip.type = 'button';
  skip.className = 'brand-intro-skip';
  const language = document.documentElement.lang.slice(0, 2) as Language;
  skip.textContent = translateText('Skip intro', language);
  layer.append(wash, title, skip);
  document.body.append(layer);
  const savedStyle = logo.style.cssText;
  const home = logo.parentNode;
  const nextSibling = logo.nextSibling;
  const wasInert = root.inert;
  const overflow = document.body.style.overflow;
  root.inert = true;
  document.body.style.overflow = 'hidden';
  let loadTimeout: ReturnType<typeof setTimeout> | undefined;
  let finishWaiting: (() => void) | undefined;
  let motion: Animation | null = null;
  let finished = false;
  const end = (): void => {
    if (finished) return;
    finished = true;
    const skippedByKeyboard = document.activeElement === skip;
    if (loadTimeout !== undefined) clearTimeout(loadTimeout);
    finishWaiting?.();
    motion?.cancel();
    logo.style.cssText = savedStyle;
    home?.insertBefore(logo, nextSibling);
    layer.remove();
    root.inert = wasInert;
    document.body.style.overflow = overflow;
    document.removeEventListener('keydown', onKey);
    window.removeEventListener('resize', end);
    window.removeEventListener('pagehide', end);
    reduced.removeEventListener('change', end);
    if (skippedByKeyboard) root.querySelector<HTMLAnchorElement>('.topbar nav a')?.focus({ preventScroll: true });
  };
  const onKey = (event: KeyboardEvent): void => { if (event.key === 'Escape') end(); };
  skip.addEventListener('click', end);
  document.addEventListener('keydown', onKey);
  window.addEventListener('resize', end);
  window.addEventListener('pagehide', end);
  reduced.addEventListener('change', end);
  try {
    await Promise.race([logo.decode().catch(() => undefined), new Promise<void>((resolve) => {
      finishWaiting = resolve;
      loadTimeout = setTimeout(resolve, 5000);
    })]);
    if (loadTimeout !== undefined) clearTimeout(loadTimeout);
    if (finished) return;
    if (!logo.complete || logo.naturalWidth === 0) { end(); return; }
    const rect = logo.getBoundingClientRect();
    layer.append(logo);
    Object.assign(logo.style, { position: 'fixed', left: '0px', top: '0px', width: `${rect.width}px`, height: `${rect.height}px`, transformOrigin: '0 0', zIndex: '10020', pointerEvents: 'none' });
    const start = `translate(${placement.x}px, ${placement.y}px) scale(${placement.scale})`;
    const target = `translate(${placement.endX}px, ${placement.endY}px) scale(1)`;
    const duration = 1850;
    motion = logo.animate([
      { transform: start, opacity: 0, offset: 0 },
      { transform: start, opacity: 1, offset: 0.14 },
      { transform: start, opacity: 1, offset: 0.38 },
      { transform: target, opacity: 1, offset: 1 },
    ], { duration, easing: 'cubic-bezier(.22,.7,.23,1)', fill: 'forwards' });
    wash.animate([{ opacity: 1, offset: 0 }, { opacity: 1, offset: 0.53 }, { opacity: 0, offset: 1 }], { duration, fill: 'forwards' });
    title.animate([{ opacity: 1 }, { opacity: 1, offset: 0.3 }, { opacity: 0, offset: 0.6 }, { opacity: 0 }], { duration, fill: 'forwards' });
    await motion.finished.catch(() => undefined);
  } finally { end(); }
}
