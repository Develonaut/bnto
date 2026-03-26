"use client";

import { SchemaField } from "./SchemaField";
import type { GroupField } from "./FieldGroup";

interface FieldGroupListProps {
  fields: GroupField[];
  values: Record<string, unknown>;
  onChange: (name: string, value: unknown) => void;
}

/** Renders a list of SchemaFields from GroupField entries. Shared by FieldGroup sections. */
function FieldGroupList({ fields, values, onChange }: FieldGroupListProps) {
  return (
    <>
      {fields.map((field) => (
        <SchemaField
          key={field.paramName}
          name={field.paramName}
          meta={field.meta}
          fieldConfig={field.fieldConfig}
          fieldInfo={field.fieldInfo}
          value={values[field.paramName]}
          onChange={onChange}
        />
      ))}
    </>
  );
}

export { FieldGroupList };
