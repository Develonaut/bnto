/** Returns all node types keyed by name — stateless read from @bnto/nodes. */

import { NODE_TYPE_INFO } from "@bnto/nodes";
import type { NodeTypeInfo, NodeTypeName } from "@bnto/nodes";

export function getAllNodeTypes(): Record<NodeTypeName, NodeTypeInfo> {
  return NODE_TYPE_INFO;
}
