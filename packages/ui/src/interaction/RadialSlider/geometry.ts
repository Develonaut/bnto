/** Convert degrees to radians. */
export function toRad(deg: number) {
  return (deg * Math.PI) / 180;
}

/**
 * CSS-convention angle (0 deg = 12 o'clock, CW) -> {x, y} on a circle.
 * Returns coordinates relative to the circle center (0, 0).
 */
export function polarToOffset(angleDeg: number, r: number) {
  const rad = toRad(angleDeg - 90);
  return { x: r * Math.cos(rad), y: r * Math.sin(rad) };
}

/** Same as polarToOffset but relative to (cx, cy) for SVG paths. */
function polarToXY(angleDeg: number, r: number, cx: number, cy: number) {
  const { x, y } = polarToOffset(angleDeg, r);
  return { x: cx + x, y: cy + y };
}

/**
 * Build an SVG arc path between two CSS-convention angles.
 * Full-circle rendered as two semicircles (SVG degenerate arc fix).
 */
export function arcPath(from: number, to: number, r: number, cx: number, cy: number) {
  let sweep = to - from;
  if (sweep < 0) sweep += 360;
  if (sweep < 0.1) return "";

  if (sweep >= 359.99) {
    const top = polarToXY(from, r, cx, cy);
    const bottom = polarToXY(from + 180, r, cx, cy);
    return [
      `M ${top.x} ${top.y}`,
      `A ${r} ${r} 0 1 1 ${bottom.x} ${bottom.y}`,
      `A ${r} ${r} 0 1 1 ${top.x} ${top.y}`,
    ].join(" ");
  }

  const start = polarToXY(from, r, cx, cy);
  const end = polarToXY(to, r, cx, cy);
  const largeArc = sweep > 180 ? 1 : 0;
  return `M ${start.x} ${start.y} A ${r} ${r} 0 ${largeArc} 1 ${end.x} ${end.y}`;
}
