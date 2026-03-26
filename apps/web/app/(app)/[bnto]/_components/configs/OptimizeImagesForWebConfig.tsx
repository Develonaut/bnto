"use client";

import { Slider } from "@bnto/ui";
import type { OptimizeImagesForWebConfig as Config } from "./types";
import { useOptimizeImagesHandlers } from "./useOptimizeImagesHandlers";
import { FormatSelect } from "./FormatSelect";
import { WidthInput } from "./WidthInput";

const COMPRESSION_PRESETS = [
  { value: 60, label: "Draft" },
  { value: 80, label: "Balanced" },
  { value: 100, label: "Maximum" },
];

interface OptimizeImagesForWebConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function OptimizeImagesForWebConfig({ value, onChange }: OptimizeImagesForWebConfigProps) {
  const { handleWidthChange, handleFormatChange, qualityValue, handleQualityChange } =
    useOptimizeImagesHandlers(value, onChange);

  return (
    <div className="flex w-full flex-col gap-3">
      <div className="flex w-full items-end gap-4">
        <WidthInput id="optimize-width" value={value.width} onChange={handleWidthChange} />
        <FormatSelect id="optimize-format" value={value.format} onChange={handleFormatChange} />
      </div>
      <Slider
        label="Compression"
        value={qualityValue}
        onValueChange={handleQualityChange}
        min={1}
        max={100}
        presets={COMPRESSION_PRESETS}
      />
    </div>
  );
}
