"use client";

import { useMemo } from "react";
import type { ReactNode } from "react";
import type { SliderPreset } from "./SliderPresets";

/** Derive the value annotation string shown in the slider header. */
export function useSliderAnnotation(
  label: ReactNode,
  value: number[] | undefined,
  defaultValue: number[] | undefined,
  sorted: SliderPreset[] | undefined,
  valueToIndex: (v: number) => number,
) {
  return useMemo(() => {
    if (!label) return null;
    const current = value?.[0] ?? defaultValue?.[0];
    if (current == null) return null;
    if (sorted && sorted.length > 0) return sorted[valueToIndex(current)]?.label ?? null;
    return String(current);
  }, [label, value, defaultValue, sorted, valueToIndex]);
}
