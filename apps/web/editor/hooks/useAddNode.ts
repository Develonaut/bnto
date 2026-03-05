/**
 * useAddNode — thin reactive wrapper around the addNode action.
 *
 * The business logic (I/O singleton guard, node creation, undo capture)
 * lives in `actions/addNode`. This hook just bridges it to the store.
 */

"use client";

import { useCallback } from "react";
import type { NodeTypeName } from "@bnto/nodes";
import { useEditorStoreApi } from "./useEditorStoreApi";
import { addNode } from "../actions/addNode";

function useAddNode() {
  const storeApi = useEditorStoreApi();

  return useCallback(
    (type: NodeTypeName, position?: { x: number; y: number }): string | null => {
      const result = addNode(storeApi.getState(), type, position);
      if (!result) return null;
      storeApi.setState(result.nextState);
      return result.nodeId;
    },
    [storeApi],
  );
}

export { useAddNode };
