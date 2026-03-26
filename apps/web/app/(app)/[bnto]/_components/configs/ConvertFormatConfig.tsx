"use client";

import { useCallback, useMemo } from "react";
import { FormHelperText, Label, Slider } from "@bnto/ui";
import type { ConvertFormatConfig as Config } from "./types";
import { FormatSelect } from "./FormatSelect";
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
      <FormatSelect
        format={value.format}
        labelId="convert-format-label"
        describedBy="convert-format-help"
        onFormatChange={handleFormatChange}
      >
        <FormHelperText id="convert-format-help">Output type</FormHelperText>
      </FormatSelect>
      <QualitySlider
        value={value}
        qualityValue={qualityValue}
        onQualityChange={handleQualityChange}
      />
    </div>
  );
}
