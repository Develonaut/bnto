"use client";

import { useMemo } from "react";
import type { ComponentProps } from "react";
import * as SliderPrimitive from "@radix-ui/react-slider";

import { cn } from "@/lib/cn";
import { GripVerticalIcon } from "@/components/ui/icons";
import { Pressable } from "./Pressable";
import { Surface } from "./Surface";

function Slider({
  className,
  defaultValue,
  value,
  min = 0,
  max = 100,
  "aria-describedby": ariaDescribedBy,
  "aria-valuetext": ariaValueText,
  ...props
}: ComponentProps<typeof SliderPrimitive.Root>) {
  const _value = useMemo(
    () => value ?? defaultValue ?? [min],
    [value, defaultValue, min],
  );

  return (
    <SliderPrimitive.Root
      data-slot="slider"
      defaultValue={defaultValue}
      value={value}
      min={min}
      max={max}
      className={cn(
        "relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50",
        className,
      )}
      {...props}
    >
      <SliderPrimitive.Track className="bg-input border border-border relative h-4 w-full grow overflow-hidden rounded-full">
        <SliderPrimitive.Range className="bg-primary absolute h-full" />
      </SliderPrimitive.Track>
      {Array.from({ length: _value.length }, (_, index) => (
        <Pressable key={index} asChild spring="sm">
          <Surface asChild variant="primary" elevation="sm" rounded="full">
            <SliderPrimitive.Thumb
              aria-describedby={ariaDescribedBy}
              aria-valuetext={ariaValueText}
              className="flex items-center justify-center size-8 ring-0 disabled:pointer-events-none"
            >
              <GripVerticalIcon strokeWidth={3} className="size-3.5 shrink-0" />
            </SliderPrimitive.Thumb>
          </Surface>
        </Pressable>
      ))}
    </SliderPrimitive.Root>
  );
}

export { Slider };
