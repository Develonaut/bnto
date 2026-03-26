"use client";

import type { GenerateThumbnailsConfig as Config } from "./types";
import { useGenerateThumbnailsHandlers } from "./useGenerateThumbnailsHandlers";
import { FormatSelect } from "./FormatSelect";
import { WidthInput } from "./WidthInput";
import { PrefixInput } from "./PrefixInput";

interface GenerateThumbnailsConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function GenerateThumbnailsConfig({ value, onChange }: GenerateThumbnailsConfigProps) {
  const { handleWidthChange, handleFormatChange, handlePrefixChange } =
    useGenerateThumbnailsHandlers(value, onChange);

  return (
    <div className="flex w-full items-end gap-4">
      <WidthInput id="thumb-width" value={value.width} onChange={handleWidthChange} />
      <FormatSelect id="thumb-format" value={value.format} onChange={handleFormatChange} />
      <PrefixInput
        id="thumb-prefix"
        value={value.prefix}
        onChange={handlePrefixChange}
        placeholder="thumb_"
      />
    </div>
  );
}
