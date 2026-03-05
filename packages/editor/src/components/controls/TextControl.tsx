"use client";

import { Input } from "@bnto/ui";
import type { ControlProps } from "./types";

function TextControl({ id, meta, value, onChange }: ControlProps) {
  return (
    <Input
      id={id}
      type="text"
      value={String(value ?? "")}
      placeholder={meta.placeholder}
      onChange={(e) => onChange(e.target.value)}
      className="h-8 text-sm"
      data-testid={`control-text-${id}`}
    />
  );
}

export { TextControl };
