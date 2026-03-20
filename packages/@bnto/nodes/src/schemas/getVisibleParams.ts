/**
 * Returns parameter names that are visible for a given condition.
 *
 * Hidden filtering uses FieldConfigMap (UI concern) when provided.
 * Falls back to looking up NODE_FIELD_CONFIGS automatically.
 */

import { getNodeSchema } from "./getNodeSchema";
import { getNodeFields } from "./getNodeFields";
import { matchesCondition } from "./matchesCondition";
import type { FieldConfigMap, NodeSchemaDefinition, ParamCondition } from "./types";

/**
 * Returns parameter names that are visible when a specific parameter
 * has a specific value.
 *
 * Parameters without a visibleWhen condition are always visible.
 *
 * Example: `getVisibleParams("image", "operation", "resize")`
 * returns names including width, height, and maintainAspect.
 */
export function getVisibleParams(typeName: string, paramName: string, paramValue: string): string[];

/**
 * Returns parameter names that are visible given the full current
 * parameter values. Evaluates each param's visibleWhen against all
 * current values (OR logic for array conditions).
 *
 * Example: `getVisibleParams("image", { operation: "resize", quality: 80 })`
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
    return resolveVisible(schemaDef, fields, (cond) =>
      matchesCondition(cond, paramNameOrValues, paramValue!),
    );
  }

  // Full parameters map check
  const parameters = paramNameOrValues;
  return resolveVisible(schemaDef, fields, (cond) => {
    const conditions = Array.isArray(cond) ? cond : [cond];
    return conditions.some((c) => String(parameters[c.param] ?? "") === c.equals);
  });
}

/** Shared filter logic — hidden via fields, then visibleWhen via matcher. */
function resolveVisible(
  schemaDef: NodeSchemaDefinition,
  fields: FieldConfigMap | undefined,
  matches: (cond: ParamCondition) => boolean,
): string[] {
  return Object.entries(schemaDef.params)
    .filter(([name, meta]) => {
      if (fields?.[name]?.hidden) return false;
      if (!meta.visibleWhen) return true;
      return matches(meta.visibleWhen);
    })
    .map(([name]) => name);
}
