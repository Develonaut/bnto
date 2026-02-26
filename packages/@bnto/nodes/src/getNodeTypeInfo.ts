/**
 * Lookup function for node type metadata.
 */

import type { NodeTypeName, NodeTypeInfo } from "./nodeTypes";
import { NODE_TYPE_INFO } from "./nodeTypes";

/** Returns the info for a node type, or undefined if not registered. */
export function getNodeTypeInfo(
  typeName: string,
): NodeTypeInfo | undefined {
  return NODE_TYPE_INFO[typeName as NodeTypeName];
}
