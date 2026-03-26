import type { PointerEvent, RefObject } from "react";

/** Update cursor and thumb hover state when not dragging. */
export function updateIdleCursor(
  e: PointerEvent,
  containerRef: RefObject<HTMLDivElement | null>,
  thumbRef: RefObject<HTMLDivElement | null>,
  checkArc: (x: number, y: number) => boolean,
  setIsHovering: (v: boolean) => void,
) {
  if (!containerRef.current) return;
  containerRef.current.style.cursor = checkArc(e.clientX, e.clientY) ? "pointer" : "default";

  if (!thumbRef.current) return;
  const rect = thumbRef.current.getBoundingClientRect();
  const dx = e.clientX - (rect.left + rect.width / 2);
  const dy = e.clientY - (rect.top + rect.height / 2);
  setIsHovering(dx * dx + dy * dy <= (rect.width / 2 + 4) ** 2);
}
