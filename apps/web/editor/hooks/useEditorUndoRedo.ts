/**
 * Undo/redo hook — simplified for controlled mode.
 *
 * The store owns everything (nodes + configs). Undo/redo restore
 * both atomically — no need to call setNodes() externally.
 *
 * Must be inside an EditorProvider.
 */

"use client";

import { useEditorStore } from "./useEditorStore";

interface EditorUndoRedoResult {
  undo: () => void;
  redo: () => void;
  canUndo: boolean;
  canRedo: boolean;
}

function useEditorUndoRedo(): EditorUndoRedoResult {
  const undo = useEditorStore((s) => s.undo);
  const redo = useEditorStore((s) => s.redo);
  const canUndo = useEditorStore((s) => s.undoStack.length > 0);
  const canRedo = useEditorStore((s) => s.redoStack.length > 0);

  return { undo, redo, canUndo, canRedo };
}

export { useEditorUndoRedo };
export type { EditorUndoRedoResult };
