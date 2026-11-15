// A calendar of months drawn as cells on the ramp, shared by the calendar and the offline scenes.
import { ring, rrect } from '../draw.js';
import { rampStep } from '../data.js';

const MONTHS = 'JFMAMJJASOND';

/**
 * Adds one cell per month to `scene`, popping row by row from `on`, `wave` apart. Returns where each cell sits and
 * the shapes the scenes ring or outline with.
 */
export function calendar(scene, months, { x0, y0, pitch, cell, on, wave, labels = false, flagsOn, off = Infinity }) {
  const max = Math.max(...months.map((m) => m.value));
  const centre = (row, col) => [x0 + col * pitch + cell / 2, y0 + row * pitch + cell / 2];
  if (labels) {
    [...MONTHS].forEach((letter, col) => scene.text(letter, x0 + col * pitch + cell / 2, y0 - 12, 'tiny mid', { on: on - 2 + col * 0.2, off, color: 'var(--secondary)' }));
    for (const year of [2000, 2005, 2010, 2015, 2019]) {
      scene.text(String(year), x0 - 12, y0 + (year - 2000) * pitch + cell / 2 + 6, 'tiny end', { on: on + (year - 2000) * wave, off, color: 'var(--secondary)' });
    }
  }
  const radius = Math.max(3, Math.round(cell / 5));
  for (const month of months) {
    const [x, y] = centre(month.row, month.col);
    const at = on + month.row * wave + month.col * wave * 0.08;
    scene.fill(rrect(-cell / 2, -cell / 2, cell, cell, radius), { x, y, on: at, dur: 3, off: off + month.row * 0.3, offDur: 3, color: `var(--ramp-${rampStep(month.value, max)})`, amp: cell / 40, boil: cell / 90 });
    if (month.unusual && flagsOn !== undefined) {
      const s = cell / 3.2;
      scene.fill([[0, -s], [s, 0], [0, s], [-s, 0]], { x: x + cell / 2 - s * 0.6, y: y - cell / 2 + s * 0.6, on: flagsOn, off, color: 'var(--paper)', amp: 0.3, boil: 0.2 });
      scene.fill([[0, -s * 0.7], [s * 0.7, 0], [0, s * 0.7], [-s * 0.7, 0]], { x: x + cell / 2 - s * 0.6, y: y - cell / 2 + s * 0.6, on: flagsOn + 0.5, off, color: 'var(--ink)', amp: 0.3, boil: 0.2 });
    }
  }
  return {
    centre,
    ring: () => ring(cell * 0.95, 11),
    rowOutline: (row) => rrect(x0 - 6, y0 + row * pitch - 5, 12 * pitch + 2, cell + 10, 8),
  };
}
