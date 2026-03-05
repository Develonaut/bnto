/**
 * removeNode action — pure function that computes the next editor state
 * after removing a node.
 *
 * Enforces the I/O deletion guard (I/O nodes cannot be removed),
 * captures an undo snapshot, removes the node + config,
 * auto-selects the nearest neighbor, and revalidates.
 *
 * Returns null if the removal is blocked (I/O node).
 */

import type { EditorState } from "../store/types";
import { captureSnapshot } from "../store/captureSnapshot";
import { pushToStack } from "../store/pushToStack";
import { revalidateState } from "../store/revalidateState";
import { isIoNodeType } from "../helpers/isIoNodeType";

export function removeNode(
  state: EditorState,
  id: string,
): Partial<EditorState> | null {
  // I/O nodes are structural — they cannot be deleted.
  const config = state.configs[id];
  if (config && isIoNodeType(config.nodeType)) return null;

  const snapshot = captureSnapshot(state.nodes, state.configs);

  const removedIndex = state.nodes.findIndex((n) => n.id === id);
  const nextNodes = state.nodes.filter((n) => n.id !== id);
  const nextConfigs = { ...state.configs };
  delete nextConfigs[id];

  // Auto-select the nearest remaining node after removal.
  if (nextNodes.length > 0) {
    const selectIndex = Math.min(
      removedIndex > 0 ? removedIndex - 1 : 0,
      nextNodes.length - 1,
    );
    nextNodes[selectIndex] = { ...nextNodes[selectIndex]!, selected: true };
  }

  return {
    nodes: nextNodes,
    configs: nextConfigs,
    isDirty: true,
    undoStack: pushToStack(state.undoStack, snapshot),
    redoStack: [],
    validationErrors: revalidateState(
      nextNodes,
      nextConfigs,
      state.recipeMetadata,
    ),
  };
}
