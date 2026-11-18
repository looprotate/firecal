// The six slides of the defence (pitch/defensa.md), drawn with the hand of the site and the film. Each slide is a
// timeline from 0 to 100 and the stops a click moves it to; every number is the app's or pitch/numbers.md's.
import { arc, circle, clamp, inOut, rrect } from '../../site/js/draw.js';
import { analogs, demoArea, heldOut } from '../../site/js/data.js';
import { calendar } from '../../site/js/scenes/cells.js';
import * as logo from '../../site/js/scenes/logo.js';

/** Numbers as they are said in Montevideo: a dot between thousands, a comma for decimals. */
const es = (value) => Math.round(value).toLocaleString('es-UY');

const ink = 'var(--ink)';
const second = 'var(--secondary)';

/** 0:00, Axel. Two numbers for the same place and year, then the question that stays open until the close. */
function hook(scene) {
  scene.text(es(heldOut.modis), 800, 330, 'huge mid', { on: 4, dur: 8, color: ink });
  scene.text(es(heldOut.viirsRaw), 800, 610, 'huge mid', { on: 26, dur: 8, color: ink });

  scene.text('Sudeste de Australia · 2019', 800, 120, 'title mid', { on: 46, color: second });
  scene.text('MODIS · desde el 2000', 800, 385, 'lede mid', { on: 48, color: second });
  scene.text('VIIRS · desde el 2012', 800, 665, 'lede mid', { on: 50, color: second });
  scene.line(arc(1170, 455, 90, 150, -Math.PI / 2, Math.PI / 2, 24), { on: 54, dur: 8, w: 5, color: 'var(--activity)', amp: 1.2, boil: 0.6 });
  scene.line([[1192, 588], [1170, 605], [1194, 622]], { on: 61, dur: 2, w: 5, color: 'var(--activity)', amp: 0.6 });
  scene.text('× 12,5', 1290, 470, 'big', { on: 60, color: 'var(--activity)' });

  scene.text('¿A cuál le creen?', 800, 820, 'big mid', { on: 76, dur: 6, color: ink });
}

/** 0:40, Axel. What firecal is, in three lines. */
function name(scene) {
  const mark = scene.svg.appendChild(document.createElementNS('http://www.w3.org/2000/svg', 'g'));
  mark.setAttribute('transform', 'translate(420 700)');
  logo.build(scene, 0.62, 2);
  scene.items.slice(-4).forEach((item) => mark.appendChild(item.node));
  scene.text('firecal', 640, 470, 'huge', { on: 14, dur: 8, color: ink });
  scene.text('Un calendario de quema para cualquier zona.', 650, 560, 'lede', { on: 24, color: ink });
  scene.text('Dos satélites de la NASA, puestos de acuerdo.', 650, 610, 'lede', { on: 32, color: ink });
  scene.text('Cada número dice cuánto confiar en él.', 650, 660, 'lede strong', { on: 40, color: ink });
  scene.text('Seis uruguayos · NASA Space Apps 2026', 650, 760, 'note', { on: 52, color: second });
}

