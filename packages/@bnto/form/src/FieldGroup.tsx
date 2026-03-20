"use client";

import type { NodeParamField, NodeParamFieldInfo, NodeParam } from "@bnto/core";
import { SchemaField } from "./SchemaField";
import { partitionGroupFields } from "./partitionGroupFields";

/**
 * FieldGroup — renders related parameters in a compact layout.
 *
 * Switch/select fields render as individual SchemaFields at the top.
 * Number/text fields render side-by-side in a 2-column grid below.
 * This matches design tools like Figma's dimension controls.
 */

interface GroupField {
  paramName: string;
  meta: NodeParam;
  fieldConfig?: NodeParamField;
  fieldInfo: NodeParamFieldInfo;
}

interface FieldGroupProps {
  fields: GroupField[];
  values: Record<string, unknown>;
  onChange: (name: string, value: unknown) => void;
}

function FieldGroup({ fields, values, onChange }: FieldGroupProps) {
  const { inlineFields, gridFields } = partitionGroupFields(fields);

  return (
    <div className="flex flex-col gap-3">
      {inlineFields.map((field) => (
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
      {gridFields.length > 0 && (
        <div className="grid grid-cols-2 gap-3">
          {gridFields.map((field) => (
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
        </div>
      )}
    </div>
  );
}

export { FieldGroup };
export type { FieldGroupProps, GroupField };
