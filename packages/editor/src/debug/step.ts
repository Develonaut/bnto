/**
 * step — advance debug execution by one logical stage.
 *
 * Call sequence: idle → running/all pending → node 1 active →
 * node 1 completed + node 2 active → ... → all completed.
 * Returns a short description of what just happened.
 */

import type { StoreApi } from "zustand";
import type { EditorStore, NodeExecutionStatus } from "../store/types";
import { isIoNodeType } from "@bnto/core";
import { preparePipeline, isPipelineError } from "../actions/runPipeline";
import type { RunPipelineError } from "../actions/runPipeline";

function step(store: StoreApi<EditorStore>): string {
  const { nodes, configs, executionPhase, executionState } = store.getState();

  const processingNodes = nodes.filter((n) => {
    const config = configs[n.id];
    return config && !isIoNodeType(config.nodeType);
  });

  if (processingNodes.length === 0) return "No processing nodes to step through.";
  if (executionPhase === "idle") return stepFromIdle(store);
  if (executionPhase === "running") return stepRunning(store, processingNodes, executionState);
  return `Already ${executionPhase}. Call reset() first.`;
}

/** idle → validate → running (or failed). */
function stepFromIdle(store: StoreApi<EditorStore>): string {
  const state = store.getState();
  const prepared = preparePipeline({
    nodes: state.nodes,
    configs: state.configs,
    recipeMetadata: state.recipeMetadata,
    definition: state.definition,
  });

  if (isPipelineError(prepared)) return applyValidationFailure(store, state, prepared);

  store.setState({
    executionPhase: "running",
    executionState: prepared.initialExecutionState,
    executionErrors: [],
    validationErrors: [],
  });
  return "Validated. Started: all nodes pending.";
}

/** Map validation errors to per-node failed states. */
function applyValidationFailure(
  store: StoreApi<EditorStore>,
  state: ReturnType<StoreApi<EditorStore>["getState"]>,
  prepared: RunPipelineError,
): string {
  const nodeStates: Record<string, NodeExecutionStatus> = {};
  for (const e of prepared.validationErrors) nodeStates[e.nodeId] = "failed";
  store.setState({
    executionPhase: "failed",
    executionErrors: prepared.errors,
    executionState: { ...state.executionState, ...nodeStates },
    validationErrors: prepared.validationErrors,
  });
  return `Validation failed: ${prepared.errors.join("; ")}`;
}

/** Advance one processing node: pending → active, or active → completed. */
function stepRunning(
  store: StoreApi<EditorStore>,
  processingNodes: { id: string }[],
  executionState: Record<string, NodeExecutionStatus>,
): string {
  const configs = store.getState().configs;
  const firstActive = processingNodes.find((n) => executionState[n.id] === "active");

  if (firstActive)
    return completeActiveNode(store, processingNodes, executionState, firstActive.id);

  const firstPending = processingNodes.find((n) => executionState[n.id] === "pending");
  if (firstPending) return activateNode(store, executionState, firstPending.id, configs);

  return "All nodes processed.";
}

/** Complete the active node and optionally activate the next pending one. */
function completeActiveNode(
  store: StoreApi<EditorStore>,
  processingNodes: { id: string }[],
  executionState: Record<string, NodeExecutionStatus>,
  activeId: string,
): string {
  const configs = store.getState().configs;
  const nextState = { ...executionState, [activeId]: "completed" as const };
  const label = configs[activeId]?.nodeType ?? activeId;

  const nextPending = processingNodes.find((n) => nextState[n.id] === "pending");
  if (nextPending) nextState[nextPending.id] = "active";

  const allDone = processingNodes.every((n) => nextState[n.id] === "completed");
  const percent = Math.round(
    (processingNodes.filter((n) => nextState[n.id] === "completed").length /
      processingNodes.length) *
      100,
  );
  const msg = allDone
    ? "Complete"
    : `Processing ${nextPending ? (configs[nextPending.id]?.nodeType ?? nextPending.id) : ""}...`;
  store.setState({
    executionState: nextState,
    executionPhase: allDone ? "completed" : "running",
    executionFileProgress: { fileIndex: 0, totalFiles: 1, overallPercent: percent, message: msg },
  });
  return allDone ? `Completed ${label}. All done!` : `Completed ${label}. (${percent}%)`;
}

/** Set a pending node to active. */
function activateNode(
  store: StoreApi<EditorStore>,
  executionState: Record<string, NodeExecutionStatus>,
  nodeId: string,
  configs: Record<string, { nodeType: string } | undefined>,
): string {
  const label = configs[nodeId]?.nodeType ?? nodeId;
  store.setState({
    executionState: { ...executionState, [nodeId]: "active" },
    executionFileProgress: {
      fileIndex: 0,
      totalFiles: 1,
      overallPercent: 0,
      message: `Processing ${label}...`,
    },
  });
  return `Activated ${label}.`;
}

export { step };
