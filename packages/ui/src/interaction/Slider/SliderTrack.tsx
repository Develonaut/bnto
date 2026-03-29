"use client";

import type { ComponentProps } from "react";
import * as SliderPrimitive from "@radix-ui/react-slider";

import { SliderThumb } from "./SliderThumb";
import type { useSliderState } from "./useSliderState";

type SliderTrackProps = {
  state: ReturnType<typeof useSliderState>;
  max: number;
  ariaDescribedBy?: string;
  ariaValueText?: string;
} & ComponentProps<typeof SliderPrimitive.Root>;

export function SliderTrack({
  state,
  max,
  ariaDescribedBy,
  ariaValueText,
  ...props
}: SliderTrackProps) {
  return (
    <SliderPrimitive.Root
      data-slot="slider"
      defaultValue={undefined}
      value={state.resolved}
      min={state.sliderMin}
      max={state.sliderMax ?? max}
      step={state.hasPresets ? 1 : undefined}
      onValueChange={state.handleChange}
      className="relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50"
      {...props}
    >
      <SliderPrimitive.Track className="bg-input border border-[var(--surface-muted-wall)] relative h-4 w-full grow overflow-hidden rounded-full">
        <SliderPrimitive.Range className="bg-primary absolute h-full" />
      </SliderPrimitive.Track>
      {Array.from({ length: state.resolved.length }, (_, i) => (
        <SliderThumb key={i} ariaDescribedBy={ariaDescribedBy} ariaValueText={ariaValueText} />
      ))}
    </SliderPrimitive.Root>
  );
}
