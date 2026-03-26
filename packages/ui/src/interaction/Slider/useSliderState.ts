"use client";

import { useCallback, useMemo } from "react";

import { useSortedPresets, useValueToIndex } from "./SliderPresets";
import type { SliderPreset } from "./SliderPresets";

function useResolvedValue(
  hasPresets: boolean,
  sorted: SliderPreset[] | undefined,
  valueToIndex: (v: number) => number,
  value: number[] | undefined,
  defaultValue: number[] | undefined,
  min: number,
) {
  return useMemo(() => {
    if (!hasPresets) return value ?? defaultValue ?? [min];
    const first = sorted as SliderPreset[];
    const v = value ?? defaultValue ?? [first[0].value];
    return [valueToIndex(v[0] as number)];
  }, [value, defaultValue, min, hasPresets, sorted, valueToIndex]);
}

function usePresetChange(
  hasPresets: boolean,
  sorted: SliderPreset[] | undefined,
  onValueChange: ((v: number[]) => void) | undefined,
) {
  const handleChange = useCallback(
    (values: number[]) => {
      if (hasPresets) {
        onValueChange?.([(sorted as SliderPreset[])[values[0] as number].value]);
      } else {
        onValueChange?.(values);
      }
    },
    [onValueChange, hasPresets, sorted],
  );

  const handlePresetClick = useCallback(
    (presetValue: number) => () => onValueChange?.([presetValue]),
    [onValueChange],
  );

  return { handleChange, handlePresetClick };
}

export function useSliderState(
  presets: SliderPreset[] | undefined,
  value: number[] | undefined,
  defaultValue: number[] | undefined,
  min: number,
  onValueChange: ((v: number[]) => void) | undefined,
) {
  const sorted = useSortedPresets(presets);
  const valueToIndex = useValueToIndex(sorted);
  const hasPresets = !!sorted && sorted.length > 0;
  const sliderMin = hasPresets ? 0 : min;
  const sliderMax = hasPresets ? (sorted as SliderPreset[]).length - 1 : undefined;
  const resolved = useResolvedValue(hasPresets, sorted, valueToIndex, value, defaultValue, min);
  const { handleChange, handlePresetClick } = usePresetChange(hasPresets, sorted, onValueChange);

  return {
    sorted,
    valueToIndex,
    hasPresets,
    sliderMin,
    sliderMax,
    resolved,
    handleChange,
    handlePresetClick,
  };
}
