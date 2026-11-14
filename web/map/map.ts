import {
  Map,
  NavigationControl,
  setWorkerUrl,
} from 'maplibre-gl';
import workerUrl from 'maplibre-gl/dist/maplibre-gl-worker.mjs?worker&url';
import { MaplibreTerradrawControl } from '@watergis/maplibre-gl-terradraw';

setWorkerUrl(workerUrl);

/** Starts the map shell without a remote basemap or an API key. */
export function createAreaMap(container: HTMLElement): Map {
  const map = new Map({
    container,
    style: {
      version: 8,
      sources: {},
      layers: [],
    },
    center: [-60, -18],
    zoom: 4,
    attributionControl: false,
  });

  map.addControl(new NavigationControl({ showCompass: false }), 'top-right');
  map.addControl(
    new MaplibreTerradrawControl({
      modes: ['rectangle', 'select', 'delete-selection', 'delete'],
      open: true,
    }),
    'top-left',
  );
  return map;
}
