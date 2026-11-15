// The hand-drawn look of the site and the film, for shapes that stay still: the same resampling, the same wobble of
// the hand and the same smooth curves, from `site/js/draw.js`, with no boil. A shape keeps its wobble across redraws
// because its seed comes from what it is (a month, a mark), never from the order it is drawn in.
import { path, resample, rough, rrect } from '../../site/js/draw.js';

/** A point in SVG units. */
export type Point = [number, number];

/** The outline through `points`, moved by the hand's wobble of `amp` units, as an SVG path. */
export function sketch(points: readonly Point[], closed: boolean, seed: number, amp: number): string {
  return path(rough(resample(points.map(([x, y]) => [x, y]), closed), seed, amp, 0, 0), closed);
}

/** A rectangle with rounded corners of radius `r`, drawn by hand. */
export function sketchRect(x: number, y: number, w: number, h: number, r: number, seed: number, amp: number): string {
  return sketch(rrect(x, y, w, h, r), true, seed, amp);
}

/** A straight line drawn by hand: a few points along it, so the wobble has somewhere to go. */
export function sketchLine(from: Point, to: Point, seed: number, amp: number): string {
  return sketch([from, to], false, seed, amp);
}

/** A diamond centred on `cx`, `cy` with half-diagonal `half`. */
export function diamondPoints(cx: number, cy: number, half: number): Point[] {
  return [[cx, cy - half], [cx + half, cy], [cx, cy + half], [cx - half, cy]];
}
