import { Map, NavigationControl, setWorkerUrl, type GeoJSONSource } from 'maplibre-gl';
import workerUrl from 'maplibre-gl/dist/maplibre-gl-worker.mjs?worker&url';
import { MaplibreTerradrawControl, getDefaultModeOptions } from '@watergis/maplibre-gl-terradraw';
import { areaFromRectangleCoordinates, selectionFromPolygonCoordinates } from '../app/area-input.ts';
import type { AreaSelection } from '../app/area-input.ts';
import type { Area } from '../app/series.ts';
import { MAP_COLOR_TOKENS, SELECTION, mapStyle, type MapColors } from './style.ts';

setWorkerUrl(workerUrl);

/** The map and a way to outline the selected area on it. */
export interface AreaMap {
  map: Map;
  /** Outlines the area (or the polygon inside it) and frames it. */
  showSelection(selection: AreaSelection): void;
}

/** Creates the map, the rectangle and polygon tools, and the outline of the selected area. */
export function createAreaMap(container: HTMLElement, onSelected: (selection: AreaSelection) => void): AreaMap {
  const colors = tokenColors();
  const map = new Map({
    container,
    style: mapStyle(colors),
    center: [-60, -18],
    zoom: 3,
    attributionControl: { compact: true },
  });
  map.addControl(new NavigationControl({ showCompass: false }), 'top-right');
  const drawing = new MaplibreTerradrawControl({
    modes: ['rectangle', 'polygon'],
    open: true,
    modeOptions: inkModes(colors.selection),
  });
  map.addControl(drawing, 'top-left');
  nameDrawingButtons(container);

  let pending: AreaSelection | null = null;
  const outline = (selection: AreaSelection): void => {
    const source = map.getSource<GeoJSONSource>(SELECTION);
    if (!source) {
      pending = selection;
      return;
    }
    source.setData({ type: 'Feature', properties: {}, geometry: { type: 'Polygon', coordinates: [ring(selection)] } });
    const { west, south, east, north } = selection.area;
    map.fitBounds([[west, south], [east, north]], { padding: 36, maxZoom: 8 });
  };

  map.once('load', () => {
    if (pending) outline(pending);
    const terraDraw = drawing.getTerraDrawInstance();
    terraDraw?.on('finish', (id, context) => {
      const feature = terraDraw.getSnapshotFeature(id);
      const drawn = feature?.geometry.type === 'Polygon' ? feature.geometry.coordinates[0] : undefined;
      terraDraw.clear();
      if (!drawn) return;
      const selection = context.mode === 'rectangle' ? rectangleSelection(drawn) : selectionFromPolygonCoordinates(drawn);
      if (selection) onSelected(selection);
    });
  });
  return { map, showSelection: outline };
}

/** The drawing buttons carry only a title; give them the names a screen reader says. */
function nameDrawingButtons(container: HTMLElement): void {
  const names: Record<string, string> = { rectangle: 'Draw a rectangle', polygon: 'Draw a polygon' };
  for (const [mode, name] of Object.entries(names)) {
    const button = container.querySelector<HTMLButtonElement>(`.maplibregl-terradraw-add-${mode}-button`);
    button?.setAttribute('aria-label', name);
  }
}

function rectangleSelection(drawn: readonly (readonly number[])[]): AreaSelection | null {
  const area = areaFromRectangleCoordinates(drawn);
  return area ? { area } : null;
}

/** The closed ring of a selection, longitude first, as GeoJSON wants it. */
export function ring(selection: AreaSelection): [number, number][] {
  if (selection.polygon) {
    const corners = selection.polygon.map(([latitude, longitude]): [number, number] => [longitude, latitude]);
    const first = corners[0];
    return first ? [...corners, first] : corners;
  }
  const { west, south, east, north }: Area = selection.area;
  return [[west, south], [east, south], [east, north], [west, north], [west, south]];
}

function tokenColors(): MapColors {
  const style = getComputedStyle(document.documentElement);
  const read = (name: string): string => style.getPropertyValue(name).trim();
  return {
    land: read(MAP_COLOR_TOKENS.land),
    water: read(MAP_COLOR_TOKENS.water),
    border: read(MAP_COLOR_TOKENS.border),
    region: read(MAP_COLOR_TOKENS.region),
    label: read(MAP_COLOR_TOKENS.label),
    selection: read(MAP_COLOR_TOKENS.selection),
  };
}

/** The drawing tools in ink: an opaque outline while drawing, never a translucent fill. */
function inkModes(color: string): ReturnType<typeof getDefaultModeOptions> {
  const ink = hexColor(color);
  const modes = getDefaultModeOptions();
  const styles = {
    fillOpacity: 0,
    outlineColor: ink,
    outlineWidth: 2,
    coordinatePointColor: ink,
    coordinatePointOutlineColor: ink,
    closingPointColor: ink,
    closingPointOutlineColor: ink,
  };
  for (const name of ['rectangle', 'polygon']) modes[name]?.updateOptions({ styles });
  return modes;
}

function hexColor(value: string): `#${string}` {
  if (!/^#[0-9a-f]{6}$/i.test(value)) throw new Error(`The map needs a #rrggbb token color, not "${value}"`);
  return `#${value.slice(1)}`;
}
