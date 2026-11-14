/** The number of ramp steps above an empty cell: `--ramp-1` to `--ramp-7` in `web/tokens/tokens.css`. */
export const RAMP_STEPS = 7;

/** One step of the ramp: 0 for a cell with no activity, 1 to `RAMP_STEPS` for the rest. */
export type RampStep = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7;

/**
 * The ramp step of `value` on a square-root scale that ends at `max`. Any activity gets at least step 1, so it
 * never looks like an empty cell. The square root keeps a few huge months from flattening all the others.
 */
export function rampStep(value: number, max: number): RampStep {
  if (!(value > 0) || !(max > 0)) return 0;
  const scaled = Math.ceil(RAMP_STEPS * Math.sqrt(Math.min(value / max, 1)));
  return Math.min(Math.max(scaled, 1), RAMP_STEPS) as RampStep;
}
