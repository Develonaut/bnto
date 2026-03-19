"use client";

import { useCallback, useMemo } from "react";
import { Label, Slider } from "@bnto/ui";
import type { SliderPreset } from "@bnto/ui";
import type { ControlProps } from "./types";

/** Convert between stored and display values when displayInverted is set. */
function invertValue(value: number, min: number, max: number): number {
  return min + max - value;
}

function SliderControl({ id, fieldInfo, meta, fieldConfig, value, onChange }: ControlProps) {
  const min = fieldInfo.min ?? 0;
  const max = fieldInfo.max ?? 100;
  const storedValue = typeof value === "number" ? value : min;
  const inverted = fieldConfig?.inverted ?? false;

  // Display value: invert if needed so the slider matches user mental model
  const displayValue = inverted ? invertValue(storedValue, min, max) : storedValue;

  // Convert presets from stored-value space to display-value space,
  // sorted ascending by display value so justify-between aligns correctly
  const presets = fieldConfig?.presets;
  const displayPresets = useMemo((): SliderPreset[] | undefined => {
    if (!presets) return undefined;
    if (!inverted) return [...presets].sort((a, b) => a.value - b.value);
    return presets
      .map((p) => ({ ...p, value: invertValue(p.value, min, max) }))
      .sort((a, b) => a.value - b.value);
  }, [presets, inverted, min, max]);

  const sliderValue = useMemo(() => [displayValue], [displayValue]);

  const handleValueChange = useCallback(
    (values: number[]) => {
      const display = values[0]!;
      onChange(inverted ? invertValue(display, min, max) : display);
    },
    [onChange, inverted, min, max],
  );

  const hasPresets = displayPresets && displayPresets.length > 0;

  const sliderLabel = (
    <Label htmlFor={id} title={meta.description}>
      {fieldConfig?.label ?? meta.label}
      {fieldInfo.required && <span className="ml-0.5 text-destructive">*</span>}
    </Label>
  );

  return (
    <Slider
      id={id}
      min={min}
      max={max}
      value={sliderValue}
      label={sliderLabel}
      presets={hasPresets ? displayPresets : undefined}
      onValueChange={handleValueChange}
      data-testid={`control-slider-${id}`}
    />
  );
}

export { SliderControl };
