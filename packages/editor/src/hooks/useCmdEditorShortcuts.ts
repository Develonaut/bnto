/**
 * useCmdEditorShortcuts — document-level keyboard shortcuts for CmdEditor.
 *
 * Replaces the RF-dependent useEditorShortcuts with a version using
 * useKeyDown from @bnto/ui. Same shortcut set, no ReactFlow dependency.
 *
 * Input safety: single-key shortcuts (Delete/Backspace) are skipped
 * when an input, textarea, or contenteditable element is focused.
 */

"use client";

import { useCallback } from "react";
import { useKeyDown } from "@bnto/ui";
import { useEditor } from "../context";
import { resolveDelete, canTriggerRun } from "../actions/shortcutActions";
import { downloadDefinition } from "../actions/downloadDefinition";

/** Returns true if the active element is a text input (should swallow single keys). */
function isInputFocused(): boolean {
  const el = document.activeElement;
  if (!el) return false;
  const tag = el.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA") return true;
  if ((el as HTMLElement).isContentEditable) return true;
  return false;
}

function useCmdEditorShortcuts() {
  const editor = useEditor();

  const handler = useCallback(
    (e: KeyboardEvent) => {
      const mod = e.metaKey || e.ctrlKey;

      // Cmd+Z → undo
      if (mod && e.key === "z" && !e.shiftKey) {
        e.preventDefault();
        editor.history.undo();
        return;
      }

      // Cmd+Shift+Z → redo
      if (mod && e.key === "z" && e.shiftKey) {
        e.preventDefault();
        editor.history.redo();
        return;
      }

      // Cmd+Enter → run
      if (mod && e.key === "Enter") {
        e.preventDefault();
        const { executionPhase, executionInputFiles } = editor.getState();
        if (canTriggerRun(executionPhase, executionInputFiles.length)) {
          editor.execution.runExecution(executionInputFiles);
        }
        return;
      }

      // Cmd+D → export definition
      if (mod && e.key === "d" && !e.shiftKey) {
        e.preventDefault();
        downloadDefinition(editor.definition);
        return;
      }

      // Cmd+S → block browser save
      if (mod && e.key === "s" && !e.shiftKey) {
        e.preventDefault();
        return;
      }

      // Cmd+/ → toggle help
      if (mod && e.key === "/") {
        e.preventDefault();
        editor.panels.togglePanel("help");
        return;
      }

      // Delete/Backspace → delete selected node (only when not in a text input)
      if ((e.key === "Delete" || e.key === "Backspace") && !mod && !e.shiftKey) {
        if (isInputFocused()) return;
        const { selectedNodeId, configs } = editor.getState();
        const nodeType = selectedNodeId ? (configs[selectedNodeId]?.nodeType ?? null) : null;
        const target = resolveDelete(selectedNodeId, nodeType);
        if (target) {
          e.preventDefault();
          editor.nodes.removeNode(target);
        }
      }
    },
    [editor],
  );

  useKeyDown(handler);
}

export { useCmdEditorShortcuts };
