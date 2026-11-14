/**
 * The browser contract of `firecal-wasm`: plain exports of `firecal.wasm`, no glue code.
 * Numbers are `u32` (or `f64` where it says so); a function that cannot do what was asked returns a status
 * and `INVALID` for anything it cannot read, never a trap. Memory addresses start at 1: address 0 is refused.
 * JavaScript reads a `u32` result of the raw module as a signed number (`INVALID` is -1): `wasm/firecal.ts` loads
 * the module and returns the exports below with every result unsigned, as typed here.
 * Changing this file is a contract change (see docs/engineering.md).
 */

/** Returned by an export that could not do what was asked or read what it was given: `u32::MAX`. */
export const INVALID = 4294967295;

/** The series was written. */
export const STATUS_OK = 0;
/** The area has no extent: south is not below north, or west not left of east. */
export const STATUS_EMPTY_AREA = 1;
/** A latitude outside -90 to 90, a longitude outside -180 to 180, or a bound that is not a number. */
export const STATUS_AREA_OUT_OF_RANGE = 2;
/** No months, or a month or count that does not fit in 16 bits. */
export const STATUS_MONTH_RANGE_INVALID = 3;
/** A tile that is not a valid tile file, or whose row and column are outside the 18 x 36 tile grid. */
export const STATUS_TILE_INVALID = 4;
/** A tile that does not hold one of the requested months. */
export const STATUS_MONTH_NOT_IN_TILE = 5;

/** The text does not fit in the buffer the page gave. */
export const STATUS_OUTPUT_TOO_SMALL = 6;

/** A baseline whose first year is after its last, a year that does not fit in 16 bits, or more than 64 years. */
export const STATUS_BASELINE_INVALID = 7;
/** A season of zero months or more than twelve, or one the series does not hold in full. */
export const STATUS_SEASON_INVALID = 8;
/** A polygon whose edges cross or touch, or that goes all the way around a pole. */
export const STATUS_POLYGON_INVALID = 9;

/** The `plane` of a request: the harmonized count, with its sigma. */
export const PLANE_COUNT = 0;
/** The `plane` of a request: the harmonized energy in megawatts. It has no sigma yet: its sigmas are NaN. */
export const PLANE_ENERGY = 1;

/** Bytes of a series request block. */
export const REQUEST_BYTES = 56;
/** Bytes of one entry of the tile table. */
export const TILE_ENTRY_BYTES = 16;
/** Bytes of an analysis request block: a series request and a baseline, a season and a polygon. */
export const ANALYSIS_REQUEST_BYTES = 80;
/** Bytes of one polygon corner: latitude then longitude, `f64`. */
export const VERTEX_BYTES = 16;
/** Bytes per month of the output of `fc_area_series`. */
export const AREA_SERIES_BYTES_PER_MONTH = 16;
/** Bytes per month of the output of `fc_comparison`. */
export const COMPARISON_BYTES_PER_MONTH = 17;
/** Bytes of the count that opens the output of `fc_analogs`. */
export const ANALOGS_COUNT_BYTES = 4;
/** Bytes of one analog year in the output of `fc_analogs`. */
export const ANALOG_BYTES = 12;

