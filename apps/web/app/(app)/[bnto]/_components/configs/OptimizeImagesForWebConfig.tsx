"use client";

import {
  Input,
  Label,
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
  Slider,
} from "@bnto/ui";
import type { OptimizeImagesForWebConfig as Config } from "./types";

const FORMAT_OPTIONS = [
  { value: "webp", label: "WebP" },
  { value: "jpeg", label: "JPEG" },
  { value: "png", label: "PNG" },
] as const;

const COMPRESSION_PRESETS = [
  { value: 20, label: "Light" },
  { value: 50, label: "Balanced" },
  { value: 80, label: "Maximum" },
];

interface OptimizeImagesForWebConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function OptimizeImagesForWebConfig({
  value,
  onChange,
}: OptimizeImagesForWebConfigProps) {
  return (
    <div className="flex w-full flex-col gap-3">
      <div className="flex w-full items-end gap-4">
        <div className="flex shrink-0 flex-col gap-1">
          <Label htmlFor="optimize-width" className="text-muted-foreground text-xs">
            Width (px)
          </Label>
          <Input
            id="optimize-width"
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
          <Label id="optimize-format-label" className="text-muted-foreground text-xs">
            Format
          </Label>
          <Select
            value={value.format}
            onValueChange={(format) => onChange({ ...value, format: format as Config["format"] })}
          >
            <SelectTrigger className="w-24" aria-labelledby="optimize-format-label">
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
      </div>
      <Slider
        label="Compression"
        value={[value.compression]}
        onValueChange={([compression]: number[]) =>
          onChange({ ...value, compression: compression ?? value.compression })
        }
        min={1}
        max={100}
        presets={COMPRESSION_PRESETS}
      />
    </div>
  );
}
