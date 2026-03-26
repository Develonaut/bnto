/** Handle document-level keyboard shortcuts (Cmd+S, Cmd+D, Cmd+/). */

import type { ReactEditorInstance } from "../reactEditorTypes";
import { downloadDefinition } from "../actions/downloadDefinition";

function handleDocumentKey(e: KeyboardEvent, editor: ReactEditorInstance) {
  const mod = e.metaKey || e.ctrlKey;
  if (!mod) return;

  if (e.key === "s" && !e.shiftKey) {
    e.preventDefault();
  }
  if (e.key === "d" && !e.shiftKey) {
    e.preventDefault();
    downloadDefinition(editor.definition);
  }
  if (e.key === "/") {
    e.preventDefault();
    editor.panels.togglePanel("help");
  }
}

export { handleDocumentKey };
