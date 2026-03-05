"use client";

import { useMemo } from "react";
import type { NodeSchemaDefinition } from "@bnto/nodes";
import { inferFieldType } from "@bnto/nodes";
import { Stack, Text } from "@bnto/ui";
import { SchemaField } from "./SchemaField";

/**
 * SchemaForm — auto-generates a form from a NodeSchemaDefinition.
 *
 * Takes a schema definition, current parameter values, and a list of visible
 * parameter names. Renders the correct UI control for each visible parameter
 * using the Zod type → control mapping from `inferFieldType`.
 *
 * This replaces the manual iteration + ParameterField switch pattern that
 * was previously in ConfigPanelRoot. The form is fully schema-driven:
 * add a new node type → add a schema → form renders automatically.
 */

interface SchemaFormProps {
  /** Full schema definition (Zod schema + UI metadata). */
  schema: NodeSchemaDefinition;
  /** Current parameter values. */
  values: Record<string, unknown>;
  /** Parameter names currently visible (after visibleWhen filtering). */
  visibleParams: string[];
  /** Called when any parameter value changes. */
  onChange: (name: string, value: unknown) => void;
}

function SchemaForm({ schema, values, visibleParams, onChange }: SchemaFormProps) {
  const fields = useMemo(
    () =>
      visibleParams
        .map((paramName) => {
          const meta = schema.params[paramName];
          if (!meta) return null;
          const zodField = schema.schema.shape[paramName];
          const fieldInfo = zodField
            ? inferFieldType(zodField)
            : { type: "string" as const, control: "text" as const, required: true };
          return { paramName, meta, fieldInfo };
        })
        .filter(Boolean),
    [schema, visibleParams],
  );

  if (fields.length === 0) {
    return (
      <Text size="xs" color="muted">
        No configurable parameters.
      </Text>
    );
  }

  return (
    <Stack gap="sm">
      {fields.map((field) => {
        if (!field) return null;
        return (
          <SchemaField
            key={field.paramName}
            name={field.paramName}
            meta={field.meta}
            fieldInfo={field.fieldInfo}
            value={values[field.paramName]}
            onChange={onChange}
          />
        );
      })}
    </Stack>
  );
}

export { SchemaForm };
export type { SchemaFormProps };
