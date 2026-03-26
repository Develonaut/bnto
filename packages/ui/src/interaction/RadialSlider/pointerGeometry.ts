import { angleToValue, pointerToAngle } from "./valueMapping";

/** Compute clamped value from pointer position relative to container center. */
export function valueFromPointer(
  clientX: number,
  clientY: number,
  rect: DOMRect,
  min: number,
  max: number,
  startAngle: number,
  endAngle: number,
) {
  const angle = pointerToAngle(clientX, clientY, rect);
  return angleToValue(angle, min, max, startAngle, endAngle);
}

/** Check if a pointer angle falls within the arc (with a small margin). */
export function isPointerInArc(
  clientX: number,
  clientY: number,
  rect: DOMRect,
  startAngle: number,
  endAngle: number,
) {
  let span = endAngle - startAngle;
  if (span < 0) span += 360;
  if (span >= 360) return true;

  const angle = pointerToAngle(clientX, clientY, rect);
  let offset = angle - startAngle;
  if (offset < 0) offset += 360;
  return offset <= span + 20 || offset >= 360 - 20;
}
