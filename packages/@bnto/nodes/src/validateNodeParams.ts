/**
 * Validates node parameters against the Zod schema for a given node type.
 *
 * Returns field-level errors suitable for UI display.
 * Returns an empty array if validation passes or the node type is unknown.
 */

import type { NodeTypeName } from "./nodeTypes";
import type { ValidationError } from "./validationError";
import { NODE_SCHEMA_DEFS } from "./schemas/registry";

/**
 * Validates parameters for a specific node type using Zod.
 *
 * @param nodeType - The node type name (e.g., "image", "http-request")
 * @param nodeId - The node ID (for error messages)
 * @param params - The parameters object to validate
 * @returns Array of field-level validation errors (empty if valid)
 */
export function validateNodeParams(
  nodeType: string,
  nodeId: string,
  params: Record<string, unknown>,
): ValidationError[] {
  const schemaDef = NODE_SCHEMA_DEFS[nodeType as NodeTypeName];
  if (!schemaDef) return [];

  const result = schemaDef.schema.safeParse(params);
  if (result.success) return [];

  return result.error.issues.map((issue) => ({
    nodeId,
    field: issue.path.join(".") || nodeType,
    message: `node '${nodeId}': ${issue.path.length ? `parameter '${issue.path.join(".")}' ` : ""}${issue.message}`,
  }));
}
