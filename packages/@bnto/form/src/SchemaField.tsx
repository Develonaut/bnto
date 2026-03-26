"use client";

import { useCallback } from "react";
import type { NodeParamField, NodeParamFieldInfo, NodeParam } from "@bnto/core";
import { CONTROL_REGISTRY } from "./controls";
import { FieldLabel } from "./controls/FieldLabel";
import { SchemaFieldLayout } from "./SchemaFieldLayout";

/**
 * SchemaField — renders the correct UI control via registry lookup.
 *
 * Layout is determined by control type:
 * - **Row** (switch, select): label left, control right — compact inline layout
 * - **Column** (text, number, slider): label on top, control below — full-width
 *
 * Descriptions are shown as native tooltips on the label (title attribute)
 * instead of block text below the control, keeping the panel dense and scannable.
 */

interface SchemaFieldProps {
  name: string;
  meta: NodeParam;
  fieldConfig?: NodeParamField;
  fieldInfo: NodeParamFieldInfo;
  value: unknown;
  onChange: (name: string, value: unknown) => void;
}

function SchemaField({ name, meta, fieldConfig, fieldInfo, value, onChange }: SchemaFieldProps) {
  const handleChange = useCallback((v: unknown) => onChange(name, v), [name, onChange]);
  const id = `param-${name}`;
  const Control = CONTROL_REGISTRY[fieldInfo.control];
  const label = (
    <FieldLabel htmlFor={id} title={meta.description} required={fieldInfo.required}>
      {fieldConfig?.label ?? meta.label}
    </FieldLabel>
  );
  const control = (
    <Control
      id={id}
      fieldInfo={fieldInfo}
      meta={meta}
      fieldConfig={fieldConfig}
      value={value}
      onChange={handleChange}
    />
  );

  return (
    <SchemaFieldLayout name={name} control={fieldInfo.control} label={label} controlEl={control} />
  );
}

export { SchemaField };
export type { SchemaFieldProps };
