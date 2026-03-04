/**
 * Store-delegate node removal.
 *
 * Delegates to the editor store's removeNode action, which updates the
 * definition. Existing sync effects propagate changes to ReactFlow.
 *
 * Must be inside EditorProvider.
 */

"use client";

import { useCallback } from "react";
import { useEditorStoreApi } from "./useEditorStoreApi";

function useRemoveNode() {
  const storeApi = useEditorStoreApi();

  return useCallback(
    (id: string) => {
      storeApi.getState().removeNode(id);
    },
    [storeApi],
  );
}

export { useRemoveNode };
