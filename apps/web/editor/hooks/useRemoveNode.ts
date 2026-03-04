/**
 * RF-first node removal.
 *
 * Removes a node from the ReactFlow canvas directly, then updates
 * the editor store for undo/validation/isDirty.
 *
 * Must be inside both EditorProvider and ReactFlowProvider.
 */

"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import type { BentoNode } from "../adapters/types";
import { useEditorStoreApi } from "./useEditorStoreApi";

function useRemoveNode() {
  const { setNodes } = useReactFlow<BentoNode>();
  const storeApi = useEditorStoreApi();

  return useCallback(
    (id: string) => {
      // 1. Remove from RF directly — instant visual feedback
      setNodes((prev) => prev.filter((n) => n.id !== id));

      // 2. Delegate to store (handles definition update, undo, validation, isDirty)
      storeApi.getState().removeNode(id);
    },
    [setNodes, storeApi],
  );
}

export { useRemoveNode };
