"use client";

import { useCallback, useMemo } from "react";
import { Slider } from "@bnto/ui";
import type { CompressImagesConfig as Config } from "./types";

const COMPRESSION_PRESETS = [
  { value: 60, label: "Draft" },
  { value: 80, label: "Balanced" },
  { value: 100, label: "Maximum" },
];

interface CompressImagesConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function CompressImagesConfig({ value, onChange }: CompressImagesConfigProps) {
  const qualityValue = useMemo(() => [value.quality], [value.quality]);

  const handleQualityChange = useCallback(
    ([quality]: number[]) => onChange({ ...value, quality: quality ?? value.quality }),
    [onChange, value],
  );

  return (
    <Slider
      label="Compression"
      aria-describedby="compress-help"
      value={qualityValue}
      onValueChange={handleQualityChange}
      min={1}
      max={100}
      presets={COMPRESSION_PRESETS}
    />
  );
}
