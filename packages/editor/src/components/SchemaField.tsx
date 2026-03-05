"use client";

import { useCallback } from "react";
import type { FieldTypeInfo, NodeParamMeta } from "@bnto/nodes";
import { Label, Text } from "@bnto/ui";
import { CONTROL_REGISTRY } from "./controls";

/**
 * SchemaField — renders the correct UI control via registry lookup.
 *
 * The `control` field from `FieldTypeInfo` keys into `CONTROL_REGISTRY`
 * to get the right component. No if/else chains, no switch statements.
 * Adding a new control type = add a component + add a registry entry.
 */

interface SchemaFieldProps {
  name: string;
  meta: NodeParamMeta;
  fieldInfo: FieldTypeInfo;
  value: unknown;
  required: boolean;
  onChange: (name: string, value: unknown) => void;
}

function SchemaField({ name, meta, fieldInfo, value, required, onChange }: SchemaFieldProps) {
  const handleChange = useCallback(
    (newValue: unknown) => onChange(name, newValue),
    [name, onChange],
  );

  const id = `param-${name}`;
  const Control = CONTROL_REGISTRY[fieldInfo.control];

  return (
    <div className="flex flex-col gap-1.5" data-testid={`schema-field-${name}`}>
      <Label htmlFor={id} className="text-xs font-medium">
        {meta.label}
        {required && <span className="ml-0.5 text-destructive">*</span>}
      </Label>

      <Control id={id} fieldInfo={fieldInfo} meta={meta} value={value} onChange={handleChange} />

      {meta.description && (
        <Text size="xs" color="muted">
          {meta.description}
        </Text>
      )}
    </div>
  );
}

export { SchemaField };
export type { SchemaFieldProps };
