// Runs the defence: one slide on screen at a time, its drawing moved along its timeline to the next stop on each
// click, the old slide undrawn before the next draws, and the boil of the film eight times a second. A click starts
// drawing at once: the timeline skips the stretch where nothing new begins.
//
// Keys: → space Enter or a click, forward; ← Backspace, back; D, the live app over the slides (and back);
// F or the button in the corner, full screen; Home, the first slide. `?app=<url>` says where the app runs (default:
// the local preview).
import { rrect } from '../../site/js/draw.js';
import { Scene } from '../../site/js/scene.js';
import { SLIDES } from './slides.js';

const BOIL_PER_SECOND = 8;
/** Timeline units a second, drawing forward. */
const FORWARD = 16;
/** How long undrawing takes, going back or leaving a slide, whatever is on it. */
const AWAY_SECONDS = 0.4;
/** How far before the next shape's start a click moves the timeline, so drawing begins at once. */
const LEAD = 0.3;
/** How long the pointer may rest before the full-screen button and the pointer hide. */
const IDLE_MS = 2200;
const SVG = 'http://www.w3.org/2000/svg';

const deck = document.querySelector('#deck');
const where = document.querySelector('#where');
const live = document.querySelector('#live');
const app = document.querySelector('#app');
const still = matchMedia('(prefers-reduced-motion: reduce)');
const appUrl = new URLSearchParams(location.search).get('app') ?? 'http://localhost:4173/';

const slides = SLIDES.map(({ build, stops }, index) => {
  const group = deck.appendChild(document.createElementNS(SVG, 'g'));
  group.setAttribute('class', 'slide');
  group.dataset.seed = String(index * 31);
  const scene = new Scene(group);
  build(scene);
  return { group, scene, stops, t: 0, target: 0, speed: FORWARD };
});

let current = 0;
let leaving = null;
const started = performance.now();
let last = started;

/** Sends a slide's timeline to `at`: forward from the next shape that starts, back in AWAY_SECONDS. */
function aim(slide, at) {
  slide.target = at;
  if (at > slide.t) {
    const drawing = slide.scene.items.some((item) => item.on <= slide.t && slide.t < item.on + item.dur);
    const starts = slide.scene.items.map((item) => item.on).filter((on) => on > slide.t && on < at);
    if (!drawing && starts.length > 0) slide.t = Math.max(slide.t, Math.min(...starts) - LEAD);
    slide.speed = FORWARD;
  } else {
    slide.speed = Math.max(1, (slide.t - at) / AWAY_SECONDS);
  }
  if (still.matches) slide.t = at;
}

function go(index, at, drawn = false) {
  if (index < 0 || index >= slides.length) return;
  if (index !== current) {
    leaving = slides[current];
    aim(leaving, 0);
    slides[index].t = drawn ? at : 0;
  }
  current = index;
  const slide = slides[current];
  if (leaving === slide) leaving = null;
  aim(slide, at);
  where.textContent = `${current + 1} / ${slides.length}`;
}

function forward() {
  const slide = slides[current];
  const next = slide.stops.find((stop) => stop > slide.target + 0.01);
  if (next !== undefined) go(current, next);
  else if (current + 1 < slides.length) go(current + 1, slides[current + 1].stops[0]);
}

function back() {
  const slide = slides[current];
  const earlier = slide.stops.filter((stop) => stop < slide.target - 0.01);
  if (earlier.length > 0) go(current, earlier.at(-1));
  else if (current > 0) go(current - 1, slides[current - 1].stops.at(-1), true);
}

function toggleLive() {
  if (live.hidden && app.src === '') app.src = appUrl;
  live.hidden = !live.hidden;
  if (!live.hidden) app.focus();
}

addEventListener('keydown', (event) => {
  if (event.metaKey || event.ctrlKey || event.altKey) return;
  const key = event.key.toLowerCase();
  if (key === 'd') toggleLive();
  else if (!live.hidden && key !== 'escape') return;
  else if (key === 'escape') live.hidden = true;
  else if (['arrowright', ' ', 'enter', 'pagedown'].includes(key)) forward();
  else if (['arrowleft', 'backspace', 'pageup'].includes(key)) back();
  else if (key === 'home') go(0, slides[0].stops[0]);
  else if (key === 'f') toggleFullScreen();
  else return;
  event.preventDefault();
});
deck.addEventListener('click', forward);

function toggleFullScreen() {
  (document.fullscreenElement ? document.exitFullscreen() : document.documentElement.requestFullscreen()).catch(() => {});
}

/** The full-screen button shows while the pointer moves and out of full screen; the pointer hides with it. */
const full = document.querySelector('#full');
let idle = 0;
full.addEventListener('click', (event) => {
  event.stopPropagation();
  toggleFullScreen();
});
const wake = () => {
  document.body.classList.remove('idle');
  clearTimeout(idle);
  idle = setTimeout(() => document.body.classList.add('idle'), IDLE_MS);
};
addEventListener('pointermove', wake);
document.addEventListener('fullscreenchange', () => {
  document.body.classList.toggle('fullscreen', document.fullscreenElement !== null);
  wake();
});
wake();
const fullScene = new Scene(full.querySelector('svg'));
fullScene.line(rrect(2, 2, 196, 44, 8), { on: 0, dur: 1, closed: true, w: 3, amp: 0.8, boil: 0.4 });

function step(slide, seconds) {
  const move = slide.speed * seconds;
  slide.t = slide.target >= slide.t ? Math.min(slide.target, slide.t + move) : Math.max(slide.target, slide.t - move);
}

function frame(now) {
  const seconds = Math.min(0.1, (now - last) / 1000);
  last = now;
  const quiet = still.matches;
  const boil = Math.floor(((now - started) / 1000) * BOIL_PER_SECOND);
  if (leaving && (quiet || leaving.t <= 0)) leaving = null;
  // While a slide leaves, it alone is on screen: the next one starts drawing once it is gone.
  const showing = leaving ?? slides[current];
  for (const slide of slides) {
    const shown = slide === showing;
    slide.group.classList.toggle('on', shown);
    if (!shown) continue;
    if (quiet) slide.t = slide.target;
    else step(slide, seconds);
    slide.scene.render(slide.t, boil, quiet);
  }
  fullScene.render(1, boil, quiet);
  requestAnimationFrame(frame);
}

go(0, slides[0].stops[0]);
requestAnimationFrame(frame);
