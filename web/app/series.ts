/** The validated shape shared by the app shell and the calendar. */
export interface Area {
  west: number;
  south: number;
  east: number;
  north: number;
}

/** One historical season and its rank distance from the current season. */
export interface Analog {
  year: number;
  distance: number;
}

/** A contiguous set of month values matching the documented Firecal Series. */
export interface Series {
  area: Area;
  plane: 'count' | 'energy';
  first_month_index: number;
  months: number[];
  value: number[];
  sigma: number[];
  median: number[];
  anomaly: number[];
  unusual: boolean[];
  analogs: Analog[];
}

/** Rejects malformed fixture or future WASM output at the UI boundary. */
export function parseSeries(input: unknown): Series {
  if (!isRecord(input)) throw new Error('Series must be an object');

  const area = readArea(input.area);
  const plane = input.plane;
  if (plane !== 'count' && plane !== 'energy') throw new Error('plane must be count or energy');

  const firstMonthIndex = readInteger(input.first_month_index, 'first_month_index', 0);
  const months = readIntegerArray(input.months, 'months', 0);
  if (months.length === 0) throw new Error('months must not be empty');
  if (months[0] !== firstMonthIndex) throw new Error('first_month_index must match the first month');
  for (let at = 1; at < months.length; at += 1) {
    if (months[at] !== firstMonthIndex + at) throw new Error('months must be consecutive');
  }

  const value = readNumberArray(input.value, 'value', 0);
  const sigma = readNumberArray(input.sigma, 'sigma', 0);
  const median = readNumberArray(input.median, 'median', 0);
  const anomaly = readNumberArray(input.anomaly, 'anomaly');
  const unusual = readBooleanArray(input.unusual, 'unusual');
  const analogs = readAnalogs(input.analogs);

  for (const [name, values] of [
    ['value', value],
    ['sigma', sigma],
    ['anomaly', anomaly],
    ['unusual', unusual],
  ] as const) {
    if (values.length !== months.length) throw new Error(`${name} length must match months length`);
  }
  if (median.length !== 12) throw new Error('median must have one value for each calendar month');

  return {
    area,
    plane,
    first_month_index: firstMonthIndex,
    months,
    value,
    sigma,
    median,
    anomaly,
    unusual,
    analogs,
  };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function readArea(value: unknown): Area {
  if (!isRecord(value)) throw new Error('area must be an object');
  const west = readFiniteNumber(value.west, 'area.west');
  const south = readFiniteNumber(value.south, 'area.south');
  const east = readFiniteNumber(value.east, 'area.east');
  const north = readFiniteNumber(value.north, 'area.north');
  if (west < -180 || east > 180 || west >= east) throw new Error('area longitude bounds are invalid');
  if (south < -90 || north > 90 || south >= north) throw new Error('area latitude bounds are invalid');
  return { west, south, east, north };
}

function readFiniteNumber(value: unknown, name: string, minimum?: number): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || (minimum !== undefined && value < minimum)) {
    throw new Error(`${name} must be a finite number${minimum === undefined ? '' : ` at least ${minimum}`}`);
  }
  return value;
}

function readInteger(value: unknown, name: string, minimum: number): number {
  if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < minimum) {
    throw new Error(`${name} must be an integer at least ${minimum}`);
  }
  return value;
}

function readNumberArray(value: unknown, name: string, minimum?: number): number[] {
  if (!Array.isArray(value)) throw new Error(`${name} must be an array`);
  return value.map((entry, at) => readFiniteNumber(entry, `${name}[${at}]`, minimum));
}

function readIntegerArray(value: unknown, name: string, minimum: number): number[] {
  if (!Array.isArray(value)) throw new Error(`${name} must be an array`);
  return value.map((entry, at) => readInteger(entry, `${name}[${at}]`, minimum));
}

function readBooleanArray(value: unknown, name: string): boolean[] {
  if (!Array.isArray(value)) throw new Error(`${name} must be an array`);
  return value.map((entry, at) => {
    if (typeof entry !== 'boolean') throw new Error(`${name}[${at}] must be a boolean`);
    return entry;
  });
}

function readAnalogs(value: unknown): Analog[] {
  if (!Array.isArray(value)) throw new Error('analogs must be an array');
  return value.map((entry, at) => {
    if (!isRecord(entry)) throw new Error(`analogs[${at}] must be an object`);
    return {
      year: readInteger(entry.year, `analogs[${at}].year`, 2000),
      distance: readFiniteNumber(entry.distance, `analogs[${at}].distance`, 0),
    };
  });
}
