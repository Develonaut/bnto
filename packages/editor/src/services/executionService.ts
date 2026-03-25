/**
 * Execution service — wraps execution lifecycle actions + storeApi.setState().
 *
 * Covers: runExecution, resetRun, downloads, and per-node execution state.
 * The async runExecution delegates to the store action which orchestrates
 * the full pipeline (validation → execution → progress → results).
 */

import type { StoreApi } from "zustand";
import type { BrowserFileResult } from "@bnto/core";
import type { EditorStore, ExecutionState } from "../store/types";
import type { ExecutionService } from "../editorTypes";

function createExecutionService(storeApi: StoreApi<EditorStore>): ExecutionService {
  return {
    async runExecution(files: File[]) {
      await storeApi.getState().runExecution(files);
    },

    resetRun() {
      storeApi.getState().resetRun();
    },

    setInputFiles(files: File[]) {
      storeApi.getState().setInputFiles(files);
    },

    downloadResult(file: BrowserFileResult) {
      storeApi.getState().downloadResult(file);
    },

    async downloadAllResults() {
      await storeApi.getState().downloadAllResults();
    },

    setExecutionState(state: ExecutionState) {
      storeApi.getState().setExecutionState(state);
    },

    resetNodeStatuses() {
      storeApi.getState().resetNodeStatuses();
    },
  };
}

export { createExecutionService };
