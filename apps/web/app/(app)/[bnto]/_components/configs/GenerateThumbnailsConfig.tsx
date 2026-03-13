"use client";

import {
  Input,
  Label,
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@bnto/ui";
import type { GenerateThumbnailsConfig as Config } from "./types";

const FORMAT_OPTIONS = [
  { value: "webp", label: "WebP" },
  { value: "jpeg", label: "JPEG" },
  { value: "png", label: "PNG" },
] as const;

interface GenerateThumbnailsConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function GenerateThumbnailsConfig({
  value,
  onChange,
}: GenerateThumbnailsConfigProps) {
  return (
    <div className="flex w-full items-end gap-4">
      <div className="flex shrink-0 flex-col gap-1">
        <Label htmlFor="thumb-width" className="text-muted-foreground text-xs">
          Width (px)
        </Label>
        <Input
          id="thumb-width"
          type="number"
          min={1}
          max={10000}
          value={value.width}
          wrapperClassName="w-24"
          onChange={(e) => {
            const width = parseInt(e.target.value, 10);
            if (!isNaN(width) && width > 0) {
              onChange({ ...value, width });
            }
          }}
        />
      </div>
      <div className="flex shrink-0 flex-col gap-1">
        <Label id="thumb-format-label" className="text-muted-foreground text-xs">
          Format
        </Label>
        <Select
          value={value.format}
          onValueChange={(format) => onChange({ ...value, format: format as Config["format"] })}
        >
          <SelectTrigger className="w-24" aria-labelledby="thumb-format-label">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {FORMAT_OPTIONS.map((opt) => (
              <SelectItem key={opt.value} value={opt.value}>
                {opt.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      <div className="flex shrink-0 flex-col gap-1">
        <Label htmlFor="thumb-prefix" className="text-muted-foreground text-xs">
          Prefix
        </Label>
        <Input
          id="thumb-prefix"
          type="text"
          wrapperClassName="w-28"
          value={value.prefix}
          onChange={(e) => onChange({ ...value, prefix: e.target.value })}
          placeholder="thumb_"
        />
      </div>
    </div>
  );
}
