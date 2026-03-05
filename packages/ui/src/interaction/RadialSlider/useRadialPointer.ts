"use client";

import { useRef, useState, useCallback } from "react";
import type { PointerEvent } from "react";

import { angleToValue, pointerToAngle } from "./valueMapping";

interface UseRadialPointerOptions {
  containerRef: React.RefObject<HTMLDivElement | null>;
  thumbRef: React.RefObject<HTMLDivElement | null>;
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

  const updateFromPointer = useCallback(
    (clientX: number, clientY: number) => {
      if (!containerRef.current) return;
      const rect = containerRef.current.getBoundingClientRect();
      const angle = pointerToAngle(clientX, clientY, rect);
      onChange(angleToValue(angle, min, max, startAngle, endAngle));
    },
    [containerRef, min, max, startAngle, endAngle, onChange],
  );

  /** Check if a pointer angle falls within the arc (with a small margin). */
  const isInArc = useCallback(
    (clientX: number, clientY: number) => {
      if (!containerRef.current) return false;
      let span = endAngle - startAngle;
      if (span < 0) span += 360;
      if (span >= 360) return true;

      const rect = containerRef.current.getBoundingClientRect();
      const angle = pointerToAngle(clientX, clientY, rect);
      let offset = angle - startAngle;
      if (offset < 0) offset += 360;
      return offset <= span + 20 || offset >= 360 - 20;
    },
    [containerRef, startAngle, endAngle],
  );

  const onPointerDown = useCallback(
    (e: PointerEvent) => {
      if (!isInArc(e.clientX, e.clientY)) return;
      draggingRef.current = true;
      setIsDragging(true);
      containerRef.current?.setPointerCapture(e.pointerId);
      updateFromPointer(e.clientX, e.clientY);
    },
    [containerRef, isInArc, updateFromPointer],
  );

  const onPointerMove = useCallback(
    (e: PointerEvent) => {
      if (draggingRef.current) {
        updateFromPointer(e.clientX, e.clientY);
      } else if (containerRef.current) {
        const inArc = isInArc(e.clientX, e.clientY);
        containerRef.current.style.cursor = inArc ? "pointer" : "default";

        if (thumbRef.current) {
          const rect = thumbRef.current.getBoundingClientRect();
          const dx = e.clientX - (rect.left + rect.width / 2);
          const dy = e.clientY - (rect.top + rect.height / 2);
          setIsHovering(dx * dx + dy * dy <= (rect.width / 2 + 4) ** 2);
        }
      }
    },
    [containerRef, thumbRef, isInArc, updateFromPointer],
  );

  const onPointerUp = useCallback(() => {
    draggingRef.current = false;
    setIsDragging(false);
  }, []);

  const clearHover = useCallback(() => setIsHovering(false), []);

  return {
    isDragging,
    isHovering,
    onPointerDown,
    onPointerMove,
    onPointerUp,
    clearHover,
  };
}
