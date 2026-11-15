import { installNavigationDrawer } from './navigation-drawer.ts';
/** Screen paths follow the team's supplied mockup; each keeps the existing application content. */
export const SCREEN_PATHS = { home: '/index.html', explorer: '/explorer.html', approach: '/approach.html', data: '/data.html', about: '/about.html', support: '/support.html' } as const;
export type Screen = keyof typeof SCREEN_PATHS;
export function screenForUrl(path: string, search = ''): Screen {
  if ((path === '/' || path === '/index.html') && new URLSearchParams(search).get('v') === '1') return 'explorer';
  return (Object.keys(SCREEN_PATHS) as Screen[]).find(key => SCREEN_PATHS[key] === path) ?? 'home';
}
const HEADINGS: Record<Exclude<Screen, 'home'>, [string, string]> = {
  explorer: ['THE EXPLORER', 'Read the seasons'],
  approach: ['OUR APPROACH', 'Better context. Honest numbers.'],
  data: ['DATA & REFERENCES', 'Know what you are looking at.'],
  about: ['ABOUT THE TOOL', 'One core. Ready for the field.'],
  support: ['SUPPORT', 'Support the people protecting forests.'],
};
function page(screen: Screen): HTMLElement {
  const section = document.createElement('section'); section.dataset.screen = screen; section.className = `screen screen-${screen}`;
  if (screen !== 'home') {
    const [label, title] = HEADINGS[screen];
    const header = document.createElement('header'); header.className = 'screen-head';
    const eyebrow = document.createElement('p'); eyebrow.className = 'eyebrow'; eyebrow.textContent = label;
    const heading = document.createElement('h1'); heading.textContent = title; heading.tabIndex = -1;
    header.append(eyebrow, heading); section.append(header);
  }
  return section;
}
/** Moves existing nodes, preserving every tool ID and its event bindings. */
export function organizeScreens(root: HTMLElement): void {
  const main = root.querySelector('main')!, workspace = root.querySelector('.workspace')!, footer = root.querySelector('.page-footer')!;
  const screens = Object.fromEntries((Object.keys(SCREEN_PATHS) as Screen[]).map(key => [key, page(key)])) as Record<Screen, HTMLElement>;
  const move = (screen: Screen, selector: string): void => { const node = root.querySelector(selector); if (node) screens[screen].append(node); };
  move('home', '.intro'); move('home', '.quick-steps');
  const overview = document.createElement('section'); overview.className = 'home-overview';
  overview.innerHTML = `<div><p class="eyebrow">THE EXPLORER</p><h2>Read the seasons</h2><p>Two generations of satellites. One comparable record.</p><p>Choose a continent or draw an area.</p></div><div class="home-feature-grid"><article><h3>Uncertainty is part of the answer.</h3><p>A month is flagged only when it clears the estimated uncertainty band and the range of the baseline years.</p></article><article><h3>Take it with you.</h3><p>Save this region while you have signal. Ask new questions when you don’t.</p></article></div>`;
  screens.home.append(overview);
  const context = document.createElement('section'); context.className = 'home-context';
  context.innerHTML = `<div><p class="eyebrow">THE THINKING BEHIND THE CALENDAR</p><h2>Better context. Honest numbers.</h2><p>Different sensors see differently. We make their records comparable, and keep the uncertainty in sight.</p></div><blockquote>Analog years are historical context, never a forecast.</blockquote>`;
  screens.home.append(context);
  move('explorer', '.explorer-topline'); move('explorer', '.workspace-grid');
  move('approach', '#method'); move('approach', '#harmonization');
  move('data', '#data'); move('about', '#technology'); move('support', '#support');
  for (const screen of Object.values(screens)) main.insertBefore(screen, workspace);
  main.insertBefore(footer, workspace); workspace.remove();
}
/** In-document navigation preserves the loaded core, downloads, chosen area and comparison state. */
export function installScreens(root: HTMLElement, onExplorer: () => void): void {
  const drawer = installNavigationDrawer(root);
  const show = (focus: boolean): void => {
    drawer.close();
    const active = screenForUrl(location.pathname, location.search);
    for (const screen of root.querySelectorAll<HTMLElement>('[data-screen]')) screen.hidden = screen.dataset.screen !== active;
    root.dataset.activeScreen = active;
    for (const link of root.querySelectorAll<HTMLAnchorElement>('#main-navigation a')) {
      if (screenForUrl(new URL(link.href).pathname) === active) link.setAttribute('aria-current', 'page'); else link.removeAttribute('aria-current');
    }
    if (active === 'explorer') requestAnimationFrame(onExplorer);
    if (location.hash) {
      const target = document.getElementById(location.hash.slice(1));
      if (target instanceof HTMLDetailsElement) target.open = true;
      requestAnimationFrame(() => target?.scrollIntoView());
    } else if (focus) { window.scrollTo(0, 0); root.querySelector<HTMLElement>(`[data-screen="${active}"] h1`)?.focus({ preventScroll: true }); }
  };
  root.addEventListener('click', event => {
    if (!(event instanceof MouseEvent) || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
    const anchor = event.target instanceof Element ? event.target.closest<HTMLAnchorElement>('a[href]') : null;
    if (!anchor || anchor.target || anchor.hasAttribute('download')) return;
    const url = new URL(anchor.href);
    if (url.origin !== location.origin || !Object.values(SCREEN_PATHS).includes(url.pathname as typeof SCREEN_PATHS[Screen])) return;
    event.preventDefault();
    if (url.pathname === SCREEN_PATHS.explorer && location.search && !url.search) url.search = location.search;
    history.pushState(null, '', url); show(true);
  });
  window.addEventListener('popstate', () => show(true)); show(false);
}
