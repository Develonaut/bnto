"use client";

import { useRef, useState, useCallback } from "react";

/** Tracks dragging and hover state for the radial pointer interaction. */
export function useRadialDragState() {
  const [isDragging, setIsDragging] = useState(false);
  const [isHovering, setIsHovering] = useState(false);
  const draggingRef = useRef(false);

  const startDrag = useCallback(() => {
    draggingRef.current = true;
    setIsDragging(true);
  }, []);

  const stopDrag = useCallback(() => {
    draggingRef.current = false;
    setIsDragging(false);
  }, []);

  const clearHover = useCallback(() => setIsHovering(false), []);

  return { isDragging, isHovering, draggingRef, setIsHovering, startDrag, stopDrag, clearHover };
}
