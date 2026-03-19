"use client";

import type { FieldConfig, FieldTypeInfo, NodeParamMeta } from "@bnto/core";
import { SchemaField } from "./SchemaField";

/**
 * FieldGroup — renders related parameters in a compact layout.
 *
 * Switch/select fields render as individual SchemaFields at the top.
 * Number/text fields render side-by-side in a 2-column grid below.
 * This matches design tools like Figma's dimension controls.
 */

/** Controls that render inline (label left, control right). */
const INLINE_CONTROLS = new Set(["switch", "select"]);

interface GroupField {
  paramName: string;
  meta: NodeParamMeta;
  fieldConfig?: FieldConfig;
  fieldInfo: FieldTypeInfo;
}

interface FieldGroupProps {
  fields: GroupField[];
  values: Record<string, unknown>;
  onChange: (name: string, value: unknown) => void;
}

function FieldGroup({ fields, values, onChange }: FieldGroupProps) {
  const inlineFields = fields.filter((f) => INLINE_CONTROLS.has(f.fieldInfo.control));
  const gridFields = fields.filter((f) => !INLINE_CONTROLS.has(f.fieldInfo.control));

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
