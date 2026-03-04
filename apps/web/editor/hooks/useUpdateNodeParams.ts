/**
 * RF-first parameter update.
 *
 * Updates a node's parameters in the ReactFlow data field directly,
 * then updates the editor store for undo/validation/isDirty.
 *
 * Must be inside both EditorProvider and ReactFlowProvider.
 */

"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import type { BentoNode } from "../adapters/types";
import { useEditorStoreApi } from "./useEditorStoreApi";

function useUpdateNodeParams() {
  const { setNodes } = useReactFlow<BentoNode>();
  const storeApi = useEditorStoreApi();

  return useCallback(
    (nodeId: string, params: Record<string, unknown>) => {
      // 1. Update RF node data directly — instant visual feedback
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

      // 2. Delegate to store (handles definition update, undo, validation, isDirty)
      storeApi.getState().updateParams(nodeId, params);
    },
    [setNodes, storeApi],
  );
}

export { useUpdateNodeParams };
