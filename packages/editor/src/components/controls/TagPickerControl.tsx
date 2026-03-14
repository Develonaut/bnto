"use client";

import { useMemo } from "react";
import { Combobox } from "@bnto/ui";
import type { ControlProps } from "./types";

function TagPickerControl({ id, meta, value, onChange }: ControlProps) {
  const values = Array.isArray(value) ? (value as string[]) : [];

  // Build options from meta.options if provided, otherwise use current values
  const options = useMemo(() => {
    if (meta.options) {
      return meta.options;
    }
    // When no predefined options, show current values as selectable
    return values.map((v) => ({ value: v, label: v }));
  }, [meta.options, values]);

  return (
    <Combobox
      id={id}
      options={options}
      value={values}
      onChange={(next) => onChange(next)}
      placeholder={meta.placeholder ?? "Select\u2026"}
      data-testid={`control-tagpicker-${id}`}
    />
  );
}

export { TagPickerControl };
