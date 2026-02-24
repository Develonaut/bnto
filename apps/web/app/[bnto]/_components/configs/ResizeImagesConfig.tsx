"use client";

import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import type { ResizeImagesConfig as Config } from "./types";

interface ResizeImagesConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function ResizeImagesConfig({
  value,
  onChange,
}: ResizeImagesConfigProps) {
  return (
    <div className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="resize-width">Width (px)</Label>
        <Input
          id="resize-width"
          type="number"
          min={1}
          max={10000}
          value={value.width}
          onChange={(e) => {
            const width = parseInt(e.target.value, 10);
            if (!isNaN(width) && width > 0) {
              onChange({ ...value, width });
            }
          }}
        />
      </div>

      <div className="flex items-center justify-between">
        <Label htmlFor="resize-aspect-ratio">Maintain aspect ratio</Label>
        <Switch
          id="resize-aspect-ratio"
          checked={value.maintainAspectRatio}
          onCheckedChange={(maintainAspectRatio) =>
            onChange({ ...value, maintainAspectRatio: !!maintainAspectRatio })
          }
        />
      </div>
    </div>
  );
}
