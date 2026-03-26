"use client";

import type { ChangeEvent } from "react";
import { useCallback, useMemo } from "react";
import {
  FormControl,
  FormLabel,
  Input,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
} from "@bnto/ui";
import type { OptimizeImagesForWebConfig as Config } from "./types";
import { FORMAT_OPTIONS } from "./formatOptions";
import { useConfigChange } from "./useConfigChange";

const COMPRESSION_PRESETS = [
  { value: 60, label: "Draft" },
  { value: 80, label: "Balanced" },
  { value: 100, label: "Maximum" },
];

interface OptimizeImagesForWebConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

type ChangeField = ReturnType<typeof useConfigChange<Config>>;

function WidthInput({
  value,
  onChange,
}: {
  value: number;
  onChange: (e: ChangeEvent<HTMLInputElement>) => void;
}) {
  return (
    <FormControl className="shrink-0">
      <FormLabel>Width (px)</FormLabel>
      <Input
        type="number"
        min={1}
        max={10000}
        value={value}
        wrapperClassName="w-24"
        onChange={onChange}
      />
    </FormControl>
  );
}

function FormatSelect({ value, change }: { value: Config; change: ChangeField }) {
  return (
    <FormControl className="shrink-0">
      <FormLabel id="optimize-format-label">Format</FormLabel>
      <Select value={value.format} onValueChange={(f) => change("format", f as Config["format"])}>
        <SelectTrigger
          className="w-24"
          aria-labelledby="optimize-format-label"
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
    </FormControl>
  );
}

function CompressionSlider({ value, change }: { value: Config; change: ChangeField }) {
  const qualityValue = useMemo(() => [value.quality], [value.quality]);

  return (
    <Slider
      label="Compression"
      value={qualityValue}
      onValueChange={([q]: number[]) => change("quality", q ?? value.quality)}
      min={1}
      max={100}
      presets={COMPRESSION_PRESETS}
    />
  );
}

export function OptimizeImagesForWebConfig({ value, onChange }: OptimizeImagesForWebConfigProps) {
  const change = useConfigChange(value, onChange);

  const handleWidthChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => {
      const width = parseInt(e.target.value, 10);
      if (!isNaN(width) && width > 0) change("width", width);
    },
    [change],
  );

  return (
    <div className="flex w-full flex-col gap-3">
      <div className="flex w-full items-end gap-4">
        <WidthInput value={value.width} onChange={handleWidthChange} />
        <FormatSelect value={value} change={change} />
      </div>
      <CompressionSlider value={value} change={change} />
    </div>
  );
}
