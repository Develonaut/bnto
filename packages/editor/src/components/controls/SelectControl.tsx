"use client";

import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "@bnto/ui";
import type { ControlProps } from "./types";

function SelectControl({ id, fieldInfo, meta, value, onChange }: ControlProps) {
  const enumValues = fieldInfo.enumValues ?? [];
  return (
    <Select value={String(value ?? "")} onValueChange={onChange}>
      <SelectTrigger size="sm" id={id} data-testid={`control-select-${id}`}>
        <SelectValue placeholder={meta.placeholder ?? "Select\u2026"} />
      </SelectTrigger>
      <SelectContent>
        {enumValues.map((enumVal) => (
          <SelectItem key={enumVal} value={enumVal}>
            {enumVal}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

export { SelectControl };
