// Moves every drawing along its timeline: the logo by the clock as the page opens and by the scroll as it flies to
// the bar, each story section by how far it has been scrolled through, and the boil eight times a second.
import { clamp, circle, inOut, ring, rrect } from './draw.js';
import { Scene } from './scene.js';
import * as built from './scenes/built.js';
import * as calendar from './scenes/calendar.js';
import * as closing from './scenes/closing.js';
import * as logo from './scenes/logo.js';
import * as offline from './scenes/offline.js';
import * as satellites from './scenes/satellites.js';
import * as season from './scenes/season.js';

const BOIL_PER_SECOND = 8;
const INTRO_UNITS_PER_SECOND = 11;
const still = matchMedia('(prefers-reduced-motion: reduce)');
const builders = { built, calendar, closing, offline, satellites, season };

const mark = document.querySelector('#mark');
const markScene = new Scene(mark);
logo.build(markScene, 1, 0);
const heroSlot = document.querySelector('#hero-slot');
const navSlot = document.querySelector('#nav-slot');
const hero = document.querySelector('#top');
const heroCopy = document.querySelector('.hero-copy');
const started = performance.now();

const stories = [...document.querySelectorAll('[data-scene]')].map((section) => {
  const svg = section.querySelector('svg.stage');
  const scene = new Scene(svg);
  builders[section.dataset.scene].build(scene);
  return { section, scene, steps: [...section.querySelectorAll('[data-from]')], pinned: section.classList.contains('story') };
});

const film = document.querySelector('#film');
const frameScene = new Scene(film.querySelector('svg.frame-art'));
frameScene.line(rrect(-14, -14, 1628, 928, 22), { on: 0, dur: 40, closed: true, w: 5, amp: 3, boil: 1.4 });

/** A card's outline, or a person's ring and initial, drawn one after another as their section comes in. */
const reveals = [...document.querySelectorAll('[data-reveal]')].map((section) => ({
  section,
  arts: [...section.querySelectorAll('svg[data-art]')].map((svg, index) => {
    const scene = new Scene(svg);
    if (svg.dataset.art === 'card') {
      scene.line(rrect(1, 1, 98, 98, 4), { on: 0, dur: 30, closed: true, w: 3, amp: 0.5, boil: 0.25 });
    } else {
      scene.fill(circle(44), { x: 60, y: 60, on: 6, dur: 14, color: 'var(--surface)', amp: 1, boil: 0.5 });
      scene.line(ring(46, index + 5), { x: 60, y: 60, on: 0, dur: 24, w: 3, amp: 1, boil: 0.6 });
      scene.text(svg.dataset.initial, 60, 76, 'initial mid', { on: 14, dur: 12, color: 'var(--ink)' });
    }
    return { scene, index };
  }),
}));

/** How far a pinned section has been scrolled through (0 to 1), or how far an ordinary one has come into view. */
function progress(section, pinned) {
  const box = section.getBoundingClientRect();
  const view = innerHeight;
  if (pinned) return clamp(-box.top / Math.max(1, box.height - view));
  return clamp((view - box.top) / (view * 0.85));
}

function onScreen(section) {
  const box = section.getBoundingClientRect();
  return box.bottom > -50 && box.top < innerHeight + 50;
}

function placeMark(flight) {
  const from = heroSlot.getBoundingClientRect();
  const to = navSlot.getBoundingClientRect();
  const lerp = (a, b) => a + (b - a) * flight;
  const width = lerp(from.width, to.width);
  mark.style.width = `${width}px`;
  mark.style.height = `${width * (620 / 480)}px`;
  mark.style.transform = `translate(${lerp(from.left, to.left)}px, ${lerp(from.top, to.top)}px)`;
  heroCopy.style.opacity = String(1 - clamp(flight * 1.4));
  heroCopy.style.transform = `translateY(${-flight * 60}px)`;
  document.body.classList.toggle('arrived', flight > 0.97);
}

function frame(now) {
  const quiet = still.matches;
  const boil = Math.floor(((now - started) / 1000) * BOIL_PER_SECOND);
  const intro = quiet ? 100 : ((now - started) / 1000) * INTRO_UNITS_PER_SECOND;
  markScene.render(Math.min(intro, 100), boil, quiet);
  const flight = inOut(clamp((progress(hero, true) - 0.04) / 0.6));
  placeMark(flight);
  document.body.classList.toggle('intro-done', intro > 14);

  for (const story of stories) {
    if (!onScreen(story.section)) continue;
    const t = progress(story.section, story.pinned) * 100;
    story.scene.render(t, boil, quiet);
    for (const step of story.steps) step.classList.toggle('on', t >= Number(step.dataset.from) && t < Number(step.dataset.to));
  }
  if (onScreen(film)) frameScene.render(progress(film, false) * 100, boil, quiet);
  for (const reveal of reveals) {
    if (!onScreen(reveal.section)) continue;
    const t = progress(reveal.section, false) * 160;
    reveal.section.classList.toggle('in', t > 10);
    for (const art of reveal.arts) art.scene.render(t - art.index * 12, boil, quiet);
  }
  requestAnimationFrame(frame);
}

requestAnimationFrame(frame);
