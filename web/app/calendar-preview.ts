import type { CalendarRenderer } from './shell.ts';

/** A small, accessible preview used while the owned calendar package is still a stub. */
export const previewCalendar: CalendarRenderer = {
  mount(host, series) {
    const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
    const rows = series.months.map((index, at) => {
      const value = series.value[at];
      const year = 2000 + Math.floor(index / 12);
      const month = months[index % 12];
      return `<tr><th scope="row">${month} ${year}</th><td>${value ?? '—'}</td><td>${series.sigma[at] ?? '—'}</td></tr>`;
    }).join('');
    host.innerHTML = `
      <table class="calendar-preview-table">
        <caption>Hand-made sample series for ${series.area.west}, ${series.area.south} to ${series.area.east}, ${series.area.north}</caption>
        <thead><tr><th scope="col">Month</th><th scope="col">Hotspots</th><th scope="col">Sigma</th></tr></thead>
        <tbody>${rows}</tbody>
      </table>`;
  },
};