export interface FirecalExports {
  /** The module's linear memory. Its buffer is replaced when the memory grows: read it again after `fc_alloc`. */
  memory: WebAssembly.Memory;
  /** The month index of `month` (1 to 12) of `year` (2000 or later), or `INVALID`. */
  fc_month_index(year: number, month: number): number;
  /** The number of months from `first` to `last` inclusive, or `INVALID`. */
  fc_month_span(first: number, last: number): number;
  /** A zeroed block of `length` bytes (1 to 2^30) for the page to fill: its address, or `INVALID`. */
  fc_alloc(length: number): number;
  /** Gives back a block from `fc_alloc`: 0, or `INVALID` when `address` is not a block held. */
  fc_free(address: number): number;
  /**
   * Answers the series request block of `requestLength` (must be `REQUEST_BYTES`) bytes at `requestAddress`:
   * the harmonized value of the chosen plane over the area, month by month, and its sigma.
   * Returns a `STATUS_*`, or `INVALID` when an address, length or plane cannot be read. Nothing is written to the
   * output unless the status is `STATUS_OK`. A tile missing from the table counts as zeros; a tile present that
   * lacks a month is `STATUS_MONTH_NOT_IN_TILE`.
   *
   * Request block, little-endian:
   *   0, 8, 16, 24  f64  south, north, west, east, in degrees (a cell is selected when the area overlaps it)
   *   32            u32  first month index
   *   36            u32  month count
   *   40            u32  plane: PLANE_COUNT or PLANE_ENERGY
   *   44            u32  tile count
   *   48            u32  address of the tile table (tile count x TILE_ENTRY_BYTES bytes)
   *   52            u32  address of the output (month count x 16 bytes)
   *
   * Tile table entry, little-endian:
   *   0   u32  tile row (0 to 17, 0 at the north)
   *   4   u32  tile column (0 to 35, 0 at 180 degrees west)
   *   8   u32  address of the tile file (docs/data-format.md)
   *   12  u32  length of the tile file in bytes
   *
   * Output, little-endian: month count `f64` values, then month count `f64` sigmas.
   */
  fc_series(requestAddress: number, requestLength: number): number;
  /**
   * Writes the `cells.csv` of the request block at `requestAddress` (same block as `fc_series`; its output
   * field is not used) into the buffer of `bufferLength` bytes at `bufferAddress`: first the length of the text
   * as a `u32`, then the text, which is `year,month,cell_lat,cell_lon,value,sigma` with a header line, one row
   * per cell of the area and month, months first, then rows north to south and columns west to east,
   * lines ending in `\n`. The sigma field is empty for the energy plane.
   * Returns a `STATUS_*` (`STATUS_OUTPUT_TOO_SMALL` when the text does not fit), or `INVALID` when an address
   * or length cannot be read or `bufferLength` is below 4. Nothing is written unless the status is `STATUS_OK`.
   */
  fc_cells_csv(requestAddress: number, requestLength: number, bufferAddress: number, bufferLength: number): number;
  /**
   * The series of the area over the whole history the page gives, month by month: the same numbers as
   * `fc_series`, but the area may be a polygon. Takes the analysis request block (below) of `requestLength`
   * (must be `ANALYSIS_REQUEST_BYTES`) bytes at `requestAddress` and writes into the buffer of `bufferLength`
   * bytes at `bufferAddress`: month count `f64` values, then month count `f64` sigmas (an energy series has
   * NaN sigmas). The baseline and the season of the block are not used.
   * Returns a `STATUS_*` (`STATUS_OUTPUT_TOO_SMALL` when `bufferLength` is below month count x
   * `AREA_SERIES_BYTES_PER_MONTH`), or `INVALID` when an address or length cannot be read. Nothing is written
   * unless the status is `STATUS_OK`.
   *
   * Analysis request block, little-endian:
   *   0 to 55  a series request block, as for `fc_series`; its output address is not used
   *   56, 60   u32  first and last baseline year (2001 to the last complete year); at most 64 years
   *   64       u32  year of the season in progress
   *   68       u32  months of the season, 1 to 12
   *   72       u32  polygon corner count; 0 means the rectangle in the series request, otherwise the rectangle
   *                 is ignored and the area is the polygon (the cells whose center it contains)
   *   76       u32  address of the polygon corners (corner count x `VERTEX_BYTES` bytes: latitude, longitude `f64`)
   *
   * The series is that of the request's `first month` and `month count`: for the comparison and the analogs it
   * is the history, so it must hold the baseline years and the season in progress.
   */
  fc_area_series(requestAddress: number, requestLength: number, bufferAddress: number, bufferLength: number): number;
  /**
   * How each month of the history compares with what is normal: for each month, the median of its calendar month
   * over the baseline years, the anomaly `(value - median) / max(median, 1)`, and whether it is unusual: the
   * difference clears the sensor band, `|value - median| > 2 * sigma`, and the value lies outside the lowest
   * to highest value of the baseline years for its calendar month. Same request and buffer as `fc_area_series`.
   * Writes month count `f64` medians, month count `f64` anomalies, then month count bytes, 1 for unusual and 0
   * for not. A month whose calendar month has no baseline value has NaN for the median and the anomaly and 0 for
   * the flag.
   * Returns a `STATUS_*` (`STATUS_BASELINE_INVALID`; `STATUS_OUTPUT_TOO_SMALL` below month count x
   * `COMPARISON_BYTES_PER_MONTH`), or `INVALID`. Nothing is written unless the status is `STATUS_OK`.
   */
  fc_comparison(requestAddress: number, requestLength: number, bufferAddress: number, bufferLength: number): number;
  /**
   * The past years most like the season in progress (January to month `season months` of `season year`), nearest
   * first. Same request as `fc_area_series`. The buffer holds first the number found as a `u32`, then room for
   * as many slots as fit after it (at most 64 are used, and the number of slots is how many are asked for), each
   * a `u32` year and an `f64` distance: the sum over the season's months of the difference of the years' ranks
   * within the calendar month. Years at the same distance come oldest first.
   * Returns a `STATUS_*` (`STATUS_BASELINE_INVALID`, `STATUS_SEASON_INVALID`, `STATUS_OUTPUT_TOO_SMALL` below
   * `ANALOGS_COUNT_BYTES`), or `INVALID`. Nothing is written unless the status is `STATUS_OK`.
   */
  fc_analogs(requestAddress: number, requestLength: number, bufferAddress: number, bufferLength: number): number;
  /**
   * The `series.csv` of the history: first the length of the text as a `u32`, then the text, which is
   * `year,month,value,sigma,median,anomaly,unusual` with a header line and a row per month, lines ending in
   * `\n` (docs/data-format.md). Same request and buffer rules as `fc_cells_csv`, with the analysis block.
   * Returns a `STATUS_*` (`STATUS_OUTPUT_TOO_SMALL` when the text does not fit), or `INVALID`.
   * Nothing is written unless the status is `STATUS_OK`.
   */
  fc_series_csv(requestAddress: number, requestLength: number, bufferAddress: number, bufferLength: number): number;
}
