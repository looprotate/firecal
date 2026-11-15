// What the site draws. Every number is the app's answer on the published south-east Australia tiles, read on
// 2026-11-15, or a measurement in pitch/numbers.md.

/** Hotspots per month, November 2000 to December 2019; "!" marks a month the core flags as unusual. */
const DEMO_AREA = '10 4 13 4 106 174 82 9 1 27 14 8 12 992 768 2 79 123 100 18 20 70 89 27 273 281 8470 2100 50 66 145 22 23 29 71 15 18 2 15 3 80 123 229 102 26 10 8 24 9 6 13 1 89 194 144 48 10 25 16 14 7 6 6 66 23 173 211 46 23 28 15 86 10 1024 991 196 28 424 148 4 9 20 21 14 8 5 23 4 91 184 95 16 20 7 43 6 8 3 36 89 180 194 237 24 53 102 34 7 40 32 14 2 309 143 84 15 4 5 19 6 8 2 4 9 72 151 18 6 6 8 19 17 1 1 5 3 59 40 32 4 14 9 24 22 1 8 218 234 66 215 113 11 14 7 8 97 33 3 404 842 33 94 51 0 5 15 13 14 3 22 10 13 212 162 44 6 2 6 6 29 8 11 10 3 25 273 161 7 7 18 10 11 31 8 4 10 62 156 122 16 14 16 61 19 33 8 40 5 33 148 145 37 55 140 40 1 1 1 103 77 152 139 143 31 25 21 12 5 418! 7112!';

/** The same months for a smaller area inside it, the one drawn with the network off. */
const OFFLINE_AREA = '2 1 0 0 43 52 5 5 0 12 3 1 3 0 13 1 50 45 19 9 11 41 18 6 0 61 1786 913 26 25 28 6 12 11 39 6 0 2 0 1 41 51 55 49 12 4 4 13 3 0 2 0 40 49 43 15 7 7 0 2 2 1 0 1 14 53 24 3 12 10 4 15 2 0 70 2 22 186 63 0 3 12 2 6 3 1 20 0 57 90 38 8 10 1 13 1 1 0 10 37 146 66 114 10 47 38 6 0 38 18 14 1 161 63 32 8 0 1 12 3 5 0 2 5 21 33 0 2 0 3 7 11 0 0 0 0 33 14 14 0 8 1 4 0 1 4 34 3 31 51 44 0 7 0 0 8 1 0 147 835 22 19 25 0 2 1 1 5 0 1 0 0 56 20 16 1 2 0 1 5 0 6 3 0 4 64 13 6 5 7 2 0 8 2 0 0 18 40 23 7 1 1 26 9 27 7 1 4 14 14 56 14 23 95 30 1 1 0 48 71 1 33 48 8 18 16 1 2 124! 2377!';

/** The first month of both series: November 2000, as months since January 2000. */
export const FIRST_MONTH = 10;

/** A series as months: value, flag, and where it sits in the calendar (row 0 is 2000, column 0 is January). */
export function parseMonths(text) {
  return text.split(' ').map((word, i) => {
    const at = i + FIRST_MONTH;
    return { value: Number.parseInt(word, 10), unusual: word.endsWith('!'), row: Math.floor(at / 12), col: at % 12 };
  });
}

export const demoArea = parseMonths(DEMO_AREA);
export const offlineArea = parseMonths(OFFLINE_AREA);

/** 2019 against the median of 2001 to 2018: hotspots, median, the band at two sigma, and the core's flag. */
export const season2019 = [
  [103, 15, 22], [77, 7, 20], [152, 69, 27], [139, 159, 28], [143, 118, 28], [31, 16, 13],
  [25, 14, 11], [21, 17, 10], [12, 19, 7], [5, 15, 5], [418, 9, 45, true], [7112, 7, 175, true],
].map(([value, median, band, unusual = false]) => ({ value, median, band, unusual }));

/** The three past years the core finds most like 2019, nearest first, with their distance. */
export const analogs = [[2009, 49], [2001, 67], [2002, 68]];

/** 2019, held out of the fit, over the nine tiles of the region (pitch/numbers.md). */
export const heldOut = { modis: 50497, viirsRaw: 631751, viirsHarmonized: 38910 };

/**
 * The calendar ramp step of a value, as `web/calendar/ramp.ts` computes it: a square-root scale ending at `max`,
 * and any activity at least step 1.
 */
export function rampStep(value, max) {
  if (!(value > 0) || !(max > 0)) return 0;
  return Math.min(Math.max(Math.ceil(7 * Math.sqrt(Math.min(value / max, 1))), 1), 7);
}

export const formatNumber = (value) => Math.round(value).toLocaleString('en-US');
