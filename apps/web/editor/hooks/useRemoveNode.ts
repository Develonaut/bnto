/**
 * useRemoveNode — thin reactive wrapper around the removeNode action.
 *
 * The business logic (I/O deletion guard, undo capture, auto-select)
 * lives in `actions/removeNode`. This hook just bridges it to the store.
 */

"use client";

import { useCallback } from "react";
import { useEditorStoreApi } from "./useEditorStoreApi";
import { removeNode } from "../actions/removeNode";

function useRemoveNode() {
  const storeApi = useEditorStoreApi();

  return useCallback(
    (id: string): void => {
      const nextState = removeNode(storeApi.getState(), id);
      if (!nextState) return;
      storeApi.setState(nextState);
    },
    [storeApi],
  );
}

export { useRemoveNode };
