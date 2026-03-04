/**
 * Store-delegate node addition.
 *
 * Delegates to the editor store's addNode action, which updates the
 * definition. Existing sync effects propagate changes to ReactFlow.
 *
 * Must be inside EditorProvider.
 */

"use client";

import { useCallback } from "react";
import type { NodeTypeName, Position } from "@bnto/nodes";
import { useEditorStoreApi } from "./useEditorStoreApi";

function useAddNode() {
  const storeApi = useEditorStoreApi();

  return useCallback(
    (type: NodeTypeName, position?: Position): string | null => {
      return storeApi.getState().addNode(type, position);
    },
    [storeApi],
  );
}

export { useAddNode };
