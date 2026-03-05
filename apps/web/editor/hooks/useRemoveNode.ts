/**
 * useRemoveNode — removes a node from the editor canvas.
 *
 * Business logic hook (layer 3 in the editor architecture):
 *   1. Captures undo snapshot
 *   2. Removes the node + its config
 *   3. Auto-selects the nearest remaining node (UI behavior)
 *   4. Revalidates the definition
 *
 * Auto-select-on-remove is a UI concern — it belongs in this hook,
 * not in the state store.
 */

"use client";

import { useCallback } from "react";
import { useEditorStoreApi } from "./useEditorStoreApi";
import { captureSnapshot, pushToStack, revalidateState } from "../store/helpers";

function useRemoveNode() {
  const storeApi = useEditorStoreApi();

  return useCallback(
    (id: string): void => {
      const state = storeApi.getState();
      const snapshot = captureSnapshot(state.nodes, state.configs);

      const removedIndex = state.nodes.findIndex((n) => n.id === id);
      const nextNodes = state.nodes.filter((n) => n.id !== id);
      const nextConfigs = { ...state.configs };
      delete nextConfigs[id];

      // Auto-select the nearest remaining node after removal.
      // This is a UI behavior concern — when a user deletes a node,
      // the selection should jump to the neighbor, not disappear.
      if (nextNodes.length > 0) {
        const selectIndex = Math.min(
          removedIndex > 0 ? removedIndex - 1 : 0,
          nextNodes.length - 1,
        );
        nextNodes[selectIndex] = { ...nextNodes[selectIndex]!, selected: true };
      }

      storeApi.setState({
        nodes: nextNodes,
        configs: nextConfigs,
        isDirty: true,
        undoStack: pushToStack(state.undoStack, snapshot),
        redoStack: [],
        validationErrors: revalidateState(nextNodes, nextConfigs, state.recipeMetadata),
      });
    },
    [storeApi],
  );
}

export { useRemoveNode };
