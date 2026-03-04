/**
 * RF-first node addition.
 *
 * Adds a node to the ReactFlow canvas directly, then updates the
 * editor store for undo/validation/isDirty. The sync effects in
 * useDefinitionSync will no-op because RF already has the node
 * (rfIds guard).
 *
 * Must be inside both EditorProvider and ReactFlowProvider.
 */

"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import type { NodeTypeName, Position } from "@bnto/nodes";
import { definitionNodeToRfNode } from "../adapters/definitionNodeToRfNode";
import type { BentoNode } from "../adapters/types";
import { useEditorStoreApi } from "./useEditorStoreApi";

function useAddNode() {
  const { setNodes } = useReactFlow<BentoNode>();
  const storeApi = useEditorStoreApi();

  return useCallback(
    (type: NodeTypeName, position?: Position): string | null => {
      // 1. Delegate to store (handles definition update, undo, validation, isDirty)
      const newId = storeApi.getState().addNode(type, position);
      if (!newId) return null;

      // 2. Read the updated definition to get the new node
      const updatedDef = storeApi.getState().definition;
      const newNode = updatedDef.nodes?.find((n) => n.id === newId);
      if (!newNode) return newId;

      // 3. Convert to RF node at the last slot index
      const nodeIndex = (updatedDef.nodes?.length ?? 1) - 1;
      const rfNode = definitionNodeToRfNode(newNode, nodeIndex);
      if (!rfNode) return newId;

      // 4. Add to RF directly — instant visual feedback
      setNodes((prev) => [...prev, rfNode as BentoNode]);

      return newId;
    },
    [setNodes, storeApi],
  );
}

export { useAddNode };
