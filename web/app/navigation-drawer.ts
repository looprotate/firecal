/** A native modal keeps keyboard focus inside the right-hand navigation drawer. */
export function installNavigationDrawer(root: HTMLElement): { close(): void } {
  const toggle = root.querySelector<HTMLButtonElement>('.menu-toggle')!;
  const navigation = root.querySelector<HTMLElement>('#main-navigation')!;
  const dialog = root.querySelector<HTMLDialogElement>('#navigation-drawer')!;
  const slot = dialog.querySelector<HTMLElement>('.navigation-drawer-content')!;
  const closeButton = dialog.querySelector<HTMLButtonElement>('.navigation-drawer-close')!;
  const anchor = document.createComment('Desktop navigation returns here');
  navigation.before(anchor);
  let previousOverflow = '';
  let presented = false;
  const close = (): void => {
    if (!presented) return;
    presented = false;
    if (dialog.open) dialog.close();
    anchor.after(navigation);
    document.body.style.overflow = previousOverflow;
    toggle.setAttribute('aria-expanded', 'false');
  };
  toggle.addEventListener('click', () => {
    if (presented) { close(); return; }
    previousOverflow = document.body.style.overflow;
    slot.append(navigation);
    dialog.showModal();
    presented = true;
    document.body.style.overflow = 'hidden';
    toggle.setAttribute('aria-expanded', 'true');
    closeButton.focus();
  });
  closeButton.addEventListener('click', close);
  dialog.addEventListener('cancel', event => { event.preventDefault(); close(); });
  dialog.addEventListener('close', () => { if (!dialog.open) close(); });
  dialog.addEventListener('click', event => {
    if (event.target !== dialog) return;
    const bounds = dialog.getBoundingClientRect();
    if (event.clientX < bounds.left || event.clientX > bounds.right || event.clientY < bounds.top || event.clientY > bounds.bottom) close();
  });
  matchMedia('(max-width: 1100px)').addEventListener('change', event => { if (!event.matches) close(); });
  window.addEventListener('pagehide', close);
  return { close };
}
