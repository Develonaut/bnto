/**
 * Panel service — wraps panel visibility actions + storeApi.setState().
 *
 * Thin wrapper: the store actions handle same-side mutual exclusivity
 * via panelHelpers internally.
 */

import type { StoreApi } from "zustand";
import type { EditorStore, PanelId } from "../store/types";
import type { PanelService } from "../editorTypes";

function createPanelService(storeApi: StoreApi<EditorStore>): PanelService {
  return {
    openPanel(id: PanelId) {
      storeApi.getState().openPanel(id);
    },

    closePanel(id: PanelId) {
      storeApi.getState().closePanel(id);
    },

    togglePanel(id: PanelId) {
      storeApi.getState().togglePanel(id);
    },
  };
}

export { createPanelService };
