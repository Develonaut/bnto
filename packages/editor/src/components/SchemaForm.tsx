"use client";

import { useMemo } from "react";
import type { NodeParamFields, NodeSchema } from "@bnto/core";
import { inferFieldType } from "@bnto/core";
import { Stack, Text } from "@bnto/ui";
import { SchemaField } from "./SchemaField";
import { FieldGroup } from "./FieldGroup";
import type { GroupField } from "./FieldGroup";

/** Field-to-field gap: "md" (16px) gives fields breathing room now that descriptions are tooltips. */
const FIELD_GAP = "md" as const;

/**
 * SchemaForm — auto-generates a form from a NodeSchema.
 *
 * Takes a schema definition, current parameter values, and a list of visible
 * parameter names. Renders the correct UI control for each visible parameter
 * using the Zod type → control mapping from `inferFieldType`.
 *
 * Consecutive fields with the same `group` are collected and rendered together
 * via FieldGroup (e.g., "dimensions" → aspect lock toggle + side-by-side W/H).
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

/** A single ungrouped field. */
interface SingleEntry {
  kind: "single";
  paramName: string;
  meta: GroupField["meta"];
  fieldConfig: GroupField["fieldConfig"];
  fieldInfo: GroupField["fieldInfo"];
}

/** A set of consecutive fields sharing the same group. */
interface GroupEntry {
  kind: "group";
  groupName: string;
  fields: GroupField[];
}

type FormEntry = SingleEntry | GroupEntry;

function SchemaForm({ schema, fields, values, visibleParams, onChange }: SchemaFormProps) {
  const entries = useMemo(() => {
    const result: FormEntry[] = [];
    let currentGroup: GroupEntry | null = null;

    for (const paramName of visibleParams) {
      const meta = schema.params[paramName];
      if (!meta) continue;
      const fieldConfig = fields?.[paramName];
      const zodField = schema.schema.shape[paramName];
      const fieldInfo = zodField
        ? inferFieldType(zodField, fieldConfig)
        : { type: "string" as const, control: "text" as const, required: true };
      const group = fieldConfig?.group;

      if (group && currentGroup?.groupName === group) {
        currentGroup.fields.push({ paramName, meta, fieldConfig, fieldInfo });
      } else if (group) {
        currentGroup = {
          kind: "group",
          groupName: group,
          fields: [{ paramName, meta, fieldConfig, fieldInfo }],
        };
        result.push(currentGroup);
      } else {
        currentGroup = null;
        result.push({ kind: "single", paramName, meta, fieldConfig, fieldInfo });
      }
    }

    return result;
  }, [schema, fields, visibleParams]);

  if (entries.length === 0) {
    return (
      <Text size="xs" color="muted">
        No configurable parameters.
      </Text>
    );
  }

  return (
    <Stack gap={FIELD_GAP}>
      {entries.map((entry) => {
        if (entry.kind === "group") {
          return (
            <FieldGroup
              key={entry.groupName}
              fields={entry.fields}
              values={values}
              onChange={onChange}
            />
          );
        }
        return (
          <SchemaField
            key={entry.paramName}
            name={entry.paramName}
            meta={entry.meta}
            fieldConfig={entry.fieldConfig}
            fieldInfo={entry.fieldInfo}
            value={values[entry.paramName]}
            onChange={onChange}
          />
        );
      })}
    </Stack>
  );
}

export { SchemaForm };
export type { SchemaFormProps };
