"use client";

import { useShallow } from "@bnto/core";
import { useEditorStore } from "./useEditorStore";

/**
 * useEditorPanels — centralized panel visibility from the editor store.
 *
 * All panels are identified by PanelId. Consumers read `panels[id]`
 * for visibility and call `openPanel`, `closePanel`, `togglePanel`
 * with the panel ID. No per-panel booleans or per-panel toggle methods.
 */
function useEditorPanels() {
  const panels = useEditorStore(useShallow((s) => s.panels));
  const openPanel = useEditorStore((s) => s.openPanel);
  const closePanel = useEditorStore((s) => s.closePanel);
  const togglePanel = useEditorStore((s) => s.togglePanel);

  return { panels, openPanel, closePanel, togglePanel };
}

export { useEditorPanels };
