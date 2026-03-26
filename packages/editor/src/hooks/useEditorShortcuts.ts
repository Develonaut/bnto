/**
 * useEditorShortcuts — keyboard shortcuts for the editor canvas.
 *
 * Uses ReactFlow's `useKeyPress` for canvas-scoped shortcuts (undo, redo,
 * delete, run) and `useKeyDown` from `@bnto/ui` for shortcuts that must
 * preventDefault at document level (Cmd+S blocks browser save, Cmd+D, Cmd+/).
 *
 * Input field safety: `useKeyPress` has built-in input exclusion —
 * single keys (Delete/Backspace) are suppressed when an input is focused.
 * Modifier combos (Cmd+Z) fire regardless of focus target.
 *
 * Guard/dispatch logic lives in pure action functions (shortcutActions.ts)
 * so it can be unit tested without React or ReactFlow context.
 *
 * Must be rendered inside a ReactFlowProvider + EditorProvider.
 */

"use client";

import { useCallback } from "react";
import { useKeyDown } from "@bnto/ui";
import { useEditor } from "../context";
import { useCanvasShortcuts } from "./useCanvasShortcuts";
import { handleDocumentKey } from "./handleDocumentKey";

function useEditorShortcuts() {
  const editor = useEditor();

  useCanvasShortcuts(editor);

  const handleDocumentKeys = useCallback(
    (e: KeyboardEvent) => handleDocumentKey(e, editor),
    [editor],
  );
  useKeyDown(handleDocumentKeys);
}

export { useEditorShortcuts };
