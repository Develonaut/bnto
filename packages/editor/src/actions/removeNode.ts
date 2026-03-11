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
import type { BentoNode } from "../adapters/types";
import type { Definition } from "@bnto/nodes";
import { withUndo } from "../store/withUndo";
import { isIoNodeType } from "@bnto/nodes";
import { STRIDE } from "../adapters/bentoSlots";
import { removeChildFromContainer } from "../adapters/definitionTreeHelpers";

export function removeNode(state: EditorState, id: string): Partial<EditorState> | null {
  const config = state.configs[id];
  if (config && isIoNodeType(config.nodeType)) return null;

  const removedNode = state.nodes.find((n) => n.id === id);
  const parentContainerId = removedNode?.data.parentContainerId;
  const idsToRemove = new Set([id]);
  collectDescendants(state, id, idsToRemove);

  const removedIndex = state.nodes.findIndex((n) => n.id === id);
  const { nodes: reflowed, configs: nextConfigs } = removeAndReflow(state, idsToRemove, parentContainerId);
  autoSelectNearest(reflowed, parentContainerId, removedIndex);

  const nextDefinition = removeFromDefinition(state.definition, id, parentContainerId);
  const nextExpandedIds = cleanExpandedIds(state.expandedContainerIds, idsToRemove);

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

/** Filter removed nodes, reflow same-level positions, clean up configs. */
function removeAndReflow(
  state: EditorState,
  idsToRemove: Set<string>,
  parentContainerId: string | undefined,
) {
  const nextNodes = state.nodes.filter((n) => !idsToRemove.has(n.id));
  const configs = { ...state.configs };
  for (const rid of idsToRemove) delete configs[rid];

  const sameLevel = nextNodes.filter((n) => n.data.parentContainerId === parentContainerId);
  const otherLevel = nextNodes.filter((n) => n.data.parentContainerId !== parentContainerId);

  for (let i = 0; i < sameLevel.length; i++) {
    const expectedX = i * STRIDE;
    if (sameLevel[i]!.position.x !== expectedX) {
      sameLevel[i] = { ...sameLevel[i]!, position: { ...sameLevel[i]!.position, x: expectedX } };
    }
  }

  return { nodes: [...sameLevel, ...otherLevel], configs };
}

/** Auto-select the nearest remaining same-level node after removal. */
function autoSelectNearest(
  nodes: BentoNode[],
  parentContainerId: string | undefined,
  removedIndex: number,
): void {
  const sameLevelNodes = nodes.filter((n) => n.data.parentContainerId === parentContainerId);
  if (sameLevelNodes.length === 0) return;

  const selectIdx = Math.min(removedIndex > 0 ? removedIndex - 1 : 0, sameLevelNodes.length - 1);
  const selectId = sameLevelNodes[selectIdx]!.id;
  const nodeIdx = nodes.findIndex((n) => n.id === selectId);
  if (nodeIdx >= 0) {
    nodes[nodeIdx] = { ...nodes[nodeIdx]!, selected: true };
  }
}

/** Remove node from the definition tree (root or container child). */
function removeFromDefinition(
  definition: Definition | null,
  id: string,
  parentContainerId: string | undefined,
): Definition | null {
  if (!definition) return definition;

  if (parentContainerId) {
    return removeChildFromContainer(definition, parentContainerId, id);
  }
  const rootNodes = definition.nodes ?? [];
  const filtered = rootNodes.filter((n) => n.id !== id);
  if (filtered.length === rootNodes.length) return definition;
  return { ...definition, nodes: filtered };
}

/** Remove all deleted IDs from the expanded container set. */
function cleanExpandedIds(expandedIds: Set<string>, idsToRemove: Set<string>): Set<string> {
  const next = new Set(expandedIds);
  for (const rid of idsToRemove) next.delete(rid);
  return next;
}
