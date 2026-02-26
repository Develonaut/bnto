/**
 * Returns only the required parameters for a node type.
 */

import type { ParameterSchema } from "./types";
import { getNodeSchema } from "./getNodeSchema";

/** Returns only the required parameters for a node type. */
export function getRequiredParams(
  typeName: string,
): readonly ParameterSchema[] {
  const schema = getNodeSchema(typeName);
  if (!schema) return [];
  return schema.parameters.filter((p) => p.required);
}
