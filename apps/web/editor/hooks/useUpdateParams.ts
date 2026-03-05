/**
 * useUpdateParams — updates a node's config parameters.
 *
 * Business logic hook (layer 3 in the editor architecture):
 *   1. Captures undo snapshot
 *   2. Merges new params into the existing config
 *   3. Revalidates the definition
 *
 * Config changes don't touch the RF nodes array — no RF re-render.
 */

"use client";

import { useCallback } from "react";
import { useEditorStoreApi } from "./useEditorStoreApi";
import { captureSnapshot } from "../store/captureSnapshot";
import { pushToStack } from "../store/pushToStack";
import { revalidateState } from "../store/revalidateState";

function useUpdateParams() {
  const storeApi = useEditorStoreApi();

  return useCallback(
    (nodeId: string, params: Record<string, unknown>): void => {
      const state = storeApi.getState();
      const existing = state.configs[nodeId];
      if (!existing) return;

      const snapshot = captureSnapshot(state.nodes, state.configs);
      const nextConfigs = {
        ...state.configs,
        [nodeId]: {
          ...existing,
          parameters: { ...existing.parameters, ...params },
        },
      };

      storeApi.setState({
        configs: nextConfigs,
        isDirty: true,
        undoStack: pushToStack(state.undoStack, snapshot),
        redoStack: [],
        validationErrors: revalidateState(state.nodes, nextConfigs, state.recipeMetadata),
      });
    },
    [storeApi],
  );
}

export { useUpdateParams };
