"use client";

import { Textarea } from "@bnto/ui";
import type { ControlProps } from "./types";

function TextareaControl({ id, meta, value, onChange }: ControlProps) {
  return (
    <Textarea
      id={id}
      size="sm"
      value={String(value ?? "")}
      placeholder={meta.placeholder}
      onChange={(e) => onChange(e.target.value)}
      data-testid={`control-textarea-${id}`}
    />
  );
}

export { TextareaControl };
