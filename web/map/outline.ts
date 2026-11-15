import type { GeoJSONSource, LngLatBoundsLike } from 'maplibre-gl';
import type { AreaSelection } from '../app/area-input.ts';
import { SELECTION } from './style.ts';

/** The part of the map the outline draws on. */
export interface OutlineTarget {
  getSource(id: string): Pick<GeoJSONSource, 'setData'> | undefined;
  fitBounds(bounds: LngLatBoundsLike, options: { padding: number; maxZoom: number }): unknown;
}

/**
 * Draws and frames the selected area. Until the map has loaded it only remembers the latest area, so an area typed
 * while the map is still loading is the one shown, not an earlier one.
 */
export function selectionOutline(target: OutlineTarget): { show(selection: AreaSelection): void; ready(): void } {
  let latest: AreaSelection | null = null;
  let loaded = false;
  const draw = (selection: AreaSelection): void => {
    target.getSource(SELECTION)?.setData({
      type: 'Feature',
      properties: {},
      geometry: { type: 'Polygon', coordinates: [ring(selection)] },
    });
    const { west, south, east, north } = selection.area;
    target.fitBounds([[west, south], [east, north]], { padding: 36, maxZoom: 8 });
  };
  return {
    show(selection) {
      latest = selection;
      if (loaded) draw(selection);
    },
    ready() {
      loaded = true;
      if (latest) draw(latest);
    },
  };
}

/** The closed ring of a selection, longitude first, as GeoJSON wants it. */
export function ring(selection: AreaSelection): [number, number][] {
  if (selection.polygon) {
    const corners = selection.polygon.map(([latitude, longitude]): [number, number] => [longitude, latitude]);
    const first = corners[0];
    return first ? [...corners, first] : corners;
  }
  const { west, south, east, north } = selection.area;
  return [[west, south], [east, south], [east, north], [west, north], [west, south]];
}
