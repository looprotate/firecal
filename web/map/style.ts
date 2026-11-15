import type { FilterSpecification, LayerSpecification, StyleSpecification } from 'maplibre-gl';

/** The token colors the basemap is drawn in, read from `web/tokens/tokens.css`. */
export interface MapColors {
  land: string;
  water: string;
  border: string;
  region: string;
  label: string;
  selection: string;
  coverage: string;
}

/** Where a map color comes from: a token, mixed with the paper (`--bg`) when `share` is under one. */
export interface MapColorSource {
  token: string;
  /** How much of the token, 0 to 1, against the paper; the rest is paper. */
  share: number;
}

/**
 * The custom property each map color comes from. Land and water are the pastels of the site's map (`site/style.css`:
 * `--land` and `--sea`), so the app's map and the film's are the same drawing; the selection is ink.
 */
export const MAP_COLOR_SOURCES: Record<keyof MapColors, MapColorSource> = {
  land: { token: '--healthy', share: 0.4 },
  water: { token: '--uncertainty', share: 0.18 },
  border: { token: '--text-secondary', share: 1 },
  region: { token: '--healthy', share: 0.7 },
  label: { token: '--text', share: 1 },
  selection: { token: '--text-strong', share: 1 },
  coverage: { token: '--activity', share: 1 },
};

/** `color` mixed with `paper`, `share` of the first, as CSS `color-mix(in srgb, …)` does: per channel, rounded. */
export function mixWithPaper(color: string, paper: string, share: number): string {
  const channels = (hex: string): number[] => [1, 3, 5].map((at) => Number.parseInt(hex.slice(at, at + 2), 16));
  const [ink, base] = [channels(color), channels(paper)];
  return `#${ink.map((value, at) => Math.round(value * share + (base[at] ?? 0) * (1 - share)).toString(16).padStart(2, '0')).join('')}`;
}

/** The id of the layer that outlines the selected area, and of its GeoJSON source. */
export const SELECTION = 'firecal-selection';
/** The id of the layer that outlines the region the host has tiles for, and of its GeoJSON source. */
export const COVERAGE = 'firecal-coverage';

const OPENFREEMAP = 'https://tiles.openfreemap.org';

/**
 * A quiet basemap on OpenFreeMap's OpenStreetMap vector tiles, in the pastels of the site: green land, sky water,
 * borders and a few place names, so what stands out is the ink selection and the data. The style is local, so with no network the
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
      [COVERAGE]: { type: 'geojson', data: { type: 'FeatureCollection', features: [] } },
      [SELECTION]: { type: 'geojson', data: { type: 'FeatureCollection', features: [] } },
    },
    layers: [
      { id: 'land', type: 'background', paint: { 'background-color': colors.land } },
      { id: 'water', type: 'fill', source: 'openmaptiles', 'source-layer': 'water', paint: { 'fill-color': colors.water } },
      border('regions', 4, colors.region, 4),
      border('countries', 2, colors.border, 0),
      placeNames('country-names', ['==', ['get', 'class'], 'country'], 12, 0, colors.label, colors.land),
      placeNames('city-names', ['==', ['get', 'class'], 'city'], 11, 5, colors.label, colors.land),
      {
        id: COVERAGE,
        type: 'line',
        source: COVERAGE,
        paint: { 'line-color': colors.coverage, 'line-width': 2.5, 'line-dasharray': [2, 2] },
        layout: { 'line-cap': 'round', 'line-join': 'round' },
      },
      { id: SELECTION, type: 'line', source: SELECTION, paint: { 'line-color': colors.selection, 'line-width': 3 }, layout: { 'line-cap': 'round', 'line-join': 'round' } },
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
