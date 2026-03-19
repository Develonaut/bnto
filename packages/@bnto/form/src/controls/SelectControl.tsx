"use client";

import { useMemo } from "react";
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "@bnto/ui";
import type { ControlProps } from "./types";

function SelectControl({ id, fieldInfo, meta, value, onChange }: ControlProps) {
  const enumValues = fieldInfo.enumValues ?? [];

  // Build a value→label map from meta.options when display labels are provided
  const labelMap = useMemo(() => {
    if (!meta.options) return null;
    return new Map(meta.options.map((o) => [o.value, o.label]));
  }, [meta.options]);

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
