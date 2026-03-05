/**
 * updateParams action — pure function that merges new parameters into a node's config.
 *
 * Takes the current EditorState, a nodeId, and a params object.
 * Returns the next partial state (configs, undo, dirty, validation) or null
 * if the node doesn't exist. Config changes don't touch the RF nodes array —
 * no RF re-render needed.
 */

import type { EditorState } from "../store/types";
import { captureSnapshot } from "../store/captureSnapshot";
import { pushToStack } from "../store/pushToStack";
import { revalidateState } from "../store/revalidateState";

export function updateParams(
  state: EditorState,
  nodeId: string,
  params: Record<string, unknown>,
): Partial<EditorState> | null {
  const existing = state.configs[nodeId];
  if (!existing) return null;

  const snapshot = captureSnapshot(state.nodes, state.configs);
  const nextConfigs = {
    ...state.configs,
    [nodeId]: {
      ...existing,
      parameters: { ...existing.parameters, ...params },
    },
  };

  return {
    configs: nextConfigs,
    isDirty: true,
    undoStack: pushToStack(state.undoStack, snapshot),
    redoStack: [],
    validationErrors: revalidateState(state.nodes, nextConfigs, state.recipeMetadata),
  };
}
