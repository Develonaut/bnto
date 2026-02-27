"use client";

import { Label } from "@/components/ui/Label";
import { Select } from "@/components/ui/Select";
import { Slider } from "@/components/ui/Slider";
import type { ConvertFormatConfig as Config } from "./types";

const FORMAT_OPTIONS = [
  { value: "webp", label: "WebP" },
  { value: "jpeg", label: "JPEG" },
  { value: "png", label: "PNG" },
] as const;

interface ConvertFormatConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function ConvertFormatConfig({
  value,
  onChange,
}: ConvertFormatConfigProps) {
  return (
    <div className="space-y-4">
      <div className="space-y-2">
        <Label>Output format</Label>
        <Select
          value={value.format}
          onValueChange={(format) =>
            onChange({ ...value, format: format as Config["format"] })
          }
        >
          <Select.Trigger>
            <Select.Value />
          </Select.Trigger>
          <Select.Content>
            {FORMAT_OPTIONS.map((opt) => (
              <Select.Item key={opt.value} value={opt.value}>
                {opt.label}
              </Select.Item>
            ))}
          </Select.Content>
        </Select>
      </div>

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
