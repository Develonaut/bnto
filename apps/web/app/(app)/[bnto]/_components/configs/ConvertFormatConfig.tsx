"use client";

import type { ConvertFormatConfig as Config } from "./types";
import { useConvertFormatHandlers } from "./useConvertFormatHandlers";
import { FormatSelect } from "./FormatSelect";
import { QualitySlider } from "./QualitySlider";

interface ConvertFormatConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function ConvertFormatConfig({ value, onChange }: ConvertFormatConfigProps) {
  const { handleFormatChange, qualityValue, handleQualityChange } = useConvertFormatHandlers(
    value,
    onChange,
  );

  return (
    <div className="flex w-full items-end gap-4">
      <FormatSelect
        id="convert-format"
        value={value.format}
        onChange={handleFormatChange}
        helpText="Output type"
      />
      <QualitySlider
        value={value.quality}
        sliderValue={qualityValue}
        onChange={handleQualityChange}
      />
    </div>
  );
}
