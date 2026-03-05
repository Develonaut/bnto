/**
 * Hook for dispatching editor actions.
 *
 * Exposes store actions directly — all mutations live in the store.
 * useShallow compares each property individually so the wrapper object
 * doesn't cause re-renders (all function refs are referentially stable).
 *
 * Must be inside EditorProvider.
 */

"use client";

import { useShallow } from "zustand/react/shallow";
import { useEditorStore } from "./useEditorStore";

function useEditorActions() {
  return useEditorStore(
    useShallow((s) => ({
      loadRecipe: s.loadRecipe,
      createBlank: s.createBlank,
      addNode: s.addNode,
      removeNode: s.removeNode,
      selectNode: s.selectNode,
      updateParams: s.updateConfigParams,
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
}

export { useEditorActions };
