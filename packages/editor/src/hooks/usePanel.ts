"use client";

import { useCallback } from "react";
import { useEditorStore } from "./useEditorStore";
import type { PanelId } from "../store/types";

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

export { usePanel };
