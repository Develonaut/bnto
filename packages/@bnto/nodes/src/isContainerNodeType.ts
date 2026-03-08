/**
 * isContainerNodeType — returns true if the node type can hold child nodes.
 *
 * Derived from NODE_TYPE_INFO.isContainer, not hardcoded names.
 * When a new container node type is added, it only needs isContainer: true
 * in NODE_TYPE_INFO and this function picks it up automatically.
 */

import type { NodeTypeName } from "./nodeTypes";
import { NODE_TYPE_INFO } from "./nodeTypes";

export function isContainerNodeType(nodeType: string): boolean {
  const info = NODE_TYPE_INFO[nodeType as NodeTypeName];
  return info?.isContainer === true;
}
