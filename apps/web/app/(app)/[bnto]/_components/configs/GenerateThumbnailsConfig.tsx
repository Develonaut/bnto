"use client";

import type { ChangeEvent } from "react";
import { useCallback } from "react";
import {
  FormControl,
  FormLabel,
  Input,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@bnto/ui";
import type { GenerateThumbnailsConfig as Config } from "./types";
import { FORMAT_OPTIONS } from "./formatOptions";
import { useConfigChange } from "./useConfigChange";

interface GenerateThumbnailsConfigProps {
  value: Config;
  onChange: (config: Config) => void;
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

  return (
    <div className="flex w-full items-end gap-4">
      <FormControl className="shrink-0">
        <FormLabel>Width (px)</FormLabel>
        <Input
          type="number"
          min={1}
          max={10000}
          value={value.width}
          wrapperClassName="w-24"
          onChange={handleWidthChange}
        />
      </FormControl>
      <FormControl className="shrink-0">
        <FormLabel id="thumb-format-label">Format</FormLabel>
        <Select value={value.format} onValueChange={(f) => change("format", f as Config["format"])}>
          <SelectTrigger
            className="w-24"
            aria-labelledby="thumb-format-label"
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
      <FormControl className="shrink-0">
        <FormLabel>Prefix</FormLabel>
        <Input
          type="text"
          wrapperClassName="w-28"
          value={value.prefix}
          onChange={(e) => change("prefix", e.target.value)}
          placeholder="thumb_"
        />
      </FormControl>
    </div>
  );
}
