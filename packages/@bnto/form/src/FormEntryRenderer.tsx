"use client";

import type { FormEntry } from "./buildFormEntries";
import { SchemaField } from "./SchemaField";
import { FieldGroup } from "./FieldGroup";

interface FormEntryRendererProps {
  entry: FormEntry;
  values: Record<string, unknown>;
  onChange: (name: string, value: unknown) => void;
}

/** Renders a single form entry — either a grouped set of fields or a standalone field. */
function FormEntryRenderer({ entry, values, onChange }: FormEntryRendererProps) {
  if (entry.kind === "group") {
    return <FieldGroup fields={entry.fields} values={values} onChange={onChange} />;
  }

  return (
    <SchemaField
      name={entry.paramName}
      meta={entry.meta}
      fieldConfig={entry.fieldConfig}
      fieldInfo={entry.fieldInfo}
      value={values[entry.paramName]}
      onChange={onChange}
    />
  );
}

export { FormEntryRenderer };
