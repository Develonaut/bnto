"use client";

import * as SliderPrimitive from "@radix-ui/react-slider";

import { GripVerticalIcon } from "../icons";
import { Button } from "./Button";

export function SliderThumb({
  ariaDescribedBy,
  ariaValueText,
}: {
  ariaDescribedBy?: string;
  ariaValueText?: string;
}) {
  return (
    <Button asChild variant="primary" elevation="sm" spring="bouncy" className="rounded-full">
      <SliderPrimitive.Thumb
        aria-describedby={ariaDescribedBy}
        aria-valuetext={ariaValueText}
        className="flex items-center justify-center size-8 ring-0 disabled:pointer-events-none"
      >
        <GripVerticalIcon strokeWidth={3} className="size-3.5 shrink-0" />
      </SliderPrimitive.Thumb>
    </Button>
  );
}
