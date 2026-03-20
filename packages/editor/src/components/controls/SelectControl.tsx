"use client";

import { useMemo } from "react";
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "@bnto/ui";
import type { ControlProps } from "./types";

function SelectControl({ id, fieldInfo, meta, fieldConfig, value, onChange }: ControlProps) {
  const enumValues = fieldInfo.enumValues ?? [];

  // Build a value→label map from fieldConfig.options when display labels are provided
  const options = fieldConfig?.options;
  const labelMap = useMemo(() => {
    if (!options) return null;
    return new Map(options.map((o) => [o.value, o.label]));
  }, [options]);

  return (
    <Select value={String(value ?? "")} onValueChange={onChange}>
      <SelectTrigger id={id} data-testid={`control-select-${id}`}>
        <SelectValue placeholder={meta.placeholder ?? "Select\u2026"} />
      </SelectTrigger>
      <SelectContent>
        {enumValues.map((enumVal) => (
          <SelectItem key={enumVal} value={enumVal} data-testid={`select-option-${enumVal}`}>
            {labelMap?.get(enumVal) ?? enumVal}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

export { SelectControl };
