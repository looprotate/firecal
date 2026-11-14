import {
  Map,
  NavigationControl,
  setWorkerUrl,
} from 'maplibre-gl';
import workerUrl from 'maplibre-gl/dist/maplibre-gl-worker.mjs?worker&url';
import { MaplibreTerradrawControl } from '@watergis/maplibre-gl-terradraw';
import { areaFromRectangleCoordinates, selectionFromPolygonCoordinates } from '../app/area-input.ts';
import type { AreaSelection } from '../app/area-input.ts';

setWorkerUrl(workerUrl);

/** Creates the map, rectangle tool and validated area-selection callback. */
export function createAreaMap(container: HTMLElement, onAreaSelected: (selection: AreaSelection) => void): Map {
  const map = new Map({
    container,
    style: 'https://tiles.openfreemap.org/styles/liberty',
    center: [-60, -18],
    zoom: 3,
    attributionControl: { compact: true },
  });

  map.addControl(new NavigationControl({ showCompass: false }), 'top-right');
  const drawingControl = new MaplibreTerradrawControl({
    modes: ['rectangle', 'polygon', 'select', 'delete-selection', 'delete'],
    open: true,
  });
  map.addControl(drawingControl, 'top-left');
  map.once('load', () => {
    const drawing = drawingControl.getTerraDrawInstance();
    if (!drawing) return;
    drawing.on('finish', (id, context) => {
      if (context.mode !== 'rectangle' && context.mode !== 'polygon') return;
      const feature = drawing.getSnapshotFeature(id);
      if (!feature || feature.geometry.type !== 'Polygon') return;
      const ring = feature.geometry.coordinates[0];
      if (!ring) return;
      if (context.mode === 'rectangle') {
        const area = areaFromRectangleCoordinates(ring);
        if (area) onAreaSelected({ area });
        return;
      }
      const selection = selectionFromPolygonCoordinates(ring);
      if (selection) onAreaSelected(selection);
    });
  });
  return map;
}
