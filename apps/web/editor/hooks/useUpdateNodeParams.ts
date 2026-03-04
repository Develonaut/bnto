/**
 * Store-delegate parameter update.
 *
 * Delegates to the editor store's updateParams action, which updates the
 * definition. Existing sync effects propagate changes to ReactFlow.
 *
 * Must be inside EditorProvider.
 */

"use client";

import { useCallback } from "react";
import { useEditorStoreApi } from "./useEditorStoreApi";

function useUpdateNodeParams() {
  const storeApi = useEditorStoreApi();

  return useCallback(
    (nodeId: string, params: Record<string, unknown>) => {
      storeApi.getState().updateParams(nodeId, params);
    },
    [storeApi],
  );
}

export { useUpdateNodeParams };
