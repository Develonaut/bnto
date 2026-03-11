/**
 * collapseContainer action — syncs children back to definition and removes from graph.
 *
 * Before removing child nodes from `state.nodes` + `state.configs`, writes
 * their current config parameters back to the definition tree. This ensures
 * edits made while expanded are preserved on collapse.
 *
 * Also recursively collapses any expanded children (nested containers).
 *
 * Returns null if the container is not expanded.
 */

import type { EditorState } from "../store/types";
import { updateNodeInTree } from "../adapters/definitionTreeHelpers";
import { withUndo } from "../store/withUndo";

export function collapseContainer(
  state: EditorState,
  nodeId: string,
): Partial<EditorState> | null {
  if (!state.expandedContainerIds.has(nodeId)) return null;
  if (!state.definition) return null;

  // Collect all child node IDs (direct + nested) to remove from graph
  const idsToRemove = collectDescendantIds(state, nodeId);

  // Write-through: sync child configs back to definition tree
  let nextDefinition = state.definition;
  for (const childId of idsToRemove) {
    const config = state.configs[childId];
    if (config) {
      nextDefinition = updateNodeInTree(nextDefinition, childId, config.parameters);
    }
  }

  // Remove child nodes from graph
  const nextNodes = state.nodes
    .filter((n) => !idsToRemove.has(n.id))
    .map((n) =>
      n.id === nodeId
        ? { ...n, data: { ...n.data, isExpanded: false } }
        : n,
    );

  // Remove child configs
  const nextConfigs = { ...state.configs };
  for (const id of idsToRemove) {
    delete nextConfigs[id];
  }

  // Remove container + any nested expanded containers from expanded set
  const nextExpandedIds = new Set(state.expandedContainerIds);
  nextExpandedIds.delete(nodeId);
  for (const id of idsToRemove) {
    nextExpandedIds.delete(id);
  }

  return withUndo(state, {
    nodes: nextNodes,
    configs: nextConfigs,
    definition: nextDefinition,
    expandedContainerIds: nextExpandedIds,
  });
}

/** Collect all descendant node IDs of a container (direct children + nested). */
function collectDescendantIds(state: EditorState, containerId: string): Set<string> {
  const ids = new Set<string>();
  for (const node of state.nodes) {
    if (node.data.parentContainerId === containerId) {
      ids.add(node.id);
      // Recursively collect nested children if this child is also expanded
      if (state.expandedContainerIds.has(node.id)) {
        for (const nestedId of collectDescendantIds(state, node.id)) {
          ids.add(nestedId);
        }
      }
    }
  }
  return ids;
}
