"use client";

import { KeyValueEditor } from "@bnto/ui";
import type { ControlProps } from "./types";

function KeyValueEditorControl({ id, meta, value, onChange }: ControlProps) {
  const record =
    value && typeof value === "object" && !Array.isArray(value)
      ? (value as Record<string, string>)
      : {};

  return (
    <KeyValueEditor
      id={id}
      value={record}
      onChange={(next) => onChange(next)}
      keyPlaceholder={meta.placeholder ?? "Key"}
      valuePlaceholder="Value"
      data-testid={`control-keyvalue-${id}`}
    />
  );
}

export { KeyValueEditorControl };
