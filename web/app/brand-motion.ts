interface Viewport { width: number; height: number }
interface Destination { x: number; y: number; width: number; height: number }

/** The same logo starts centered and finishes at its measured header position. */
export function introPlacement(viewport: Viewport, target: Destination): { x: number; y: number; scale: number; endX: number; endY: number } | null {
  if (viewport.width <= 0 || viewport.height <= 0 || target.width <= 0 || target.height <= 0) return null;
  const size = Math.min(176, viewport.width * 0.36, viewport.height * 0.3);
  const scale = size / Math.max(target.width, target.height);
  return { x: (viewport.width - target.width * scale) / 2, y: (viewport.height - target.height * scale) / 2 - 24, scale, endX: target.x, endY: target.y };
}
