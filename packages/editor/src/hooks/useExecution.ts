/**
 * useExecution — standalone domain hook for execution lifecycle state + actions.
 *
 * Delegates to editor.execution.useExecution() for state. Actions come
 * directly from the editor instance. Prefer editor.execution.useExecution()
 * in new code.
 */

"use client";

import { useEditor } from "../context";

function useExecution() {
  const editor = useEditor();
  const state = editor.execution.useExecution();

  return {
    ...state,
    run: editor.execution.runExecution,
    reset: editor.execution.resetRun,
    downloadFile: editor.execution.downloadResult,
    downloadAll: editor.execution.downloadAllResults,
  };
}

export { useExecution };
