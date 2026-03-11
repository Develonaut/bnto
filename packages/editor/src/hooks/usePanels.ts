/**
 * usePanels — standalone domain hook for panel visibility.
 *
 * Delegates to editor.panels.usePanels() for state and pre-bound actions.
 * Prefer editor.panels.usePanels() in new code.
 */

"use client";

import { useEditor } from "../context";
import type { PanelId } from "../store/types";

function usePanels(panelId: PanelId) {
  const editor = useEditor();
  return editor.panels.usePanels(panelId);
}

export { usePanels };
