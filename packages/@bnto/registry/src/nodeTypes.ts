/** Node type lookup functions — stateless, reads from @bnto/nodes static exports. */

import { NODE_TYPE_INFO } from "@bnto/nodes";
import type { NodeTypeInfo, NodeTypeName } from "@bnto/nodes";

/** Returns all node types keyed by name. */
export function getAllNodeTypes(): Record<NodeTypeName, NodeTypeInfo> {
  return NODE_TYPE_INFO;
}

/** Returns only node types that can execute in the browser (WASM). */
export function getBrowserNodeTypes(): NodeTypeInfo[] {
  return Object.values(NODE_TYPE_INFO).filter((t) => t.browserCapable);
}
