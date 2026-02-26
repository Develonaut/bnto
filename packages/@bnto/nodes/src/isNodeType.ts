/**
 * Type guard for node type names.
 */

import type { NodeTypeName } from "./nodeTypes";
import { NODE_TYPE_NAMES } from "./nodeTypes";

/** Returns true if the given string is a registered node type name. */
export function isNodeType(value: string): value is NodeTypeName {
  return NODE_TYPE_NAMES.includes(value as NodeTypeName);
}
