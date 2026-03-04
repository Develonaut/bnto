/**
 * RF-first node removal.
 *
 * Removes a node directly from ReactFlow by ID. The store is
 * only used for undo/dirty/validation.
 *
 * Must be inside both EditorProvider and ReactFlowProvider.
 */

"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import { useEditorStoreApi } from "./useEditorStoreApi";
import type { BentoNode } from "../adapters/types";

function useRemoveNode() {
  const storeApi = useEditorStoreApi();
  const { setNodes } = useReactFlow<BentoNode>();

  return useCallback(
    (id: string) => {
      const { pushUndo, markDirty, revalidate } = storeApi.getState();
      pushUndo();
      setNodes((prev) => prev.filter((n) => n.id !== id));
      markDirty();
      revalidate();
    },
    [storeApi, setNodes],
  );
}

export { useRemoveNode };
