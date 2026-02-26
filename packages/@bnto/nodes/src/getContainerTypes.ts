/**
 * Returns all container node types (group, loop, parallel).
 */

import type { NodeTypeInfo } from "./nodeTypes";
import { NODE_TYPE_NAMES, NODE_TYPE_INFO } from "./nodeTypes";

/** Returns all container node types (group, loop, parallel). */
export function getContainerTypes(): readonly NodeTypeInfo[] {
  return NODE_TYPE_NAMES.filter((n) => NODE_TYPE_INFO[n].isContainer).map(
    (n) => NODE_TYPE_INFO[n],
  );
}
