/**
 * Undo/redo with RF position restoration.
 *
 * When a snapshot is returned from the store, applies the saved
 * position map back to ReactFlow nodes via setNodes.
 *
 * Must be inside both EditorProvider and ReactFlowProvider.
 */

"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import { useEditorStore } from "./useEditorStore";
import type { UndoSnapshot } from "../store/types";

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
  const { setNodes } = useReactFlow();

  const applyPositions = useCallback(
    (snapshot: UndoSnapshot) => {
      const positions = snapshot.positions;
      if (Object.keys(positions).length === 0) return;
      setNodes((prev) =>
        prev.map((node) => {
          const pos = positions[node.id];
          return pos ? { ...node, position: pos } : node;
        }),
      );
    },
    [setNodes],
  );

  const undo = useCallback(() => {
    const snapshot = storeUndo();
    if (snapshot) applyPositions(snapshot);
  }, [storeUndo, applyPositions]);

  const redo = useCallback(() => {
    const snapshot = storeRedo();
    if (snapshot) applyPositions(snapshot);
  }, [storeRedo, applyPositions]);

  return { undo, redo, canUndo, canRedo };
}

export { useEditorUndoRedo };
export type { EditorUndoRedoResult };
