/**
 * resolveShiftArrow — pure function for Shift+Arrow tree navigation.
 *
 * Given the current tree state and a key, returns a command describing
 * what the editor should do. No React, no store.
 */

import type { BentoNode } from "../adapters/types";
import { resolveTreeNav } from "./treeNavigation";

type ShiftArrowResult =
  | { kind: "select"; nodeId: string }
  | { kind: "expand"; nodeId: string }
  | { kind: "collapse"; nodeId: string }
  | null;

function resolveShiftArrow(
  key: string,
  flatIds: string[],
  nodes: BentoNode[],
  selectedNodeId: string | null,
  expandedContainerIds: Set<string>,
): ShiftArrowResult {
  if (key === "ArrowUp" || key === "ArrowDown") {
    const direction = key === "ArrowDown" ? "down" : "up";
    const nextId = resolveTreeNav(flatIds, selectedNodeId, direction);
    return nextId ? { kind: "select", nodeId: nextId } : null;
  }

  if (!selectedNodeId) return null;
  const node = nodes.find((n) => n.id === selectedNodeId);
  if (!node) return null;

  if (key === "ArrowRight" && node.data.isContainer) {
    if (!expandedContainerIds.has(selectedNodeId)) {
      return { kind: "expand", nodeId: selectedNodeId };
    }
    const childId = resolveTreeNav(flatIds, selectedNodeId, "down");
    return childId ? { kind: "select", nodeId: childId } : null;
  }

  if (key === "ArrowLeft") {
    if (node.data.isContainer && expandedContainerIds.has(selectedNodeId)) {
      return { kind: "collapse", nodeId: selectedNodeId };
    }
    if (node.data.parentContainerId) {
      return { kind: "select", nodeId: node.data.parentContainerId };
    }
  }

  return null;
}

export { resolveShiftArrow };
export type { ShiftArrowResult };
