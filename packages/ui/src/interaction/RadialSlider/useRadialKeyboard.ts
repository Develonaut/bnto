"use client";

import { useCallback } from "react";
import type { KeyboardEvent } from "react";

interface UseRadialKeyboardOptions {
  min: number;
  max: number;
  value: number;
  step: number;
  onChange: (value: number) => void;
}

export function useRadialKeyboard({
  min,
  max,
  value,
  step,
  onChange,
}: UseRadialKeyboardOptions) {
  return useCallback(
    (e: KeyboardEvent) => {
      let next: number | null = null;
      switch (e.key) {
        case "ArrowRight":
        case "ArrowUp":
          next = Math.min(max, value + step);
          break;
        case "ArrowLeft":
        case "ArrowDown":
          next = Math.max(min, value - step);
          break;
        case "Home":
          next = min;
          break;
        case "End":
          next = max;
          break;
        default:
          return;
      }
      e.preventDefault();
      onChange(next);
    },
    [min, max, value, step, onChange],
  );
}
