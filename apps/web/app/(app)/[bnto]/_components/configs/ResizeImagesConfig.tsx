"use client";

import { Input } from "@/components/ui/Input";
import { Label } from "@/components/ui/Label";
import { Switch } from "@/components/ui/Switch";
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
    <div className="flex w-full items-end gap-4">
      <div className="flex shrink-0 flex-col gap-1">
        <Label htmlFor="resize-width" className="text-muted-foreground text-xs">Width (px)</Label>
        <Input
          id="resize-width"
          type="number"
          min={1}
          max={10000}
          aria-describedby="resize-width-help"
          value={value.width}
          wrapperClassName="w-24"
          onChange={(e) => {
            const width = parseInt(e.target.value, 10);
            if (!isNaN(width) && width > 0) {
              onChange({ ...value, width });
            }
          }}
        />
        <p id="resize-width-help" className="text-muted-foreground text-xs">Target width in pixels</p>
      </div>
      <div className="flex shrink-0 flex-col gap-1">
        <Label htmlFor="resize-aspect-ratio" className="text-muted-foreground text-xs">Aspect ratio</Label>
        <Switch
          id="resize-aspect-ratio"
          aria-describedby="resize-aspect-ratio-help"
          checked={value.maintainAspectRatio}
          onCheckedChange={(maintainAspectRatio) =>
            onChange({ ...value, maintainAspectRatio: !!maintainAspectRatio })
          }
        />
        <p id="resize-aspect-ratio-help" className="text-muted-foreground text-xs">Keep proportions</p>
      </div>
    </div>
  );
}
