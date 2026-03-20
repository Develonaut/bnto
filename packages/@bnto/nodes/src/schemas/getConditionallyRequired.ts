/**
 * Returns parameter names that are conditionally required.
 *
 * Reads requiredWhen from FieldConfig (UI concern). Falls back to
 * looking up NODE_FIELD_CONFIGS automatically via getNodeFields().
 */

import { getNodeSchema } from "./getNodeSchema";
import { getNodeFields } from "./getNodeFields";
import { matchesCondition } from "./matchesCondition";

/**
 * Returns parameter names that are conditionally required when a
 * specific parameter has a specific value.
 *
 * Example: `getConditionallyRequired("loop", "mode", "times")`
 * returns `["count"]`.
 */
export function getConditionallyRequired(
  typeName: string,
  paramName: string,
  paramValue: string,
): string[] {
  const schemaDef = getNodeSchema(typeName);
  if (!schemaDef) return [];

  const fields = getNodeFields(typeName);

  return Object.keys(schemaDef.params).filter((name) => {
    const fieldRequiredWhen = fields?.[name]?.requiredWhen;
    return matchesCondition(fieldRequiredWhen, paramName, paramValue);
  });
}
