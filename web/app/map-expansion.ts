/** Expands the existing map rather than creating a second map or losing the selection. */
export function installMapExpansion(root: HTMLElement, map: { resize(): unknown }): void {
  const dialog = root.querySelector<HTMLDialogElement>('#map-dialog');
  const destination = root.querySelector<HTMLElement>('#map-dialog-content');
  const unit = root.querySelector<HTMLElement>('.map-unit');
  const button = root.querySelector<HTMLButtonElement>('#map-expand');
  const close = root.querySelector<HTMLButtonElement>('#map-dialog-close');
  const label = root.querySelector<HTMLElement>('#map-expand-label');
  if (!dialog || !destination || !unit || !button || !close || !label) return;
  const anchor = document.createComment('Map returns here when the expanded view closes');
  unit.before(anchor);
  let previousOverflow = '';
  const resize = (): void => { requestAnimationFrame(() => map.resize()); };
  button.addEventListener('click', () => {
    if (dialog.open) { dialog.close(); return; }
    previousOverflow = document.body.style.overflow;
    destination.append(unit);
    dialog.showModal();
    document.body.style.overflow = 'hidden';
    button.setAttribute('aria-expanded', 'true');
    label.textContent = 'Restore map';
    close.focus();
    resize();
  });
  close.addEventListener('click', () => dialog.close());
  dialog.addEventListener('close', () => {
    anchor.after(unit);
    document.body.style.overflow = previousOverflow;
    button.setAttribute('aria-expanded', 'false');
    label.textContent = 'Expand map';
    resize();
    button.focus({ preventScroll: true });
  });
  dialog.addEventListener('click', (event) => {
    if (event.target !== dialog) return;
    const bounds = dialog.getBoundingClientRect();
    if (event.clientX < bounds.left || event.clientX > bounds.right || event.clientY < bounds.top || event.clientY > bounds.bottom) dialog.close();
  });
}
