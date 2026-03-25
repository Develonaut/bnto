/**
 * Pure debug helper functions for console-driven editor state manipulation.
 *
 * Each function takes a StoreApi<EditorStore> and performs a targeted
 * mutation. Used by the debug API facade (registerEditorDebug) to
 * provide ergonomic shortcuts on window.__bnto__.editor.
 */

import type { StoreApi } from "zustand";
import type { EditorStore, ExecutionPhase, NodeExecutionStatus } from "../store/types";
import type { ValidationError } from "@bnto/core";
import { createBlank } from "../actions/createBlank";

function setPhase(store: StoreApi<EditorStore>, phase: ExecutionPhase): void {
  store.setState({ executionPhase: phase });
}

function setNodeStatus(
  store: StoreApi<EditorStore>,
  nodeId: string,
  status: NodeExecutionStatus,
): void {
  const current = store.getState().executionState;
  store.setState({ executionState: { ...current, [nodeId]: status } });
}

function setAllNodeStatuses(store: StoreApi<EditorStore>, status: NodeExecutionStatus): void {
  const nodes = store.getState().nodes;
  const next: Record<string, NodeExecutionStatus> = {};
  for (const node of nodes) {
    next[node.id] = status;
  }
  store.setState({ executionState: next });
}

function simulateProgress(store: StoreApi<EditorStore>, percent: number, message?: string): void {
  const clamped = Math.max(0, Math.min(100, percent));
  store.setState({
    executionFileProgress: {
      fileIndex: 0,
      totalFiles: 1,
      overallPercent: clamped,
      message: message ?? `Debug progress: ${clamped}%`,
    },
  });
}

function forceError(store: StoreApi<EditorStore>, message: string): void {
  const current = store.getState().executionErrors;
  store.setState({
    executionPhase: "failed",
    executionErrors: [...current, message],
  });
}

function forceComplete(store: StoreApi<EditorStore>): void {
  const nodes = store.getState().nodes;
  const completedState: Record<string, NodeExecutionStatus> = {};
  for (const node of nodes) {
    completedState[node.id] = "completed";
  }
  store.setState({
    executionPhase: "completed",
    executionState: completedState,
    executionErrors: [],
  });
}

function setDirty(store: StoreApi<EditorStore>, dirty: boolean): void {
  store.setState({ isDirty: dirty });
}

function clearValidation(store: StoreApi<EditorStore>): void {
  store.setState({ validationErrors: [] });
}

function injectValidationError(store: StoreApi<EditorStore>, error: ValidationError): void {
  const current = store.getState().validationErrors;
  store.setState({ validationErrors: [...current, error] });
}

function reset(store: StoreApi<EditorStore>): void {
  store.setState(createBlank());
}

/** Clear execution state but keep the current recipe and nodes. */
function clear(store: StoreApi<EditorStore>): void {
  store.setState({
    executionPhase: "idle",
    executionState: {},
    executionErrors: [],
    executionFileProgress: null,
    executionResults: [],
    executionLogs: [],
    nodeProgress: {},
  });
}

export {
  setPhase,
  setNodeStatus,
  setAllNodeStatuses,
  simulateProgress,
  forceError,
  forceComplete,
  setDirty,
  clearValidation,
  injectValidationError,
  reset,
  clear,
};
export { step } from "./step";
export { run } from "./run";
