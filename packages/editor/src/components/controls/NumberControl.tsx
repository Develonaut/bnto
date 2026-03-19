"use client";

import { useCallback } from "react";
import { Input } from "@bnto/ui";
import type { ControlProps } from "./types";

function NumberControl({ id, fieldInfo, meta, fieldConfig, value, onChange }: ControlProps) {
  const suffix = fieldConfig?.suffix;

  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const num = e.target.value === "" ? undefined : Number(e.target.value);
      onChange(num);
    },
    [onChange],
  );

  return (
    <Input
      id={id}
      type="number"
      value={String(value ?? "")}
      min={fieldInfo.min}
      max={fieldInfo.max}
      placeholder={meta.placeholder}
      onChange={handleChange}
      className="h-8 text-sm"
      suffix={suffix}
      data-testid={`control-number-${id}`}
    />
  );
}

export { NumberControl };
