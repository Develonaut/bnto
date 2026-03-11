/**
 * createUseHistory — hook factory for undo/redo state.
 *
 * Returns a useHistory hook closure that captures storeApi at creation time.
 * Derives canUndo/canRedo from stack lengths.
 */

"use client";

import type { StoreApi } from "zustand";
import { useStore } from "zustand";
import type { EditorStore } from "../../store/types";
import type { HistoryHookResult } from "../../reactEditorTypes";

function createUseHistory(storeApi: StoreApi<EditorStore>) {
  return function useHistory(): HistoryHookResult {
    const canUndo = useStore(storeApi, (s) => s.undoStack.length > 0);
    const canRedo = useStore(storeApi, (s) => s.redoStack.length > 0);

    return { canUndo, canRedo };
  };
}

export { createUseHistory };
