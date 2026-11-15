import { parseAreaBounds, selectionFromPolygonCoordinates } from './area-input.ts';
import { parseMonth, monthCode, type MonthRange } from './period.ts';
import type { Area, Series } from './series.ts';
export interface SharedQuery { area: Area; polygon?: [number,number][]; plane: Series['plane']; period?: MonthRange; month?: number }
/** Versioned URLs carry only a bounded geographic query, never data or credentials. */
export function parseSharedQuery(search: string): SharedQuery | null {
  if (search.length > 12000) return null;
  const p = new URLSearchParams(search);
  if (p.get('v') !== '1') return null;
  const coordinates = (p.get('box') ?? '').split(',');
  if (coordinates.length !== 4) return null;
  const result = parseAreaBounds({ west: coordinates[0]!, south: coordinates[1]!, east: coordinates[2]!, north: coordinates[3]! });
  if (!result.area) return null;
  const plane = p.get('mode');
  if (plane !== 'count' && plane !== 'energy') return null;
  let polygon: [number,number][] | undefined;
  if (p.has('poly')) {
    try {
      const points: unknown = JSON.parse(p.get('poly')!);
      if (!Array.isArray(points) || points.length < 3 || points.length > 128 || points.some(point => !Array.isArray(point) || point.length !== 2 || point.some(n => typeof n !== 'number' || !Number.isFinite(n)))) return null;
      const ring = points.map(point => [point[1],point[0]]);
      ring.push([...ring[0]!]);
      const selection = selectionFromPolygonCoordinates(ring);
      if (!selection) return null;
      polygon = selection.polygon;
      if (!polygon || Object.entries(result.area).some(([key,value]) => Math.abs(value - selection.area[key as keyof Area]) > 0.00001)) return null;
    } catch { return null; }
  }
  let period: MonthRange | undefined;
  if (p.has('from') || p.has('to')) {
    const first=parseMonth(p.get('from') ?? ''),last=parseMonth(p.get('to') ?? '');
    if (first===null || last===null || first>last) return null;
    period={first,last};
  }
  const month = p.has('month') ? parseMonth(p.get('month')!) : null;
  if (p.has('month') && month===null) return null;
  return {area:result.area,plane, ...(polygon?{polygon}:{}), ...(period?{period}:{}), ...(month!==null?{month}:{})};
}
export function sharedUrl(base: string, query: SharedQuery): string {
  const url=new URL(base);url.search='';url.hash='';
  url.searchParams.set('v','1');
  url.searchParams.set('box',[query.area.west,query.area.south,query.area.east,query.area.north].join(','));
  url.searchParams.set('mode',query.plane);
  if(query.polygon)url.searchParams.set('poly',JSON.stringify(query.polygon));
  if(query.period){url.searchParams.set('from',monthCode(query.period.first));url.searchParams.set('to',monthCode(query.period.last));}
  if(query.month!==undefined)url.searchParams.set('month',monthCode(query.month));
  if(!parseSharedQuery(url.search))throw new Error('This query cannot be shared.');
  return url.href;
}
