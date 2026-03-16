"use client";

import { useCallback } from "react";
import { Input } from "@bnto/ui";
import type { ControlProps } from "./types";

function TextControl({ id, meta, value, onChange }: ControlProps) {
  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => onChange(e.target.value),
    [onChange],
  );

  return (
    <Input
      id={id}
      type="text"
      value={String(value ?? "")}
      placeholder={meta.placeholder}
      onChange={handleChange}
      className="h-8 text-sm"
      data-testid={`control-text-${id}`}
    />
  );
}

export { TextControl };
