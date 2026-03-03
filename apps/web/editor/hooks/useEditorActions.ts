/**
 * Hook for dispatching editor actions.
 *
 * Returns all action functions from the editor store. Actions are
 * referentially stable (store methods don't change between renders).
 */

"use client";

import { useEditorStore } from "./useEditorStore";

function useEditorActions() {
  return useEditorStore((s) => ({
    loadRecipe: s.loadRecipe,
    createBlank: s.createBlank,
    addNode: s.addNode,
    removeNode: s.removeNode,
    selectNode: s.selectNode,
    updateParams: s.updateParams,
    moveNode: s.moveNode,
    undo: s.undo,
    redo: s.redo,
    resetDirty: s.resetDirty,
    setExecutionState: s.setExecutionState,
    resetExecution: s.resetExecution,
    setDefinition: s.setDefinition,
  }));
}

export { useEditorActions };
