/**
 * Returns parameter names that are visible for a given condition.
 *
 * Visibility rules live in FieldConfigMap (UI concern). Falls back to
 * looking up NODE_FIELD_CONFIGS automatically via getNodeFields().
 */

import { getNodeSchema } from "./getNodeSchema";
import { getNodeFields } from "./getNodeFields";
import { matchesCondition } from "./matchesCondition";
import type { FieldConfigMap, ParamCondition } from "./types";

/**
 * Returns parameter names that are visible when a specific parameter
 * has a specific value.
 *
 * Parameters without a visibleWhen condition are always visible.
 *
 * Example: `getVisibleParams("input", "mode", "file-upload")`
 * returns names including accept, extensions, label, etc.
 */
export function getVisibleParams(typeName: string, paramName: string, paramValue: string): string[];

/**
 * Returns parameter names that are visible given the full current
 * parameter values. Evaluates each param's visibleWhen against all
 * current values (OR logic for array conditions).
 *
 * Example: `getVisibleParams("input", { mode: "file-upload" })`
 */
export function getVisibleParams(typeName: string, parameters: Record<string, unknown>): string[];

export function getVisibleParams(
  typeName: string,
  paramNameOrValues: string | Record<string, unknown>,
  paramValue?: string,
): string[] {
  const schemaDef = getNodeSchema(typeName);
  if (!schemaDef) return [];

  const fields = getNodeFields(typeName);

  // Single param/value check (original API)
  if (typeof paramNameOrValues === "string") {
    return resolveVisible(schemaDef.params, fields, (cond) =>
      matchesCondition(cond, paramNameOrValues, paramValue!),
    );
  }

  // Full parameters map check
  const parameters = paramNameOrValues;
  return resolveVisible(schemaDef.params, fields, (cond) => {
    const conditions = Array.isArray(cond) ? cond : [cond];
    return conditions.some((c) => String(parameters[c.param] ?? "") === c.equals);
  });
}

/** Shared filter logic — reads visibleWhen from FieldConfig. */
function resolveVisible(
  params: Record<string, { label: string }>,
  fields: FieldConfigMap | undefined,
  matches: (cond: ParamCondition) => boolean,
): string[] {
  return Object.keys(params).filter((name) => {
    const fieldVisibleWhen = fields?.[name]?.visibleWhen;
    if (!fieldVisibleWhen) return true;
    return matches(fieldVisibleWhen);
  });
}
