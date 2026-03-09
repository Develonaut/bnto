/**
 * executionState — pure functions for mapping pipeline lifecycle to
 * per-node execution state in the editor store.
 *
 * Extracted from useEditorExecution so the state transitions are
 * testable without React or store dependencies.
 */

import type { PipelineEvent, PipelineDefinition } from "@bnto/core";
import type { ExecutionState, NodeExecutionStatus } from "../store/types";

/**
 * Map a pipeline event to the next execution state.
 *
 * Only NodeStarted, NodeCompleted, and NodeFailed change state.
 * Other events (FileProgress, PipelineStarted/Completed) are
 * informational and don't affect node status.
 */
function applyPipelineEvent(current: ExecutionState, event: PipelineEvent): ExecutionState {
  switch (event.type) {
    case "NodeStarted":
      return { ...current, [event.nodeId]: "active" };
    case "NodeCompleted":
      return { ...current, [event.nodeId]: "completed" };
    case "NodeFailed":
      return { ...current, [event.nodeId]: "failed" };
    default:
      return current;
  }
}

/**
 * Build the pending state before execution starts.
 *
 * All nodes (including I/O) → "pending".
 */
function buildPendingState(definition: PipelineDefinition, base: ExecutionState): ExecutionState {
  const state: ExecutionState = { ...base };
  for (const node of definition.nodes) {
    state[node.id] = "pending";
  }
  return state;
}

/**
 * Build the final state after successful execution.
 *
 * All nodes (including I/O) → "completed".
 */
function buildFinalState(definition: PipelineDefinition): ExecutionState {
  const state: ExecutionState = {};
  for (const node of definition.nodes) {
    state[node.id] = "completed";
  }
  return state;
}

/**
 * Build the failed state after an execution error.
 *
 * Active nodes → "failed", pending nodes → "idle".
 * Already completed/failed nodes keep their status.
 */
function buildFailedState(current: ExecutionState): ExecutionState {
  const state: ExecutionState = { ...current };
  const transitions: Record<string, NodeExecutionStatus> = {
    active: "failed",
    pending: "idle",
  };
  for (const [nodeId, status] of Object.entries(state)) {
    const next = transitions[status];
    if (next) state[nodeId] = next;
  }
  return state;
}

export { applyPipelineEvent, buildPendingState, buildFinalState, buildFailedState };
