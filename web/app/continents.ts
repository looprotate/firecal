import type { Area } from './series.ts';

/** Each country is assigned once, following UN M49; Russia is grouped with Europe. */
export const CONTINENTS = [
  { id: 'north-america', name: 'North America', country: 'Canada', capital: 'Ottawa', latitude: 45.421, longitude: -75.697 },
  { id: 'south-america', name: 'South America', country: 'Brazil', capital: 'Brasília', latitude: -15.794, longitude: -47.883 },
  { id: 'asia', name: 'Asia', country: 'China', capital: 'Beijing', latitude: 39.904, longitude: 116.407 },
  { id: 'europe', name: 'Europe', country: 'Russia', capital: 'Moscow', latitude: 55.756, longitude: 37.617 },
  { id: 'africa', name: 'Africa', country: 'Algeria', capital: 'Algiers', latitude: 36.754, longitude: 3.059 },
  { id: 'oceania', name: 'Oceania', country: 'Australia', capital: 'Canberra', latitude: -35.281, longitude: 149.130 },
] as const;

/** A four-degree viewing area centered on the capital; unknown regions cannot create a query. */
export function continentArea(id: string): Area | null {
  const place = CONTINENTS.find((item) => item.id === id);
  if (!place) return null;
  const rounded = (value: number): number => Math.round(value * 1000) / 1000;
  return { west: rounded(place.longitude - 2), east: rounded(place.longitude + 2), south: rounded(place.latitude - 2), north: rounded(place.latitude + 2) };
}

/** One selection per inhabited continent, with Central America intentionally excluded. */
export function renderContinentPicker(): string {
  const options = CONTINENTS.map((place) => `<option value="${place.id}">${place.name} — ${place.country} · ${place.capital}</option>`).join('');
  return `<div class="continent-picker"><label for="continent-select">Explore by continent</label><select id="continent-select"><option value="">Choose a continent</option>${options}<option value="antarctica" disabled>Antarctica — no national capital</option></select><p>Largest country by area. Russia is grouped with Europe.</p></div>`;
}
