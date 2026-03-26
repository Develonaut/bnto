"use client";

import { useCallback } from "react";
import type { PointerEvent, RefObject } from "react";

import { useRadialEmit } from "./useRadialEmit";
import { useRadialDragState } from "./useRadialDragState";
import { updateIdleCursor } from "./updateIdleCursor";

interface UseRadialPointerOptions {
  containerRef: RefObject<HTMLDivElement | null>;
  thumbRef: RefObject<HTMLDivElement | null>;
  min: number;
  max: number;
  startAngle: number;
  endAngle: number;
  onChange: (value: number) => void;
}

export function useRadialPointer(opts: UseRadialPointerOptions) {
  const { containerRef, thumbRef } = opts;
  const drag = useRadialDragState();
  const { emitValue, checkArc } = useRadialEmit(
    containerRef, opts.min, opts.max, opts.startAngle, opts.endAngle, opts.onChange,
  );
  const onPointerDown = useCallback((e: PointerEvent) => {
    if (!checkArc(e.clientX, e.clientY)) return;
    drag.startDrag();
    containerRef.current?.setPointerCapture(e.pointerId);
    emitValue(e.clientX, e.clientY);
  }, [containerRef, checkArc, emitValue, drag.startDrag]);
  const onPointerMove = useCallback((e: PointerEvent) => {
    if (drag.draggingRef.current) return emitValue(e.clientX, e.clientY);
    updateIdleCursor(e, containerRef, thumbRef, checkArc, drag.setIsHovering);
  }, [containerRef, thumbRef, checkArc, emitValue, drag.draggingRef, drag.setIsHovering]);
  return { isDragging: drag.isDragging, isHovering: drag.isHovering, onPointerDown, onPointerMove, onPointerUp: drag.stopDrag, clearHover: drag.clearHover };
}
