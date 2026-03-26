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
import type { NodeConfigs } from "../adapters/types";
import { updateNodeInTree } from "../adapters/definitionTreeHelpers";
import { withUndo } from "../store/withUndo";

/** Write-through: sync child configs back to definition tree. */
function syncChildConfigsToDefinition(
  definition: EditorState["definition"],
  idsToRemove: Set<string>,
  configs: NodeConfigs,
) {
  let next = definition;
  if (!next) return next;
  for (const childId of idsToRemove) {
    const config = configs[childId];
    if (config) next = updateNodeInTree(next, childId, config.parameters);
  }
  return next;
}

export function collapseContainer(state: EditorState, nodeId: string): Partial<EditorState> | null {
  if (!state.expandedContainerIds.has(nodeId)) return null;
  if (!state.definition) return null;

  const idsToRemove = collectDescendantIds(state, nodeId);
  const nextDefinition = syncChildConfigsToDefinition(state.definition, idsToRemove, state.configs);

  const nextNodes = state.nodes
    .filter((n) => !idsToRemove.has(n.id))
    .map((n) => (n.id === nodeId ? { ...n, data: { ...n.data, isExpanded: false } } : n));

  const nextConfigs = { ...state.configs };
  for (const id of idsToRemove) delete nextConfigs[id];

  const nextExpandedIds = new Set(state.expandedContainerIds);
  nextExpandedIds.delete(nodeId);
  for (const id of idsToRemove) nextExpandedIds.delete(id);

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
      if (state.expandedContainerIds.has(node.id)) {
        for (const nestedId of collectDescendantIds(state, node.id)) {
          ids.add(nestedId);
        }
      }
    }
  }
  return ids;
}
