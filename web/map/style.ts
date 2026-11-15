import type { FilterSpecification, LayerSpecification, StyleSpecification } from 'maplibre-gl';

/** The token colors the basemap is drawn in, read from `web/tokens/tokens.css`. */
export interface MapColors {
  land: string;
  water: string;
  border: string;
  region: string;
  label: string;
  selection: string;
  forest: string;
  park: string;
  farmland: string;
  road: string;
  highway: string;
}

/** The custom property each map color comes from. */
export const MAP_COLOR_TOKENS: Record<keyof MapColors, string> = {
  land: '--map-land',
  water: '--map-water',
  border: '--disabled',
  region: '--line',
  label: '--text-secondary',
  selection: '--map-selection',
  forest: '--map-forest',
  park: '--map-park',
  farmland: '--map-farmland',
  road: '--map-road',
  highway: '--map-highway',
};

/** The id of the layer that outlines the selected area, and of its GeoJSON source. */
export const SELECTION = 'firecal-selection';
export const SELECTION_FILL = 'firecal-activity-fill';

const OPENFREEMAP = 'https://tiles.openfreemap.org';

/** A local natural-color basemap on OpenStreetMap data, with a data-driven selection fill. */
export function mapStyle(colors: MapColors): StyleSpecification {
  return {
    version: 8,
    glyphs: `${OPENFREEMAP}/fonts/{fontstack}/{range}.pbf`,
    sources: {
      openmaptiles: {
        type: 'vector',
        url: `${OPENFREEMAP}/planet`,
        attribution: 'OpenFreeMap © OpenMapTiles, data © OpenStreetMap contributors',
      },
      [SELECTION]: { type: 'geojson', data: { type: 'FeatureCollection', features: [] } },
    },
    layers: [
      { id: 'land', type: 'background', paint: { 'background-color': colors.land } },
      { id: 'farmland', type: 'fill', source: 'openmaptiles', 'source-layer': 'landcover', filter: ['==', ['get', 'class'], 'farmland'], paint: { 'fill-color': colors.farmland } },
      { id: 'forest', type: 'fill', source: 'openmaptiles', 'source-layer': 'landcover', filter: ['==', ['get', 'class'], 'wood'], paint: { 'fill-color': colors.forest } },
      { id: 'parks', type: 'fill', source: 'openmaptiles', 'source-layer': 'landuse', filter: ['in', ['get', 'class'], ['literal', ['park', 'grass', 'recreation_ground', 'nature_reserve']]], paint: { 'fill-color': colors.park } },
      { id: 'water', type: 'fill', source: 'openmaptiles', 'source-layer': 'water', paint: { 'fill-color': colors.water } },
      { id: 'waterways', type: 'line', source: 'openmaptiles', 'source-layer': 'waterway', minzoom: 7, paint: { 'line-color': colors.water, 'line-width': 1 } },
      { id: 'roads', type: 'line', source: 'openmaptiles', 'source-layer': 'transportation', minzoom: 6, filter: ['in', ['get', 'class'], ['literal', ['primary', 'secondary', 'tertiary', 'minor']]], paint: { 'line-color': colors.road, 'line-width': ['interpolate', ['linear'], ['zoom'], 6, 0.5, 14, 4] } },
      { id: 'highways', type: 'line', source: 'openmaptiles', 'source-layer': 'transportation', minzoom: 5, filter: ['in', ['get', 'class'], ['literal', ['motorway', 'trunk']]], paint: { 'line-color': colors.highway, 'line-width': ['interpolate', ['linear'], ['zoom'], 5, 1, 13, 5] } },
      border('regions', 4, colors.region, 4),
      border('countries', 2, colors.border, 0),
      placeNames('country-names', ['==', ['get', 'class'], 'country'], 12, 0, colors.label, colors.land),
      placeNames('city-names', ['==', ['get', 'class'], 'city'], 11, 5, colors.label, colors.land),
      { id: SELECTION_FILL, type: 'fill', source: SELECTION, paint: { 'fill-color': colors.selection, 'fill-opacity': 0 } },
      { id: SELECTION, type: 'line', source: SELECTION, paint: { 'line-color': colors.selection, 'line-width': 3 } },
    ],
  };
}

function border(id: string, adminLevel: number, color: string, minzoom: number): LayerSpecification {
  return {
    id,
    type: 'line',
    source: 'openmaptiles',
    'source-layer': 'boundary',
    minzoom,
    filter: ['all', ['==', ['get', 'admin_level'], adminLevel], ['!=', ['get', 'maritime'], 1]],
    paint: { 'line-color': color, 'line-width': 1 },
  };
}

function placeNames(
  id: string,
  filter: FilterSpecification,
  size: number,
  minzoom: number,
  color: string,
  halo: string,
): LayerSpecification {
  return {
    id,
    type: 'symbol',
    source: 'openmaptiles',
    'source-layer': 'place',
    minzoom,
    filter,
    layout: { 'text-field': ['get', 'name:latin'], 'text-font': ['Noto Sans Regular'], 'text-size': size },
    paint: { 'text-color': color, 'text-halo-color': halo, 'text-halo-width': 1.2 },
  };
}
