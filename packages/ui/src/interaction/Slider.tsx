"use client";

import { useCallback, useMemo } from "react";
import type { ComponentProps } from "react";
import * as SliderPrimitive from "@radix-ui/react-slider";

import type { ReactNode } from "react";

import { cn } from "../utils/cn";
import { GripVerticalIcon } from "../icons";
import { Label } from "../typography/Label";
import { Text } from "../typography/Text";
import { Button } from "./Button";

interface SliderPreset {
  value: number;
  label: string;
}

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
  // Sort presets by value ascending so indices map left-to-right
  const sorted = useMemo(
    () =>
      presets && presets.length > 0 ? [...presets].sort((a, b) => a.value - b.value) : undefined,
    [presets],
  );

  // When presets exist, the slider operates on indices (0..n-1) with step=1.
  // We convert between preset values and indices at the boundary.
  const hasPresets = sorted && sorted.length > 0;

  const valueToIndex = useCallback(
    (v: number) => {
      if (!sorted) return 0;
      let closest = 0;
      for (let i = 1; i < sorted.length; i++) {
        if (Math.abs(sorted[i]!.value - v) < Math.abs(sorted[closest]!.value - v)) {
          closest = i;
        }
      }
      return closest;
    },
    [sorted],
  );

  const sliderMin = hasPresets ? 0 : min;
  const sliderMax = hasPresets ? sorted.length - 1 : max;

  const _value = useMemo(() => {
    if (hasPresets) {
      const v = value ?? defaultValue ?? [sorted[0]!.value];
      return [valueToIndex(v[0]!)];
    }
    return value ?? defaultValue ?? [min];
  }, [value, defaultValue, min, hasPresets, sorted, valueToIndex]);

  const handleValueChange = useCallback(
    (values: number[]) => {
      if (hasPresets) {
        const index = values[0]!;
        onValueChange?.([sorted[index]!.value]);
      } else {
        onValueChange?.(values);
      }
    },
    [onValueChange, hasPresets, sorted],
  );

  const handlePresetClick = useCallback(
    (presetValue: number) => {
      onValueChange?.([presetValue]);
    },
    [onValueChange],
  );

  // Derive the value annotation for the header row.
  // For presets, use the same index mapping as the thumb so they always agree.
  const annotation = useMemo(() => {
    if (!label) return null;
    const current = value?.[0] ?? defaultValue?.[0];
    if (current == null) return null;
    if (sorted && sorted.length > 0) {
      const index = valueToIndex(current);
      return sorted[index]?.label ?? null;
    }
    return String(current);
  }, [label, value, defaultValue, sorted, valueToIndex]);

  return (
    <div className={cn("flex min-w-0 flex-col gap-3", className)}>
      {label && (
        <div className="flex items-center justify-between gap-4">
          {typeof label === "string" ? <Label>{label}</Label> : label}
          {annotation && (
            <Text size="xs" mono color="muted">
              {annotation}
            </Text>
          )}
        </div>
      )}
      <SliderPrimitive.Root
        data-slot="slider"
        defaultValue={undefined}
        value={_value}
        min={sliderMin}
        max={sliderMax}
        step={hasPresets ? 1 : undefined}
        onValueChange={handleValueChange}
        className="relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50"
        {...props}
      >
        <SliderPrimitive.Track className="bg-input border border-border relative h-4 w-full grow overflow-hidden rounded-full">
          <SliderPrimitive.Range className="bg-primary absolute h-full" />
        </SliderPrimitive.Track>
        {Array.from({ length: _value.length }, (_, index) => (
          <Button
            key={index}
            asChild
            variant="primary"
            elevation="sm"
            spring="bouncy"
            className="rounded-full"
          >
            <SliderPrimitive.Thumb
              aria-describedby={ariaDescribedBy}
              aria-valuetext={ariaValueText}
              className="flex items-center justify-center size-8 ring-0 disabled:pointer-events-none"
            >
              <GripVerticalIcon strokeWidth={3} className="size-3.5 shrink-0" />
            </SliderPrimitive.Thumb>
          </Button>
        ))}
      </SliderPrimitive.Root>
      {sorted && sorted.length > 0 && (
        <div className="relative h-4 w-full">
          {sorted.map((preset, i) => {
            const currentValue = value?.[0] ?? defaultValue?.[0];
            const isActive = currentValue === preset.value;
            const position = sorted.length > 1 ? (i / (sorted.length - 1)) * 100 : 50;
            const isFirst = i === 0;
            const isLast = i === sorted.length - 1;
            return (
              <button
                key={preset.value}
                type="button"
                onClick={() => handlePresetClick(preset.value)}
                className={cn(
                  "absolute text-xs cursor-pointer transition-colors duration-fast",
                  isFirst ? "translate-x-0" : isLast ? "-translate-x-full" : "-translate-x-1/2",
                  isActive ? "text-foreground font-medium" : "text-muted-foreground",
                )}
                style={{ left: `${position}%` }}
              >
                {preset.label}
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}

export { Slider };
export type { SliderPreset };
