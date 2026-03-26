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

function resolveKeyValue(key: string, value: number, min: number, max: number, step: number) {
  switch (key) {
    case "ArrowRight":
    case "ArrowUp":
      return Math.min(max, value + step);
    case "ArrowLeft":
    case "ArrowDown":
      return Math.max(min, value - step);
    case "Home":
      return min;
    case "End":
      return max;
    default:
      return null;
  }
}

export function useRadialKeyboard({ min, max, value, step, onChange }: UseRadialKeyboardOptions) {
  return useCallback(
    (e: KeyboardEvent) => {
      const next = resolveKeyValue(e.key, value, min, max, step);
      if (next === null) return;
      e.preventDefault();
      onChange(next);
    },
    [min, max, value, step, onChange],
  );
}
