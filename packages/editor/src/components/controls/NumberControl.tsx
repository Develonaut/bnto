"use client";

import { Input } from "@bnto/ui";
import type { ControlProps } from "./types";

function NumberControl({ id, fieldInfo, meta, value, onChange }: ControlProps) {
  return (
    <Input
      id={id}
      type="number"
      value={String(value ?? "")}
      min={fieldInfo.min}
      max={fieldInfo.max}
      placeholder={meta.placeholder}
      onChange={(e) => {
        const num = e.target.value === "" ? undefined : Number(e.target.value);
        onChange(num);
      }}
      className="h-8 text-sm"
      data-testid={`control-number-${id}`}
    />
  );
}

export { NumberControl };
