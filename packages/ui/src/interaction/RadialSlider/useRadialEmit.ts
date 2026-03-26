"use client";

import { useCallback } from "react";
import type { RefObject } from "react";

import { isPointerInArc, valueFromPointer } from "./pointerGeometry";

/** Emit a clamped value from pointer position, and check if pointer is in arc. */
export function useRadialEmit(
  containerRef: RefObject<HTMLDivElement | null>,
  min: number,
  max: number,
  startAngle: number,
  endAngle: number,
  onChange: (value: number) => void,
) {
  const emitValue = useCallback(
    (clientX: number, clientY: number) => {
      const rect = containerRef.current?.getBoundingClientRect();
      if (!rect) return;
      onChange(valueFromPointer(clientX, clientY, rect, min, max, startAngle, endAngle));
    },
    [containerRef, min, max, startAngle, endAngle, onChange],
  );

  const checkArc = useCallback(
    (clientX: number, clientY: number) => {
      const rect = containerRef.current?.getBoundingClientRect();
      if (!rect) return false;
      return isPointerInArc(clientX, clientY, rect, startAngle, endAngle);
    },
    [containerRef, startAngle, endAngle],
  );

  return { emitValue, checkArc };
}
