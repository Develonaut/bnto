/**
 * Returns parameters that are visible for a given condition.
 */

import type { ParameterSchema } from "./types";
import { getNodeSchema } from "./getNodeSchema";
import { matchesCondition } from "./matchesCondition";

/**
 * Returns parameters that are visible when a specific parameter
 * has a specific value.
 *
 * Example: `getVisibleParams("image", "operation", "resize")`
 * returns width, height, and maintainAspect parameters.
 */
export function getVisibleParams(
  typeName: string,
  paramName: string,
  paramValue: string,
): readonly ParameterSchema[] {
  const schema = getNodeSchema(typeName);
  if (!schema) return [];
  return schema.parameters.filter((p) => {
    if (!p.visibleWhen) return true;
    return matchesCondition(p.visibleWhen, paramName, paramValue);
  });
}
