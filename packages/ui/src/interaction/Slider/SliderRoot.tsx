"use client";

import type { ComponentProps, ReactNode } from "react";
import type * as SliderPrimitive from "@radix-ui/react-slider";

import { cn } from "../../utils/cn";
import { PresetLabels } from "./SliderPresets";
import type { SliderPreset } from "./SliderPresets";
import { useSliderState } from "./useSliderState";
import { useSliderAnnotation } from "./useSliderAnnotation";
import { SliderTrack } from "./SliderTrack";
import { SliderHeader } from "./SliderHeader";

interface SliderProps extends ComponentProps<typeof SliderPrimitive.Root> {
  presets?: SliderPreset[];
  /** When provided, renders a header row with the label and a value annotation. */
  label?: ReactNode;
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
  const annotation = useSliderAnnotation(
    label,
    value,
    defaultValue,
    state.sorted,
    state.valueToIndex,
  );

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
