import { CONTINENTS, continentArea } from './continents.ts';
import type { Area } from './series.ts';
export function regionLabel(area:Area):string {
  const place=CONTINENTS.find(item=>{const box=continentArea(item.id)!;return Object.entries(box).every(([key,value])=>Math.abs(value-area[key as keyof Area])<0.0001);});
  return place?`${place.capital} · ${place.country}`:`${area.south.toFixed(2)}° to ${area.north.toFixed(2)}°, ${area.west.toFixed(2)}° to ${area.east.toFixed(2)}°`;
}
