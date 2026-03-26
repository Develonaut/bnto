"use client";

import { useMemo } from "react";
import type { ComponentProps, ReactNode } from "react";
import type * as SliderPrimitive from "@radix-ui/react-slider";

import { cn } from "../utils/cn";
import { Label } from "../typography/Label";
import { Text } from "../typography/Text";
import { PresetLabels } from "./SliderPresets";
import type { SliderPreset } from "./SliderPresets";
import { useSliderState } from "./useSliderState";
import { SliderTrack } from "./SliderTrack";

interface SliderProps extends ComponentProps<typeof SliderPrimitive.Root> {
  presets?: SliderPreset[];
  /** When provided, renders a header row with the label and a value annotation. */
  label?: ReactNode;
}

function SliderHeader({ label, annotation }: { label: ReactNode; annotation: string | null }) {
  return (
    <div className="flex items-center justify-between gap-4">
      {typeof label === "string" ? <Label>{label}</Label> : label}
      {annotation && (
        <Text size="xs" mono color="muted">
          {annotation}
        </Text>
      )}
    </div>
  );
}

function useAnnotation(
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

function Slider({
  className,
  defaultValue,
  value,
  min = 0,
  max = 100,
  presets,
  label,
  onValueChange,
  "aria-describedby": ariaDescribedBy,
  "aria-valuetext": ariaValueText,
  ...props
}: SliderProps) {
  const state = useSliderState(presets, value, defaultValue, min, onValueChange);
  const annotation = useAnnotation(label, value, defaultValue, state.sorted, state.valueToIndex);

  return (
    <div className={cn("flex min-w-0 flex-col gap-3", className)}>
      {label && <SliderHeader label={label} annotation={annotation} />}
      <SliderTrack
        state={state}
        max={max}
        ariaDescribedBy={ariaDescribedBy}
        ariaValueText={ariaValueText}
        {...props}
      />
      {state.sorted && state.sorted.length > 0 && (
        <PresetLabels
          sorted={state.sorted}
          value={value}
          defaultValue={defaultValue}
          onPresetClick={state.handlePresetClick}
        />
      )}
    </div>
  );
}

export { Slider };
export type { SliderPreset };
