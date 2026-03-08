"use client";

import { useCallback } from "react";
import { useShallow } from "@bnto/core";
import { useEditorStore } from "./useEditorStore";
import type { PanelId } from "../store/types";

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

/**
 * usePanel — convenience hook for a single panel's open state + toggle.
 *
 * Avoids boilerplate in triggers and panel components that only care
 * about one panel. Returns `{ isOpen, open, close, toggle }`.
 */
function usePanel(id: PanelId) {
  const isOpen = useEditorStore((s) => s.panels[id]);
  const openPanel = useEditorStore((s) => s.openPanel);
  const closePanel = useEditorStore((s) => s.closePanel);
  const togglePanel = useEditorStore((s) => s.togglePanel);

  const open = useCallback(() => openPanel(id), [openPanel, id]);
  const close = useCallback(() => closePanel(id), [closePanel, id]);
  const toggle = useCallback(() => togglePanel(id), [togglePanel, id]);

  return { isOpen, open, close, toggle };
}

export { useEditorPanels, usePanel };
