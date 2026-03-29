"use client";

import { useMemo } from "react";
import type { NodeParamFields, NodeSchema } from "@bnto/core";
import { Stack, Text } from "@bnto/ui";
import { buildFormEntries } from "./buildFormEntries";
import { FormEntryRenderer } from "./FormEntryRenderer";
import { FormStoreProvider } from "./FormStoreContext";

/** Field-to-field gap: "md" (16px) gives fields breathing room now that descriptions are tooltips. */
const FIELD_GAP = "md" as const;

/**
 * SchemaForm — auto-generates a form from a NodeSchema.
 *
 * Takes a schema definition, current parameter values, and a list of visible
 * parameter names. Renders the correct UI control for each visible parameter
 * using the Zod type -> control mapping from `inferFieldType`.
 *
 * Consecutive fields with the same `group` are collected and rendered together
 * via FieldGroup (e.g., "dimensions" -> aspect lock toggle + side-by-side W/H).
 */

interface SchemaFormProps {
  /** Full schema definition (Zod schema + engine metadata). */
  schema: NodeSchema;
  /** UI presentation metadata per field. */
  fields?: NodeParamFields;
  /** Current parameter values. */
  values: Record<string, unknown>;
  /** Parameter names currently visible (after visibleWhen filtering). */
  visibleParams: string[];
  /** Called when any parameter value changes. */
  onChange: (name: string, value: unknown) => void;
}

function SchemaForm({ schema, fields, values, visibleParams, onChange }: SchemaFormProps) {
  const entries = useMemo(
    () => buildFormEntries(schema, visibleParams, fields),
    [schema, fields, visibleParams],
  );

  if (entries.length === 0) {
    return (
      <Text size="xs" color="muted">
        No configurable parameters.
      </Text>
    );
  }

  return (
    <FormStoreProvider values={values} onChange={onChange}>
      <Stack gap={FIELD_GAP}>
        {entries.map((entry) => (
          <FormEntryRenderer
            key={entry.kind === "group" ? entry.groupName : entry.paramName}
            entry={entry}
            values={values}
            onChange={onChange}
          />
        ))}
      </Stack>
    </FormStoreProvider>
  );
}

export { SchemaForm };
export type { SchemaFormProps };
