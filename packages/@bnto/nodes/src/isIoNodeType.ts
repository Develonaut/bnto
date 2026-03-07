/**
 * isIoNodeType — returns true if the node type is an I/O declaration node.
 *
 * Derived from NODE_TYPE_INFO category, not hardcoded names.
 * When a new I/O node type is added, it only needs category: "io"
 * in NODE_TYPE_INFO and this function picks it up automatically.
 */

import type { NodeTypeName } from "./nodeTypes";
import { NODE_TYPE_INFO } from "./nodeTypes";

export function isIoNodeType(nodeType: string): boolean {
  const info = NODE_TYPE_INFO[nodeType as NodeTypeName];
  return info?.category === "io";
}
