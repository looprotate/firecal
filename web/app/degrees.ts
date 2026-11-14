/** A coordinate as it is typed: decimal degrees to a thousandth, about 100 m, with no trailing zeros. */
export function degreesText(value: number): string {
  const rounded = Number(value.toFixed(3));
  return String(Object.is(rounded, -0) ? 0 : rounded);
}

/** A latitude as it is read: `16.5° S`. */
export function latitudeLabel(value: number): string {
  return `${degreesText(Math.abs(value))}° ${value < 0 ? 'S' : 'N'}`;
}

/** A longitude as it is read: `58.071° W`. */
export function longitudeLabel(value: number): string {
  return `${degreesText(Math.abs(value))}° ${value < 0 ? 'W' : 'E'}`;
}
