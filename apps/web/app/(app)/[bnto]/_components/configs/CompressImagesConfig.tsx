"use client";

import { useCallback, useMemo } from "react";
import { Slider } from "@bnto/ui";
import type { CompressImagesConfig as Config } from "./types";

const COMPRESSION_PRESETS = [
  { value: 20, label: "Light" },
  { value: 50, label: "Balanced" },
  { value: 80, label: "Maximum" },
];

interface CompressImagesConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function CompressImagesConfig({ value, onChange }: CompressImagesConfigProps) {
  const compressionValue = useMemo(() => [value.compression], [value.compression]);

  const handleCompressionChange = useCallback(
    ([compression]: number[]) =>
      onChange({ ...value, compression: compression ?? value.compression }),
    [onChange, value],
  );

  return (
    <Slider
      label="Compression"
      aria-describedby="compress-help"
      value={compressionValue}
      onValueChange={handleCompressionChange}
      min={1}
      max={100}
      presets={COMPRESSION_PRESETS}
    />
  );
}
