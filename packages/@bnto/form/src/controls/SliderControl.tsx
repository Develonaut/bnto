"use client";

import { Slider } from "@bnto/ui";
import type { ControlProps } from "./types";
import { FieldLabel } from "./FieldLabel";
import { useSliderState } from "./useSliderState";

function SliderControl({ id, fieldInfo, meta, fieldConfig, value, onChange }: ControlProps) {
  const { min = 0, max = 100 } = fieldInfo;
  const inverted = fieldConfig?.inverted ?? false;
  const { sliderValue, displayPresets, handleChange } = useSliderState({
    min,
    max,
    inverted,
    value,
    presets: fieldConfig?.presets,
    onChange,
  });
  const label = (
    <FieldLabel htmlFor={id} title={meta.description} required={fieldInfo.required}>
      {fieldConfig?.label ?? meta.label}
    </FieldLabel>
  );

  return (
    <Slider
      id={id}
      min={min}
      max={max}
      value={sliderValue}
      label={label}
      presets={displayPresets?.length ? displayPresets : undefined}
      onValueChange={handleChange}
      data-testid={`control-slider-${id}`}
    />
  );
}

export { SliderControl };
