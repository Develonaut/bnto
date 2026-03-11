import type { NodeChange } from "@xyflow/react";
import type { BentoNode } from "../adapters/types";
import { PLACEHOLDER_ID } from "./injectPlaceholder";
import { GROUP_ID_PREFIX } from "../adapters/layoutNodes";
import { ADD_DIVIDER_PREFIX } from "./injectAddDividers";

/**
 * Filter out ReactFlow node changes targeting injected nodes.
 * RF reports dimension/position changes for all rendered nodes, including
 * injected ones the store doesn't know about (placeholder, containerGroup,
 * addDivider).
 */
function filterPlaceholderChanges(changes: NodeChange<BentoNode>[]): NodeChange<BentoNode>[] {
  return changes.filter((c) => {
    if (!("id" in c)) return true;
    if (c.id === PLACEHOLDER_ID) return false;
    if (typeof c.id === "string" && c.id.startsWith(GROUP_ID_PREFIX)) return false;
    if (typeof c.id === "string" && c.id.startsWith(ADD_DIVIDER_PREFIX)) return false;
    return true;
  });
}

export { filterPlaceholderChanges };
