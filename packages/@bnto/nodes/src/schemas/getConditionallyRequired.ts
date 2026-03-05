/**
 * Returns parameter names that are conditionally required.
 */

import { getNodeSchema } from "./getNodeSchema";
import { matchesCondition } from "./matchesCondition";

/**
 * Returns parameter names that are conditionally required when a
 * specific parameter has a specific value.
 *
 * Example: `getConditionallyRequired("loop", "mode", "forEach")`
 * returns `["items"]`.
 */
export function getConditionallyRequired(
  typeName: string,
  paramName: string,
  paramValue: string,
): string[] {
  const schemaDef = getNodeSchema(typeName);
  if (!schemaDef) return [];

  return Object.entries(schemaDef.params)
    .filter(([, meta]) => matchesCondition(meta.requiredWhen, paramName, paramValue))
    .map(([name]) => name);
}
