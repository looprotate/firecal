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

/** The `plane` of a request: the harmonized count, with its sigma. */
export const PLANE_COUNT = 0;
/** The `plane` of a request: the harmonized energy in megawatts. It has no sigma yet: its sigmas are NaN. */
export const PLANE_ENERGY = 1;

/** Bytes of a series request block. */
export const REQUEST_BYTES = 56;
/** Bytes of one entry of the tile table. */
export const TILE_ENTRY_BYTES = 16;

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
}
