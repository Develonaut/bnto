import type { NodeChange } from "@xyflow/react";
import type { BentoNode } from "../adapters/types";
import { PLACEHOLDER_ID } from "./injectPlaceholder";

/**
 * Filter out ReactFlow node changes targeting the injected placeholder.
 * RF reports dimension/position changes for all rendered nodes, including
 * injected ones the store doesn't know about.
 */
function filterPlaceholderChanges(changes: NodeChange<BentoNode>[]): NodeChange<BentoNode>[] {
  return changes.filter((c) => !("id" in c && c.id === PLACEHOLDER_ID));
}

export { filterPlaceholderChanges };
