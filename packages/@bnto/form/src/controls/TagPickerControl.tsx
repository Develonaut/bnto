"use client";

import { useMemo } from "react";
import { Combobox } from "@bnto/ui";
import type { ControlProps } from "./types";

function TagPickerControl({ id, meta, fieldConfig, value, onChange }: ControlProps) {
  const values = Array.isArray(value) ? (value as string[]) : [];
  const configOptions = fieldConfig?.options;

  // Build options from fieldConfig.options if provided, otherwise use current values
  const options = useMemo(() => {
    if (configOptions) {
      return configOptions;
    }
    // When no predefined options, show current values as selectable
    // Dep uses `value` (prop) not `values` (derived array with new ref each render)
    return values.map((v) => ({ value: v, label: v }));
  }, [configOptions, value]);

  return (
    <Combobox
      id={id}
      options={options}
      value={values}
      onChange={onChange}
      placeholder={meta.placeholder ?? "Select\u2026"}
      data-testid={`control-tagpicker-${id}`}
    />
  );
}

export { TagPickerControl };
