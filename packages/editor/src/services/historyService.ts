/**
 * History service — wraps undo/redo actions + storeApi.setState().
 *
 * Thin wrapper: all logic lives in the store actions which use
 * captureSnapshot/pushToStack/revalidateState internally.
 */

import type { StoreApi } from "zustand";
import type { EditorStore } from "../store/types";
import type { HistoryService } from "../editorTypes";

function createHistoryService(storeApi: StoreApi<EditorStore>): HistoryService {
  return {
    pushUndo() {
      storeApi.getState().pushUndo();
    },

    undo() {
      storeApi.getState().undo();
    },

    redo() {
      storeApi.getState().redo();
    },

    resetHistory() {
      storeApi.getState().resetHistory();
    },
  };
}

export { createHistoryService };
