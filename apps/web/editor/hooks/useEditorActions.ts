/**
 * Hook for dispatching editor actions.
 *
 * Composes RF-first mutation hooks (addNode, removeNode, updateParams)
 * with store-level actions (loadRecipe, undo, redo, etc.). The RF-first
 * hooks write to ReactFlow directly for instant visual feedback, then
 * delegate to the store for undo/validation/isDirty.
 *
 * Must be inside both EditorProvider and ReactFlowProvider.
 */

"use client";

import { useShallow } from "zustand/react/shallow";
import { useEditorStore } from "./useEditorStore";
import { useAddNode } from "./useAddNode";
import { useRemoveNode } from "./useRemoveNode";
import { useUpdateNodeParams } from "./useUpdateNodeParams";

function useEditorActions() {
  const addNode = useAddNode();
  const removeNode = useRemoveNode();
  const updateParams = useUpdateNodeParams();

  const storeActions = useEditorStore(
    useShallow((s) => ({
      loadRecipe: s.loadRecipe,
      createBlank: s.createBlank,
      undo: s.undo,
      redo: s.redo,
      resetDirty: s.resetDirty,
      setExecutionState: s.setExecutionState,
      resetExecution: s.resetExecution,
      setDefinition: s.setDefinition,
      setPositionGetter: s.setPositionGetter,
    })),
  );

  return {
    ...storeActions,
    addNode,
    removeNode,
    updateParams,
  };
}

export { useEditorActions };