/** 1:05, Kevin. The calendar of the demo area; the live app is one key away (D). */
function demo(scene) {
  const X0 = 250;
  const Y0 = 150;
  const PITCH = 31;
  const grid = calendar(scene, demoArea, { x0: X0, y0: Y0, pitch: PITCH, cell: 27, on: 4, wave: 1.4, labels: true, flagsOn: 34 });
  scene.text('Sudeste de Australia, 2000 a 2019', 760, 190, 'title', { on: 6, color: ink });
  scene.text('Meses a lo ancho, años hacia abajo.', 760, 236, 'lede', { on: 10, color: second });

  const [x, y] = grid.centre(19, 11);
  scene.line(grid.ring(), { x, y, on: 44, dur: 6, w: 5, color: 'var(--attention)', amp: 0.8, boil: 0.5 });
  scene.line([[x + 22, y - 16], [x + 80, y - 100], [x + 136, y - 196]], { on: 48, dur: 4, w: 4, color: 'var(--attention)', amp: 1 });
  scene.text('7.112 ± 88', 760, 520, 'big', { on: 50, color: ink });
  scene.text('focos de calor en diciembre de 2019', 762, 566, 'lede', { on: 52, color: second });
  scene.text('la mediana de un diciembre acá: 7', 762, 606, 'lede', { on: 54, color: second });
  scene.text('Inusual: supera su banda y todos los años anteriores.', 762, 660, 'note strong', { on: 58, color: ink });

  analogs.forEach(([year], rank) => {
    const on = 72 + rank * 4;
    scene.line(grid.rowOutline(year - 2000), { on, dur: 5, w: 4, closed: true, amp: 0.8, boil: 0.5 });
    scene.text(`${rank + 1}`, X0 + 12 * PITCH + 30, Y0 + (year - 2000) * PITCH + 21, 'note mid strong', { on: on + 3, color: ink });
  });
  scene.text('Los años más parecidos: 2009, 2001 y 2002.', 762, 740, 'lede', { on: 82, color: ink });
  scene.text('Historia, no un pronóstico.', 762, 782, 'lede', { on: 86, color: second });
}

/** 2:20, Bayron. 2019 held out of the fit: raw VIIRS 12.5 times MODIS, then harmonized down to 0.77 times. */
function agree(scene) {
  const BASE = 790;
  const TALL = 560;
  const height = (count) => (TALL * count) / heldOut.viirsRaw;
  const viirs = (t) => {
    const up = inOut(clamp((t - 12) / 14));
    const down = inOut(clamp((t - 52) / 16));
    return up * (heldOut.viirsRaw + (heldOut.viirsHarmonized - heldOut.viirsRaw) * down);
  };
  const modis = (t) => heldOut.modis * inOut(clamp((t - 8) / 8));
  const bar = (x, count, color, on) => {
    const shape = (t) => {
      const h = Math.max(1, height(count(t)));
      return rrect(x - 85, BASE - h, 170, h, Math.min(8, h / 2));
    };
    scene.fill(shape, { anim: 'fixed', on, dur: 1, color, amp: 1, boil: 0.5 });
    scene.line(shape, { anim: 'fixed', on, dur: 1, closed: true, w: 4, amp: 1, boil: 0.5 });
  };
  scene.text('2019, fuera del entrenamiento: el modelo nunca lo vio.', 800, 110, 'title mid', { on: 2, color: ink });
  scene.line([[300, BASE], [1300, BASE]], { on: 2, dur: 6, w: 4, amp: 1.2, boil: 0.5 });

  bar(560, modis, 'var(--panel)', 8);
  bar(860, viirs, 'var(--fire)', 12);
  const above = (x, count) => (t) => ({ x, y: BASE - height(count(t)) - 22 });
  scene.text((t) => es(modis(t)), 0, 0, 'title mid', { at: above(560, modis), on: 10, color: ink });
  scene.text((t) => es(viirs(t)), 0, 0, 'title mid', { at: above(860, viirs), on: 14, color: ink });
  scene.text('MODIS', 560, BASE + 42, 'lede mid', { on: 8, color: second });
  scene.text((t) => (t < 56 ? 'VIIRS, crudo' : 'VIIRS, armonizado'), 860, BASE + 42, 'lede mid', { on: 12, color: second });

  scene.text((t) => (t < 56 ? '× 12,5' : '× 0,77'), 1130, 470, 'huge', { on: 28, dur: 6, color: 'var(--activity)' });
  scene.text('VIIRS contra MODIS', 1140, 520, 'lede', { on: 30, color: second });
  scene.text('Cerca, no exacto: 2019 fue más extremo que todo lo que el modelo había visto.', 800, 860, 'note mid', { on: 74, color: second });
}

