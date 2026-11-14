import type { Area } from './series.ts';

export interface AreaBoundsText {
  west: string;
  south: string;
  east: string;
  north: string;
}

export type AreaBoundsResult =
  | { area: Area; error: null }
  | { area: null; error: string };

export const AREA_SELECTED_EVENT = 'firecal:area-selected';

/** Parses user-entered bounds and enforces the rectangle limits accepted by the core. */
export function parseAreaBounds(bounds: AreaBoundsText): AreaBoundsResult {
  const west = parseCoordinate(bounds.west, 'West longitude');
  if (!west.valid) return { area: null, error: west.error };
  const south = parseCoordinate(bounds.south, 'South latitude');
  if (!south.valid) return { area: null, error: south.error };
  const east = parseCoordinate(bounds.east, 'East longitude');
  if (!east.valid) return { area: null, error: east.error };
  const north = parseCoordinate(bounds.north, 'North latitude');
  if (!north.valid) return { area: null, error: north.error };

  const area = {
    west: west.value,
    south: south.value,
    east: east.value,
    north: north.value,
  };
  if (area.west < -180 || area.east > 180) {
    return { area: null, error: 'Longitudes must be between -180° and 180°.' };
  }
  if (area.south < -90 || area.north > 90) {
    return { area: null, error: 'Latitudes must be between -90° and 90°.' };
  }
  if (area.west >= area.east) {
    return { area: null, error: 'West longitude must be less than east longitude.' };
  }
  if (area.south >= area.north) {
    return { area: null, error: 'South latitude must be less than north latitude.' };
  }
  return { area, error: null };
}

/** Sends the one area-selection event used by coordinate entry and map drawing. */
export function emitAreaSelected(target: EventTarget, area: Area): void {
  target.dispatchEvent(new CustomEvent<Area>(AREA_SELECTED_EVENT, { detail: area }));
}

/** Reads and validates the detail of an area-selection event from the app shell. */
export function areaFromSelectionEvent(event: Event): Area | null {
  if (!(event instanceof CustomEvent)) return null;
  const detail: unknown = event.detail;
  if (typeof detail !== 'object' || detail === null) return null;
  const result = parseAreaBounds({
    west: String(Reflect.get(detail, 'west') ?? ''),
    south: String(Reflect.get(detail, 'south') ?? ''),
    east: String(Reflect.get(detail, 'east') ?? ''),
    north: String(Reflect.get(detail, 'north') ?? ''),
  });
  return result.area;
}

/** Converts the closed outer ring emitted by rectangle mode into its geographic bounds. */
export function areaFromRectangleCoordinates(ring: readonly (readonly number[])[]): Area | null {
  const corners = closedRingCorners(ring);
  if (!corners) return null;

  const longitudes = corners.map(([longitude]) => longitude);
  const latitudes = corners.map(([, latitude]) => latitude);
  const west = Math.min(...longitudes);
  const east = Math.max(...longitudes);
  const south = Math.min(...latitudes);
  const north = Math.max(...latitudes);
  const result = parseAreaBounds({
    west: String(west),
    south: String(south),
    east: String(east),
    north: String(north),
  });
  if (!result.area) return null;

  const expectedCorners = new Set([
    `${west},${south}`,
    `${west},${north}`,
    `${east},${south}`,
    `${east},${north}`,
  ]);
  const actualCorners = new Set(corners.map(([longitude, latitude]) => `${longitude},${latitude}`));
  if (actualCorners.size !== 4 || [...expectedCorners].some((corner) => !actualCorners.has(corner))) {
    return null;
  }
  return result.area;
}

type CoordinateResult =
  | { valid: true; value: number; error: null }
  | { valid: false; value: null; error: string };

function parseCoordinate(input: string, name: string): CoordinateResult {
  if (input.trim() === '') return { valid: false, value: null, error: `${name} is required.` };
  const value = Number(input);
  if (!Number.isFinite(value)) return { valid: false, value: null, error: `${name} must be a finite number.` };
  return { valid: true, value, error: null };
}

function closedRingCorners(
  ring: readonly (readonly number[])[],
): [number, number][] | null {
  if (ring.length < 5) return null;
  const corners = ring.slice(0, -1);
  const first = corners[0];
  const last = ring[ring.length - 1];
  if (!first || !last || !samePosition(first, last) || corners.length !== 4) return null;
  const parsedCorners: [number, number][] = [];
  for (const position of corners) {
    const [longitude, latitude] = position;
    if (
      longitude === undefined ||
      latitude === undefined ||
      !Number.isFinite(longitude) ||
      !Number.isFinite(latitude)
    ) {
      return null;
    }
    parsedCorners.push([longitude, latitude]);
  }
  return parsedCorners;
}

function samePosition(first: readonly number[], second: readonly number[]): boolean {
  return first[0] === second[0] && first[1] === second[1];
}
