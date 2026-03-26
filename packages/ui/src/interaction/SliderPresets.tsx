"use client";

import { useCallback, useMemo } from "react";

import { cn } from "../utils/cn";

interface SliderPreset {
  value: number;
  label: string;
}

/** Sort presets by value ascending so indices map left-to-right. */
export function useSortedPresets(presets?: SliderPreset[]) {
  return useMemo(
    () =>
      presets && presets.length > 0 ? [...presets].sort((a, b) => a.value - b.value) : undefined,
    [presets],
  );
}

/** Map a raw value to the closest preset index. */
export function useValueToIndex(sorted?: SliderPreset[]) {
  return useCallback(
    (v: number) => {
      if (!sorted) return 0;
      let closest = 0;
      for (let i = 1; i < sorted.length; i++) {
        if (Math.abs(sorted[i]!.value - v) < Math.abs(sorted[closest]!.value - v)) closest = i;
      }
      return closest;
    },
    [sorted],
  );
}

export function PresetLabels({
  sorted,
  value,
  defaultValue,
  onPresetClick,
}: {
  sorted: SliderPreset[];
  value?: number[];
  defaultValue?: number[];
  onPresetClick: (value: number) => () => void;
}) {
  return (
    <div className="relative h-4 w-full">
      {sorted.map((preset, i) => (
        <PresetLabel
          key={preset.value}
          preset={preset}
          index={i}
          total={sorted.length}
          isActive={(value?.[0] ?? defaultValue?.[0]) === preset.value}
          onClick={onPresetClick(preset.value)}
        />
      ))}
    </div>
  );
}

const alignmentMap = {
  first: "translate-x-0",
  last: "-translate-x-full",
  middle: "-translate-x-1/2",
} as const;

function resolveAlignment(index: number, total: number) {
  if (index === 0) return alignmentMap.first;
  if (index === total - 1) return alignmentMap.last;
  return alignmentMap.middle;
}

function PresetLabel({
  preset,
  index,
  total,
  isActive,
  onClick,
}: {
  preset: SliderPreset;
  index: number;
  total: number;
  isActive: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      data-testid="slider-preset"
      data-preset-label={preset.label}
      className={cn(
        "absolute text-xs cursor-pointer transition-colors duration-fast focus-ring rounded-sm",
        resolveAlignment(index, total),
        isActive ? "text-foreground font-medium" : "text-muted-foreground",
      )}
      style={{ left: `${total > 1 ? (index / (total - 1)) * 100 : 50}%` }}
    >
      {preset.label}
    </button>
  );
}

export type { SliderPreset };
