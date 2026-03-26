"use client";

import type { ChangeEvent } from "react";
import { useCallback } from "react";
import { FormControl, FormHelperText, FormLabel, Input, Switch } from "@bnto/ui";
import type { ResizeImagesConfig as Config } from "./types";
import { useConfigChange } from "./useConfigChange";

interface ResizeImagesConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function ResizeImagesConfig({ value, onChange }: ResizeImagesConfigProps) {
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
        <FormHelperText>Target width in pixels</FormHelperText>
      </FormControl>
      <FormControl className="shrink-0">
        <FormLabel htmlFor="resize-aspect-ratio">Aspect ratio</FormLabel>
        <Switch
          id="resize-aspect-ratio"
          checked={value.maintainAspectRatio}
          onCheckedChange={(v) => change("maintainAspectRatio", !!v)}
        />
        <FormHelperText>Keep proportions</FormHelperText>
      </FormControl>
    </div>
  );
}
