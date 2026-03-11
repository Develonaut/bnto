/**
 * updateParams action — pure function that merges new parameters into a node's config.
 *
 * Takes the current EditorState, a nodeId, and a params object.
 * Returns the next partial state (configs, undo, dirty, validation) or null
 * if the node doesn't exist. Config changes don't touch the RF nodes array —
 * no RF re-render needed.
 *
 * Child-aware: if the node has a parentContainerId (it's a child of an
 * expanded container), also writes through to the definition tree.
 */

import type { EditorState } from "../store/types";
import { withUndo } from "../store/withUndo";
import { updateNodeInTree } from "../adapters/definitionTreeHelpers";

export function updateParams(
  state: EditorState,
  nodeId: string,
  params: Record<string, unknown>,
): Partial<EditorState> | null {
  const existing = state.configs[nodeId];
  if (!existing) return null;

  const nextConfigs = {
    ...state.configs,
    [nodeId]: {
      ...existing,
      parameters: { ...existing.parameters, ...params },
    },
  };

  // Write-through to definition tree for child nodes
  const node = state.nodes.find((n) => n.id === nodeId);
  let nextDefinition = state.definition;
  if (node?.data.parentContainerId && nextDefinition) {
    nextDefinition = updateNodeInTree(nextDefinition, nodeId, params);
  }

  return withUndo(state, {
    configs: nextConfigs,
    ...(nextDefinition !== state.definition ? { definition: nextDefinition } : {}),
  });
}
