/**
 * Execution service — wraps execution lifecycle actions + storeApi.setState().
 *
 * Covers: runExecution, resetRun, downloads, and per-node execution state.
 * The async runExecution delegates to the store action which orchestrates
 * the full pipeline (validation → execution → progress → results).
 */

import type { StoreApi } from "zustand";
import type { BrowserFileResult } from "@bnto/core";
import type { EditorStore, EditorState, ExecutionState, NodeExecutionStatus } from "../store/types";
import type { ExecutionService } from "../editorTypes";

function createExecutionService(storeApi: StoreApi<EditorStore>): ExecutionService {
  return {
    async runExecution(files: File[]) {
      await storeApi.getState().runExecution(files);
    },

    resetRun() {
      storeApi.getState().resetRun();
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

    setNodeStatus(nodeId: string, status: NodeExecutionStatus) {
      storeApi.setState((s) => ({
        executionState: { ...s.executionState, [nodeId]: status },
      }));
    },

    setNodeProgress(nodeId: string, percent: number) {
      storeApi.setState((s) => ({
        nodeProgress: { ...s.nodeProgress, [nodeId]: percent },
      }));
    },

    forceExecutionState(partial: Partial<EditorState>) {
      storeApi.setState(partial);
    },
  };
}

export { createExecutionService };
