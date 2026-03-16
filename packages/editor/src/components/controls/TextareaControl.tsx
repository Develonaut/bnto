"use client";

import { useCallback } from "react";
import { Textarea } from "@bnto/ui";
import type { ControlProps } from "./types";

function TextareaControl({ id, meta, value, onChange }: ControlProps) {
  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLTextAreaElement>) => onChange(e.target.value),
    [onChange],
  );

  return (
    <Textarea
      id={id}
      size="sm"
      value={String(value ?? "")}
      placeholder={meta.placeholder}
      onChange={handleChange}
      data-testid={`control-textarea-${id}`}
    />
  );
}

export { TextareaControl };
