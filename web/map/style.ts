import type { FilterSpecification, LayerSpecification, StyleSpecification } from 'maplibre-gl';

/** The token colors the basemap is drawn in, read from `web/tokens/tokens.css`. */
export interface MapColors {
  land: string;
  water: string;
  border: string;
  region: string;
  label: string;
  selection: string;
}

/** The custom property each map color comes from. */
export const MAP_COLOR_TOKENS: Record<keyof MapColors, string> = {
  land: '--bg',
  water: '--raised',
  border: '--disabled',
  region: '--line',
  label: '--text-secondary',
  selection: '--text-strong',
};

/** The id of the layer that outlines the selected area, and of its GeoJSON source. */
export const SELECTION = 'firecal-selection';

const OPENFREEMAP = 'https://tiles.openfreemap.org';

/**
 * A quiet basemap on OpenFreeMap's OpenStreetMap vector tiles: paper land, grey water, borders and a few place
 * names, no color, so what stands out is the selection and the data. The style is local, so with no network the
 * map still draws its paper and the selection.
 */
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
      { id: 'water', type: 'fill', source: 'openmaptiles', 'source-layer': 'water', paint: { 'fill-color': colors.water } },
      border('regions', 4, colors.region, 4),
      border('countries', 2, colors.border, 0),
      placeNames('country-names', ['==', ['get', 'class'], 'country'], 12, 0, colors.label, colors.land),
      placeNames('city-names', ['==', ['get', 'class'], 'city'], 11, 5, colors.label, colors.land),
      { id: SELECTION, type: 'line', source: SELECTION, paint: { 'line-color': colors.selection, 'line-width': 2 } },
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
