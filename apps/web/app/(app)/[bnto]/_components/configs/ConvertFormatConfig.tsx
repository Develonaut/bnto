"use client";

import { useCallback, useMemo } from "react";
import {
  FormControl,
  FormHelperText,
  FormLabel,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
} from "@bnto/ui";
import type { ConvertFormatConfig as Config } from "./types";
import { FORMAT_OPTIONS } from "./formatOptions";
import { useConfigChange } from "./useConfigChange";

interface ConvertFormatConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

interface QualitySliderProps {
  value: Config;
  qualityValue: number[];
  onQualityChange: (v: number[]) => void;
}

function FormatSelect({
  value,
  onFormatChange,
}: {
  value: Config;
  onFormatChange: (f: string) => void;
}) {
  return (
    <FormControl className="shrink-0">
      <FormLabel id="convert-format-label">Format</FormLabel>
      <Select value={value.format} onValueChange={onFormatChange}>
        <SelectTrigger
          className="w-24"
          aria-labelledby="convert-format-label"
          aria-describedby="convert-format-help"
          data-testid="format-select"
        >
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {FORMAT_OPTIONS.map((opt) => (
            <SelectItem
              key={opt.value}
              value={opt.value}
              data-testid={`format-option-${opt.value}`}
            >
              {opt.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
      <FormHelperText id="convert-format-help">Output type</FormHelperText>
    </FormControl>
  );
}

function QualitySlider({ value, qualityValue, onQualityChange }: QualitySliderProps) {
  return (
    <div className="flex min-w-0 flex-1 flex-col gap-1">
      <Label id="convert-quality-label" className="text-muted-foreground text-xs">
        Quality
      </Label>
      <div className="flex items-center gap-3">
        <Slider
          className="w-full"
          aria-labelledby="convert-quality-label"
          aria-describedby="convert-quality-help"
          aria-valuetext={`${value.quality} percent`}
          value={qualityValue}
          onValueChange={onQualityChange}
          min={1}
          max={100}
          step={1}
        />
        <span className="text-muted-foreground shrink-0 font-mono text-sm tabular-nums">
          {value.quality}%
        </span>
      </div>
      <p id="convert-quality-help" className="text-muted-foreground text-xs">
        Lower values reduce file size
      </p>
    </div>
  );
}

export function ConvertFormatConfig({ value, onChange }: ConvertFormatConfigProps) {
  const change = useConfigChange(value, onChange);
  const qualityValue = useMemo(() => [value.quality], [value.quality]);
  const handleFormatChange = useCallback(
    (f: string) => change("format", f as Config["format"]),
    [change],
  );
  const handleQualityChange = useCallback(
    ([q]: number[]) => change("quality", q ?? value.quality),
    [change, value.quality],
  );

  return (
    <div className="flex w-full items-end gap-4">
      <FormatSelect value={value} onFormatChange={handleFormatChange} />
      <QualitySlider
        value={value}
        qualityValue={qualityValue}
        onQualityChange={handleQualityChange}
      />
    </div>
  );
}
