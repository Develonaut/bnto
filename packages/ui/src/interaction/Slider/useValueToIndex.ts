"use client";

import { useCallback } from "react";
import type { SliderPreset } from "./SliderPresets";

/** Map a raw value to the closest preset index. */
export function useValueToIndex(sorted?: SliderPreset[]) {
  return useCallback(
    (v: number) => {
      if (!sorted) return 0;
      let closest = 0;
      for (let i = 1; i < sorted.length; i++) {
        if (
          Math.abs((sorted[i] as SliderPreset).value - v) <
          Math.abs((sorted[closest] as SliderPreset).value - v)
        )
          closest = i;
      }
      return closest;
    },
    [sorted],
  );
}
