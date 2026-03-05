/**
 * useUpdateParams — thin reactive wrapper around the updateParams action.
 *
 * The business logic (undo capture, param merge, revalidation)
 * lives in `actions/updateParams`. This hook just bridges it to the store.
 */

"use client";

import { useCallback } from "react";
import { useEditorStoreApi } from "./useEditorStoreApi";
import { updateParams } from "../actions/updateParams";

function useUpdateParams() {
  const storeApi = useEditorStoreApi();

  return useCallback(
    (nodeId: string, params: Record<string, unknown>): void => {
      const nextState = updateParams(storeApi.getState(), nodeId, params);
      if (!nextState) return;
      storeApi.setState(nextState);
    },
    [storeApi],
  );
}

export { useUpdateParams };
