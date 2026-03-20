import type { NodeParamFields, NodeSchema, NodeParamFieldInfo } from "@bnto/core";
import { inferFieldType } from "@bnto/core";
import type { GroupField } from "./FieldGroup";

/** A single ungrouped field. */
export interface SingleEntry {
  kind: "single";
  paramName: string;
  meta: GroupField["meta"];
  fieldConfig: GroupField["fieldConfig"];
  fieldInfo: GroupField["fieldInfo"];
}

/** A set of consecutive fields sharing the same group. */
export interface GroupEntry {
  kind: "group";
  groupName: string;
  fields: GroupField[];
}

export type FormEntry = SingleEntry | GroupEntry;

/**
 * Build form entries from schema + visible params.
 *
 * Consecutive fields sharing the same `group` (from fieldConfig) are collected
 * into a single GroupEntry. Ungrouped fields become SingleEntries. Fields not
 * present in `schema.params` are skipped.
 */
export function buildFormEntries(
  schema: NodeSchema,
  visibleParams: string[],
  fields?: NodeParamFields,
): FormEntry[] {
  const result: FormEntry[] = [];
  let currentGroup: GroupEntry | null = null;

  for (const paramName of visibleParams) {
    const meta = schema.params[paramName];
    if (!meta) continue;
    const fieldConfig = fields?.[paramName];
    const zodField = schema.schema.shape[paramName];
    const fieldInfo: NodeParamFieldInfo = zodField
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
}
