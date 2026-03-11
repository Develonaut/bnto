/**
 * createUsePanels — hook factory for panel visibility.
 *
 * Returns a usePanels hook closure that captures storeApi at creation time.
 * Takes a panelId and returns pre-bound open/close/toggle actions.
 */

"use client";

import { useCallback } from "react";
import type { StoreApi } from "zustand";
import { useStore } from "zustand";
import type { EditorStore, PanelId } from "../../store/types";
import type { PanelsHookResult } from "../../reactEditorTypes";

function createUsePanels(
  storeApi: StoreApi<EditorStore>,
  panelActions: { openPanel: (id: PanelId) => void; closePanel: (id: PanelId) => void; togglePanel: (id: PanelId) => void },
) {
  return function usePanels(panelId: PanelId): PanelsHookResult {
    const isOpen = useStore(storeApi, (s) => s.panels[panelId]);

    const open = useCallback(() => panelActions.openPanel(panelId), [panelId]);
    const close = useCallback(() => panelActions.closePanel(panelId), [panelId]);
    const toggle = useCallback(() => panelActions.togglePanel(panelId), [panelId]);

    return { isOpen, open, close, toggle };
  };
}

export { createUsePanels };
