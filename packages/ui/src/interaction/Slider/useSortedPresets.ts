"use client";

import { useMemo } from "react";
import type { SliderPreset } from "./SliderPresets";

/** Sort presets by value ascending so indices map left-to-right. */
export function useSortedPresets(presets?: SliderPreset[]) {
  return useMemo(
    () =>
      presets && presets.length > 0 ? [...presets].sort((a, b) => a.value - b.value) : undefined,
    [presets],
  );
}
