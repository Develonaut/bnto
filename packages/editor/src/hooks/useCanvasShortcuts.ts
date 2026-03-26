/**
 * useCanvasShortcuts — ReactFlow key-press shortcuts for undo, redo, delete, run, escape.
 *
 * Extracted from useEditorShortcuts to keep each hook under the line limit.
 */

"use client";

import { useEffect } from "react";
import { useKeyPress } from "@xyflow/react";
import type { ReactEditorInstance } from "../reactEditorTypes";
import { resolveDelete, canTriggerRun, resolveEscape } from "../actions/shortcutActions";

function useCanvasShortcuts(editor: ReactEditorInstance) {
  useUndoRedo(editor);
  useDeleteShortcut(editor);
  useRunShortcut(editor);
  useEscapeShortcut(editor);
}

function useUndoRedo(editor: ReactEditorInstance) {
  const undoPressed = useKeyPress(["Meta+z", "Control+z"]);
  useEffect(() => {
    if (undoPressed) editor.history.undo();
  }, [undoPressed, editor]);

  const redoPressed = useKeyPress(["Meta+Shift+z", "Control+Shift+z"]);
  useEffect(() => {
    if (redoPressed) editor.history.redo();
  }, [redoPressed, editor]);
}

function useDeleteShortcut(editor: ReactEditorInstance) {
  const deletePressed = useKeyPress(["Backspace", "Delete"]);
  useEffect(() => {
    if (!deletePressed) return;
    const { selectedNodeId, configs } = editor.getState();
    const nodeType = selectedNodeId ? configs[selectedNodeId]?.nodeType : undefined;
    const nodeId = resolveDelete(selectedNodeId, nodeType ?? null);
    if (nodeId) editor.nodes.removeNode(nodeId);
  }, [deletePressed, editor]);
}

function useRunShortcut(editor: ReactEditorInstance) {
  const runPressed = useKeyPress(["Meta+Enter", "Control+Enter"]);
  useEffect(() => {
    if (!runPressed) return;
    const { executionPhase, executionInputFiles } = editor.getState();
    if (canTriggerRun(executionPhase, executionInputFiles.length)) {
      editor.execution.runExecution(executionInputFiles);
    }
  }, [runPressed, editor]);
}

function useEscapeShortcut(editor: ReactEditorInstance) {
  const escapePressed = useKeyPress("Escape");
  useEffect(() => {
    if (!escapePressed) return;
    const { selectedNodeId, panels } = editor.getState();
    const result = resolveEscape(selectedNodeId, panels);
    if (result.closeConfig) editor.panels.closePanel("config");
    if (result.closeRun) editor.panels.closePanel("run");
    if (result.deselect) editor.nodes.selectNode(null);
  }, [escapePressed, editor]);
}

export { useCanvasShortcuts };
