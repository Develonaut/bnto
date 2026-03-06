/**
 * Returns parameter names that are visible for a given condition.
 */

import { getNodeSchema } from "./getNodeSchema";
import { matchesCondition } from "./matchesCondition";
import type { NodeSchemaDefinition, ParamCondition } from "./types";

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

  // Single param/value check (original API)
  if (typeof paramNameOrValues === "string") {
    return resolveVisible(schemaDef, (cond) =>
      matchesCondition(cond, paramNameOrValues, paramValue!),
    );
  }

  // Full parameters map check
  const parameters = paramNameOrValues;
  return resolveVisible(schemaDef, (cond) => {
    const conditions = Array.isArray(cond) ? cond : [cond];
    return conditions.some((c) => String(parameters[c.param] ?? "") === c.equals);
  });
}

/** Shared filter logic — hidden first, then visibleWhen via matcher. */
function resolveVisible(
  schemaDef: NodeSchemaDefinition,
  matches: (cond: ParamCondition) => boolean,
): string[] {
  return Object.entries(schemaDef.params)
    .filter(([, meta]) => {
      if (meta.hidden) return false;
      if (!meta.visibleWhen) return true;
      return matches(meta.visibleWhen);
    })
    .map(([name]) => name);
}
