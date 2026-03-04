/**
 * Undo/redo with direct RF node restoration.
 *
 * Undo/redo return BentoNode[] snapshots from the store.
 * This hook applies them directly to RF via setNodes.
 *
 * Must be inside both EditorProvider and ReactFlowProvider.
 */

"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import { useEditorStore } from "./useEditorStore";
import type { BentoNode } from "../adapters/types";

interface EditorUndoRedoResult {
  undo: () => void;
  redo: () => void;
  canUndo: boolean;
  canRedo: boolean;
}

function useEditorUndoRedo(): EditorUndoRedoResult {
  const storeUndo = useEditorStore((s) => s.undo);
  const storeRedo = useEditorStore((s) => s.redo);
  const canUndo = useEditorStore((s) => s.undoStack.length > 0);
  const canRedo = useEditorStore((s) => s.redoStack.length > 0);
  const { setNodes } = useReactFlow<BentoNode>();

  const undo = useCallback(() => {
    const snapshot = storeUndo();
    if (snapshot) setNodes(snapshot);
  }, [storeUndo, setNodes]);

  const redo = useCallback(() => {
    const snapshot = storeRedo();
    if (snapshot) setNodes(snapshot);
  }, [storeRedo, setNodes]);

  return { undo, redo, canUndo, canRedo };
}

export { useEditorUndoRedo };
export type { EditorUndoRedoResult };
