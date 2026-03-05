"use client";

import { Label, Slider } from "@bnto/ui";
import type { CompressImagesConfig as Config } from "./types";

interface CompressImagesConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function CompressImagesConfig({
  value,
  onChange,
}: CompressImagesConfigProps) {
  return (
    <div className="flex w-full flex-col gap-3">
      <div className="flex items-baseline justify-between gap-2">
        <Label id="compress-quality-label" className="text-muted-foreground text-sm">Quality</Label>
        <span className="text-muted-foreground shrink-0 font-mono text-sm tabular-nums">
          {value.quality}%
        </span>
      </div>
      <Slider
        className="w-full"
        aria-labelledby="compress-quality-label"
        aria-describedby="compress-quality-help"
        aria-valuetext={`${value.quality} percent`}
        value={[value.quality]}
        onValueChange={([quality]: number[]) =>
          onChange({ ...value, quality: quality ?? value.quality })
        }
        min={1}
        max={100}
        step={1}
      />
      <p id="compress-quality-help" className="min-h-4 text-xs text-muted-foreground">Lower values reduce file size more but also lower quality</p>
    </div>
  );
}
