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

import { useCallback, useEffect } from "react";
import { useKeyPress } from "@xyflow/react";
import { useKeyDown } from "@bnto/ui";
import { useEditor } from "../context";
import { resolveDelete, canTriggerRun, resolveEscape } from "../actions/shortcutActions";
import { downloadDefinition } from "../actions/downloadDefinition";

function useEditorShortcuts() {
  const editor = useEditor();

  // --- Undo: Cmd+Z / Ctrl+Z ---
  const undoPressed = useKeyPress(["Meta+z", "Control+z"]);
  useEffect(() => {
    if (undoPressed) editor.history.undo();
  }, [undoPressed, editor]);

  // --- Redo: Cmd+Shift+Z / Ctrl+Shift+Z ---
  const redoPressed = useKeyPress(["Meta+Shift+z", "Control+Shift+z"]);
  useEffect(() => {
    if (redoPressed) editor.history.redo();
  }, [redoPressed, editor]);

  // --- Delete selected node: Delete / Backspace ---
  const deletePressed = useKeyPress(["Backspace", "Delete"]);
  useEffect(() => {
    if (!deletePressed) return;
    const { selectedNodeId, configs } = editor.getState();
    const nodeType = selectedNodeId ? configs[selectedNodeId]?.nodeType : undefined;
    const nodeId = resolveDelete(selectedNodeId, nodeType ?? null);
    if (nodeId) editor.nodes.removeNode(nodeId);
  }, [deletePressed, editor]);

  // --- Run: Cmd+Enter / Ctrl+Enter ---
  const runPressed = useKeyPress(["Meta+Enter", "Control+Enter"]);
  useEffect(() => {
    if (!runPressed) return;
    const { executionPhase, executionInputFiles } = editor.getState();
    if (canTriggerRun(executionPhase, executionInputFiles.length)) {
      editor.execution.runExecution(executionInputFiles);
    }
  }, [runPressed, editor]);

  // --- Escape: deselect node + close panels ---
  const escapePressed = useKeyPress("Escape");
  useEffect(() => {
    if (!escapePressed) return;
    const { selectedNodeId, panels } = editor.getState();
    const result = resolveEscape(selectedNodeId, panels);
    if (result.closeConfig) editor.panels.closePanel("config");
    if (result.closeRun) editor.panels.closePanel("run");
    if (result.deselect) editor.nodes.selectNode(null);
  }, [escapePressed, editor]);

  // --- Cmd+S / Ctrl+S: block browser "Save Page As" ---
  // --- Export: Cmd+D / Ctrl+D (preventDefault to block browser Bookmark) ---
  // --- Help: Cmd+/ / Ctrl+/ (toggle help dialog via panel system) ---
  const handleDocumentKeys = useCallback(
    (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "s" && !e.shiftKey) {
        e.preventDefault();
      }
      if ((e.metaKey || e.ctrlKey) && e.key === "d" && !e.shiftKey) {
        e.preventDefault();
        downloadDefinition(editor.definition);
      }
      if ((e.metaKey || e.ctrlKey) && e.key === "/") {
        e.preventDefault();
        editor.panels.togglePanel("help");
      }
    },
    [editor],
  );
  useKeyDown(handleDocumentKeys);
}

export { useEditorShortcuts };
