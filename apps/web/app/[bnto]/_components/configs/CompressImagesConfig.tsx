"use client";

import { Label } from "@/components/ui/label";
import { Slider } from "@/components/ui/slider";
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
    <div className="space-y-4">
      <div className="space-y-2">
        <div className="flex items-center justify-between">
          <Label>Quality</Label>
          <span className="text-muted-foreground font-mono text-sm">
            {value.quality}%
          </span>
        </div>
        <Slider
          value={[value.quality]}
          onValueChange={([quality]: number[]) =>
            onChange({ ...value, quality: quality ?? value.quality })
          }
          min={1}
          max={100}
          step={1}
        />
        <p className="text-muted-foreground text-xs">
          Lower quality = smaller file size. 70-90 is recommended.
        </p>
      </div>
    </div>
  );
}
