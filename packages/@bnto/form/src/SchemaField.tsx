"use client";

import { useCallback } from "react";
import type { NodeParamField, NodeParamFieldInfo, NodeParam } from "@bnto/core";
import { Label } from "@bnto/ui";
import { CONTROL_REGISTRY } from "./controls";
import { getFieldLayout } from "./fieldLayout";

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
  const handleChange = useCallback(
    (newValue: unknown) => onChange(name, newValue),
    [name, onChange],
  );

  const id = `param-${name}`;
  const Control = CONTROL_REGISTRY[fieldInfo.control];
  const layout = getFieldLayout(fieldInfo.control);

  const label = (
    <Label htmlFor={id} title={meta.description}>
      {fieldConfig?.label ?? meta.label}
      {fieldInfo.required && <span className="ml-0.5 text-destructive">*</span>}
    </Label>
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

  if (layout === "self-labeled") {
    return <div data-testid={`schema-field-${name}`}>{control}</div>;
  }

  if (layout === "inline") {
    return (
      <div className="flex items-center justify-between gap-2" data-testid={`schema-field-${name}`}>
        {label}
        {control}
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-1.5" data-testid={`schema-field-${name}`}>
      {label}
      {control}
    </div>
  );
}

export { SchemaField };
export type { SchemaFieldProps };
