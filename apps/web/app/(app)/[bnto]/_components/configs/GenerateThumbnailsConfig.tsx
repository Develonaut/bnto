"use client";

import type { ChangeEvent } from "react";
import { useCallback } from "react";
import { FormControl, FormLabel, Input } from "@bnto/ui";
import type { GenerateThumbnailsConfig as Config } from "./types";
import { FormatSelect } from "./FormatSelect";
import { useConfigChange } from "./useConfigChange";

interface GenerateThumbnailsConfigProps {
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

function PrefixInput({ value, change }: { value: string; change: ChangeField }) {
  const handlePrefixChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => change("prefix", e.target.value),
    [change],
  );

  return (
    <FormControl className="shrink-0">
      <FormLabel>Prefix</FormLabel>
      <Input
        type="text"
        wrapperClassName="w-28"
        value={value}
        onChange={handlePrefixChange}
        placeholder="thumb_"
      />
    </FormControl>
  );
}

export function GenerateThumbnailsConfig({ value, onChange }: GenerateThumbnailsConfigProps) {
  const change = useConfigChange(value, onChange);
  const handleWidthChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => {
      const width = parseInt(e.target.value, 10);
      if (!isNaN(width) && width > 0) change("width", width);
    },
    [change],
  );
  const handleFormatChange = useCallback(
    (f: string) => change("format", f as Config["format"]),
    [change],
  );

  return (
    <div className="flex w-full items-end gap-4">
      <WidthInput value={value.width} onChange={handleWidthChange} />
      <FormatSelect
        format={value.format}
        labelId="thumb-format-label"
        onFormatChange={handleFormatChange}
      />
      <PrefixInput value={value.prefix} change={change} />
    </div>
  );
}
