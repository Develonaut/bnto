"use client";

import { Slider } from "@bnto/ui";
import type { ControlProps } from "./types";

function SliderControl({ id, fieldInfo, value, onChange }: ControlProps) {
  const numValue = typeof value === "number" ? value : (fieldInfo.min ?? 0);
  return (
    <div className="flex items-center gap-3">
      <Slider
        id={id}
        min={fieldInfo.min}
        max={fieldInfo.max}
        value={[numValue]}
        onValueChange={([v]) => onChange(v)}
        className="flex-1"
        data-testid={`control-slider-${id}`}
      />
      <span className="min-w-8 text-right text-xs tabular-nums text-muted-foreground">
        {numValue}
      </span>
    </div>
  );
}

export { SliderControl };
