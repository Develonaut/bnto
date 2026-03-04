/**
 * RF-first parameter update.
 *
 * Updates a node's parameters directly in ReactFlow. The store
 * is only used for undo/dirty/validation.
 *
 * Must be inside both EditorProvider and ReactFlowProvider.
 */

"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import { useEditorStoreApi } from "./useEditorStoreApi";
import type { BentoNode } from "../adapters/types";

function useUpdateNodeParams() {
  const storeApi = useEditorStoreApi();
  const { setNodes } = useReactFlow<BentoNode>();

  return useCallback(
    (nodeId: string, params: Record<string, unknown>) => {
      const { pushUndo, markDirty, revalidate } = storeApi.getState();
      pushUndo();
      setNodes((prev) =>
        prev.map((n) =>
          n.id === nodeId
            ? {
                ...n,
                data: {
                  ...n.data,
                  parameters: { ...n.data.parameters, ...params },
                },
              }
            : n,
        ),
      );
      markDirty();
      revalidate();
    },
    [storeApi, setNodes],
  );
}

export { useUpdateNodeParams };
