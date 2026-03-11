/**
 * useHistory — standalone domain hook for undo/redo state + actions.
 *
 * Delegates to editor.history.useHistory() for state. Actions come
 * directly from the editor instance. Prefer editor.history.useHistory()
 * in new code.
 */

"use client";

import { useEditor } from "../context";

function useHistory() {
  const editor = useEditor();
  const state = editor.history.useHistory();

  return {
    ...state,
    undo: editor.history.undo,
    redo: editor.history.redo,
  };
}

export { useHistory };
