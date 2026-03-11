/**
 * removeNode action — pure function that computes the next editor state
 * after removing a node.
 *
 * Enforces the I/O deletion guard (I/O nodes cannot be removed),
 * captures an undo snapshot, removes the node + config,
 * auto-selects the nearest neighbor, and revalidates.
 *
 * Child-aware: if the removed node is a child of a container, it is
 * also removed from the definition tree. If the removed node is an
 * expanded container, its children are also removed.
 *
 * Returns null if the removal is blocked (I/O node).
 */

import type { EditorState } from "../store/types";
import { withUndo } from "../store/withUndo";
import { isIoNodeType } from "@bnto/nodes";
import { STRIDE } from "../adapters/bentoSlots";
import { removeChildFromContainer } from "../adapters/definitionTreeHelpers";

export function removeNode(state: EditorState, id: string): Partial<EditorState> | null {
  // I/O nodes are structural — they cannot be deleted.
  const config = state.configs[id];
  if (config && isIoNodeType(config.nodeType)) return null;

  const removedNode = state.nodes.find((n) => n.id === id);
  const parentContainerId = removedNode?.data.parentContainerId;

  // Collect IDs to remove: the node + any expanded descendants
  const idsToRemove = new Set([id]);
  collectDescendants(state, id, idsToRemove);

  const removedIndex = state.nodes.findIndex((n) => n.id === id);

  // Filter out removed nodes, separating by level for reflow
  const nextNodes = state.nodes.filter((n) => !idsToRemove.has(n.id));
  const nextConfigs = { ...state.configs };
  for (const rid of idsToRemove) {
    delete nextConfigs[rid];
  }

  // Reflow positions for same-level siblings only
  const sameLevel = nextNodes.filter((n) => n.data.parentContainerId === parentContainerId);
  const otherLevel = nextNodes.filter((n) => n.data.parentContainerId !== parentContainerId);

  for (let i = 0; i < sameLevel.length; i++) {
    const expectedX = i * STRIDE;
    if (sameLevel[i]!.position.x !== expectedX) {
      sameLevel[i] = { ...sameLevel[i]!, position: { ...sameLevel[i]!.position, x: expectedX } };
    }
  }

  const reflowed = [...sameLevel, ...otherLevel];

  // Auto-select the nearest remaining same-level node after removal.
  const sameLevelNodes = reflowed.filter((n) => n.data.parentContainerId === parentContainerId);
  if (sameLevelNodes.length > 0) {
    const selectIdx = Math.min(
      removedIndex > 0 ? removedIndex - 1 : 0,
      sameLevelNodes.length - 1,
    );
    const selectId = sameLevelNodes[selectIdx]!.id;
    const nodeIdx = reflowed.findIndex((n) => n.id === selectId);
    if (nodeIdx >= 0) {
      reflowed[nodeIdx] = { ...reflowed[nodeIdx]!, selected: true };
    }
  }

  // Update definition tree — remove child from parent container
  let nextDefinition = state.definition;
  if (nextDefinition && parentContainerId) {
    nextDefinition = removeChildFromContainer(nextDefinition, parentContainerId, id);
  }

  // If the removed node was expanded, clean up expandedContainerIds
  const nextExpandedIds = new Set(state.expandedContainerIds);
  for (const rid of idsToRemove) {
    nextExpandedIds.delete(rid);
  }

  return withUndo(state, {
    nodes: reflowed,
    configs: nextConfigs,
    ...(nextDefinition !== state.definition ? { definition: nextDefinition } : {}),
    ...(nextExpandedIds.size !== state.expandedContainerIds.size
      ? { expandedContainerIds: nextExpandedIds }
      : {}),
  });
}

/** Recursively collect descendant node IDs of an expanded container. */
function collectDescendants(state: EditorState, containerId: string, ids: Set<string>): void {
  for (const node of state.nodes) {
    if (node.data.parentContainerId === containerId) {
      ids.add(node.id);
      if (state.expandedContainerIds.has(node.id)) {
        collectDescendants(state, node.id, ids);
      }
    }
  }
}
