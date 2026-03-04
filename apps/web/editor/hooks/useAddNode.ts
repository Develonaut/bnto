/**
 * RF-first node addition.
 *
 * Creates a BentoNode directly via createCompartmentNode and adds
 * it to ReactFlow. The store is only used for undo/dirty/validation.
 *
 * Must be inside both EditorProvider and ReactFlowProvider.
 */

"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import type { NodeTypeName } from "@bnto/nodes";
import { useEditorStoreApi } from "./useEditorStoreApi";
import { createCompartmentNode } from "../adapters/createCompartmentNode";
import type { BentoNode } from "../adapters/types";

function useAddNode() {
  const storeApi = useEditorStoreApi();
  const { getNodes, setNodes } = useReactFlow<BentoNode>();

  return useCallback(
    (type: NodeTypeName, position?: { x: number; y: number }): string | null => {
      const slotIndex = getNodes().length;
      const node = createCompartmentNode(type, slotIndex, position);
      if (!node) return null;

      const { pushUndo, markDirty, revalidate } = storeApi.getState();
      pushUndo();
      setNodes((prev) => [...prev, node]);
      markDirty();
      revalidate();
      return node.id;
    },
    [storeApi, getNodes, setNodes],
  );
}

export { useAddNode };
