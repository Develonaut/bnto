/**
 * Returns parameters that are conditionally required.
 */

import type { ParameterSchema } from "./types";
import { getNodeSchema } from "./getNodeSchema";
import { matchesCondition } from "./matchesCondition";

/**
 * Returns parameters that are conditionally required when a
 * specific parameter has a specific value.
 *
 * Example: `getConditionallyRequired("loop", "mode", "forEach")`
 * returns `[{ name: "items", ... }]`.
 */
export function getConditionallyRequired(
  typeName: string,
  paramName: string,
  paramValue: string,
): readonly ParameterSchema[] {
  const schema = getNodeSchema(typeName);
  if (!schema) return [];
  return schema.parameters.filter((p) =>
    matchesCondition(p.requiredWhen, paramName, paramValue),
  );
}
