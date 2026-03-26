"use client";

import { useMemo } from "react";
import { Slider } from "@bnto/ui";
import type { CompressImagesConfig as Config } from "./types";
import { useConfigChange } from "./useConfigChange";

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
  const change = useConfigChange(value, onChange);
  const qualityValue = useMemo(() => [value.quality], [value.quality]);

  return (
    <Slider
      label="Compression"
      aria-describedby="compress-help"
      value={qualityValue}
      onValueChange={([q]: number[]) => change("quality", q ?? value.quality)}
      min={1}
      max={100}
      presets={COMPRESSION_PRESETS}
    />
  );
}
