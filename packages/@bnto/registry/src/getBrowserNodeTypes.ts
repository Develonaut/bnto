/** Returns only browser-capable (WASM) node types — stateless read from @bnto/nodes. */

import { NODE_TYPE_INFO } from "@bnto/nodes";
import type { NodeTypeInfo } from "@bnto/nodes";

export function getBrowserNodeTypes(): NodeTypeInfo[] {
  return Object.values(NODE_TYPE_INFO).filter((t) => t.platforms.includes("browser"));
}
