"use client";

import { useRef, useState, useCallback } from "react";
import type { PointerEvent, RefObject } from "react";

import { useRadialEmit } from "./useRadialEmit";
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

export function useRadialPointer({
  containerRef,
  thumbRef,
  min,
  max,
  startAngle,
  endAngle,
  onChange,
}: UseRadialPointerOptions) {
  const [isDragging, setIsDragging] = useState(false);
  const [isHovering, setIsHovering] = useState(false);
  const draggingRef = useRef(false);
  const { emitValue, checkArc } = useRadialEmit(
    containerRef,
    min,
    max,
    startAngle,
    endAngle,
    onChange,
  );

  const onPointerDown = useCallback(
    (e: PointerEvent) => {
      if (!checkArc(e.clientX, e.clientY)) return;
      draggingRef.current = true;
      setIsDragging(true);
      containerRef.current?.setPointerCapture(e.pointerId);
      emitValue(e.clientX, e.clientY);
    },
    [containerRef, checkArc, emitValue],
  );

  const onPointerMove = useCallback(
    (e: PointerEvent) => {
      if (draggingRef.current) return emitValue(e.clientX, e.clientY);
      updateIdleCursor(e, containerRef, thumbRef, checkArc, setIsHovering);
    },
    [containerRef, thumbRef, checkArc, emitValue],
  );

  const onPointerUp = useCallback(() => {
    draggingRef.current = false;
    setIsDragging(false);
  }, []);
  const clearHover = useCallback(() => setIsHovering(false), []);

  return { isDragging, isHovering, onPointerDown, onPointerMove, onPointerUp, clearHover };
}
