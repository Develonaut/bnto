/**
 * Returns parameter names that are visible for a given condition.
 */

import { getNodeSchema } from "./getNodeSchema";
import { matchesCondition } from "./matchesCondition";

/**
 * Returns parameter names that are visible when a specific parameter
 * has a specific value.
 *
 * Parameters without a visibleWhen condition are always visible.
 *
 * Example: `getVisibleParams("image", "operation", "resize")`
 * returns names including width, height, and maintainAspect.
 */
export function getVisibleParams(
  typeName: string,
  paramName: string,
  paramValue: string,
): string[] {
  const schemaDef = getNodeSchema(typeName);
  if (!schemaDef) return [];

  return Object.entries(schemaDef.params)
    .filter(([, meta]) => {
      if (!meta.visibleWhen) return true;
      return matchesCondition(meta.visibleWhen, paramName, paramValue);
    })
    .map(([name]) => name);
}
