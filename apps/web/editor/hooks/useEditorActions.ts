/**
 * useEditorActions — composed editor action API.
 *
 * Composes action hooks (addNode, removeNode, updateParams) with
 * store actions (selectNode, undo/redo, entry points, utility).
 *
 * This is the primary action hook for components — they import
 * useEditorActions instead of individual hooks or store selectors.
 *
 * Must be inside EditorProvider + ReactFlowProvider.
 */

"use client";

import { useShallow } from "zustand/react/shallow";
import { useEditorStore } from "./useEditorStore";
import { useAddNode } from "./useAddNode";
import { useRemoveNode } from "./useRemoveNode";
import { useUpdateParams } from "./useUpdateParams";

function useEditorActions() {
  const addNode = useAddNode();
  const removeNode = useRemoveNode();
  const updateParams = useUpdateParams();

  const storeActions = useEditorStore(
    useShallow((s) => ({
      loadRecipe: s.loadRecipe,
      createBlank: s.createBlank,
      selectNode: s.selectNode,
      undo: s.undo,
      redo: s.redo,
      pushUndo: s.pushUndo,
      markDirty: s.markDirty,
      revalidate: s.revalidate,
      resetDirty: s.resetDirty,
      setExecutionState: s.setExecutionState,
      resetExecution: s.resetExecution,
      setRecipeMetadata: s.setRecipeMetadata,
      resetHistory: s.resetHistory,
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