/** 3:20, Santiago. One Rust core from the archive to the browser; then the network goes off. */
function engine(scene) {
  const nodes = [
    [140, 'Archivo de FIRMS', '16,6 millones de registros', 'leídos en 7,3 s'],
    [620, 'Un núcleo en Rust', 'nativo y WebAssembly', '8.968 consultas, idénticas'],
    [1100, 'Tu navegador', '0,75 ms por consulta', 'sin servidor'],
  ];
  nodes.forEach(([x, title, a, b], i) => {
    const on = 4 + i * 10;
    scene.fill(rrect(x, 200, 360, 190, 14), { on, dur: 4, color: i === 1 ? 'var(--fire)' : 'var(--paper)', amp: 1.2, boil: 0.5 });
    scene.line(rrect(x, 200, 360, 190, 14), { on, dur: 6, closed: true, w: 4, amp: 1.2, boil: 0.5 });
    scene.text(title, x + 180, 262, 'title mid', { on: on + 3, color: ink });
    scene.text(a, x + 180, 310, 'note mid', { on: on + 4, color: ink });
    scene.text(b, x + 180, 346, 'note mid', { on: on + 5, color: second });
    if (i < 2) {
      scene.line([[x + 372, 295], [x + 470, 295]], { on: on + 6, dur: 3, w: 4, amp: 0.8 });
      scene.line([[x + 452, 280], [x + 470, 295], [x + 452, 310]], { on: on + 8, dur: 2, w: 4, amp: 0.4 });
    }
  });

  [44, 88, 132].forEach((r, i) => scene.line(arc(0, 0, r, r, -Math.PI * 0.78, -Math.PI * 0.22, 14), { x: 800, y: 700, on: 58 + i * 3, dur: 5, w: 9, amp: 1, boil: 0.5 }));
  scene.fill(circle(13), { x: 800, y: 700, on: 58, color: ink, amp: 0.4 });
  scene.line([[-130, -170], [130, 40]], { x: 800, y: 700, on: 80, dur: 5, w: 11, color: 'var(--attention)', amp: 1.2 });
  scene.text('Respondió en menos de un milisegundo.', 800, 840, 'title mid', { on: 88, color: ink });
}

/** 4:25, Axel. The two numbers again, the band between them, and the name. */
function close(scene) {
  scene.text(es(heldOut.modis), 520, 330, 'huge mid', { on: 4, dur: 8, off: 66, offDur: 6, color: ink });
  scene.text(es(heldOut.viirsRaw), 520, 610, 'huge mid', { on: 10, dur: 8, off: 66, offDur: 6, color: ink });
  const band = { off: 66, offDur: 6, color: 'var(--uncertainty)', amp: 0.8, boil: 0.5 };
  scene.line([[890, 220], [890, 600]], { ...band, on: 34, dur: 8, w: 9 });
  scene.line([[855, 220], [925, 220]], { ...band, on: 40, dur: 3, w: 9 });
  scene.line([[855, 600], [925, 600]], { ...band, on: 40, dur: 3, w: 9 });
  scene.text('A ninguno solo.', 970, 360, 'big', { on: 36, off: 66, offDur: 6, color: ink });
  scene.text('A los dos juntos,', 970, 440, 'big', { on: 42, off: 66, offDur: 6, color: ink });
  scene.text('con su margen de error.', 970, 520, 'big', { on: 46, off: 66, offDur: 6, color: 'var(--uncertainty)' });

  const mark = scene.svg.appendChild(document.createElementNS('http://www.w3.org/2000/svg', 'g'));
  mark.setAttribute('transform', 'translate(470 640)');
  logo.build(scene, 0.55, 72);
  scene.items.slice(-4).forEach((item) => mark.appendChild(item.node));
  scene.text('firecal', 660, 460, 'huge', { on: 78, dur: 8, color: ink });
  scene.text('firecal.pages.dev', 668, 540, 'title', { on: 84, color: 'var(--activity)' });
  scene.text('El sensor deja de importar. Cada número dice cuánto confiar en él.', 668, 600, 'lede', { on: 88, color: second });
}

/** The slides in order, each with the moments a click stops at. */
export const SLIDES = [
  { build: hook, stops: [16, 38, 70, 100] },
  { build: name, stops: [100] },
  { build: demo, stops: [40, 66, 100] },
  { build: agree, stops: [40, 100] },
  { build: engine, stops: [50, 76, 100] },
  { build: close, stops: [30, 62, 100] },
];
