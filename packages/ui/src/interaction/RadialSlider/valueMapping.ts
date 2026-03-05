/** Map a numeric value to an angle on the arc. */
export function valueToAngle(
  value: number,
  min: number,
  max: number,
  startAngle: number,
  endAngle: number,
) {
  const t = Math.max(0, Math.min(1, (value - min) / (max - min)));
  let span = endAngle - startAngle;
  if (span < 0) span += 360;
  return startAngle + t * span;
}

/** Map an arbitrary angle back to a clamped value within [min, max]. */
export function angleToValue(
  rawAngle: number,
  min: number,
  max: number,
  startAngle: number,
  endAngle: number,
) {
  let span = endAngle - startAngle;
  if (span < 0) span += 360;

  let offset = rawAngle - startAngle;
  if (offset < 0) offset += 360;

  // For partial arcs, clamp when pointer is in the dead zone
  if (span < 360 && offset > span) {
    const pastEnd = offset - span;
    const deadZone = 360 - span;
    offset = pastEnd < deadZone / 2 ? span : 0;
  }

  const t = Math.max(0, Math.min(1, offset / span));
  return Math.round(min + t * (max - min));
}

/** Convert a pointer position to an angle relative to an element's center. */
export function pointerToAngle(clientX: number, clientY: number, rect: DOMRect) {
  const cx = rect.left + rect.width / 2;
  const cy = rect.top + rect.height / 2;
  const rad = Math.atan2(clientY - cy, clientX - cx);
  let deg = (rad * 180) / Math.PI + 90;
  if (deg < 0) deg += 360;
  return deg;
}
