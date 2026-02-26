/**
 * Returns all node types that can run in the browser.
 */

import type { NodeTypeInfo } from "./nodeTypes";
import { NODE_TYPE_NAMES, NODE_TYPE_INFO } from "./nodeTypes";

/** Returns all node types that can run in the browser. */
export function getBrowserCapableTypes(): readonly NodeTypeInfo[] {
  return NODE_TYPE_NAMES.filter((n) => NODE_TYPE_INFO[n].browserCapable).map(
    (n) => NODE_TYPE_INFO[n],
  );
}
