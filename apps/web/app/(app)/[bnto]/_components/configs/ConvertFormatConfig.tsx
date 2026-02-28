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
    <div className="flex w-full items-end gap-4">
      <div className="flex shrink-0 flex-col gap-1">
        <Label className="text-muted-foreground text-xs">Format</Label>
        <Select
          value={value.format}
          onValueChange={(format) =>
            onChange({ ...value, format: format as Config["format"] })
          }
        >
          <Select.Trigger className="w-24">
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
        <p className="text-muted-foreground text-xs">Output type</p>
      </div>

      <div className="flex min-w-0 flex-1 flex-col gap-1">
        <Label className="text-muted-foreground text-xs">Quality</Label>
        <div className="flex items-center gap-3">
          <Slider
            className="w-full"
            value={[value.quality]}
            onValueChange={([quality]: number[]) =>
              onChange({ ...value, quality: quality ?? value.quality })
            }
            min={1}
            max={100}
            step={1}
          />
          <span className="text-muted-foreground shrink-0 font-mono text-sm tabular-nums">
            {value.quality}%
          </span>
        </div>
        <p className="text-muted-foreground text-xs">Lower values reduce file size</p>
      </div>
    </div>
  );
}
