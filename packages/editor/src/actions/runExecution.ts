/**
 * runExecution — async action that orchestrates pipeline execution.
 *
 * Extracted from createEditorStore to keep the store factory under
 * the 250-line cap and follow the Actions Pattern.
 */

import { core } from "@bnto/core";
import type { PipelineEvent } from "@bnto/core";
import type { EditorStore } from "../store/types";
import { preparePipeline, isPipelineError } from "./runPipeline";
import type { RunPipelineResult } from "./runPipeline";
import {
  applyPipelineEvent,
  buildPendingState,
  buildFinalState,
  buildFailedState,
} from "./executionState";
import { logEventToConsole } from "./logEventToConsole";

type SetState = (
  partial: Partial<EditorStore> | ((s: EditorStore) => Partial<EditorStore>),
) => void;
type GetState = () => EditorStore;

async function runExecution(set: SetState, get: GetState, files: File[]): Promise<void> {
  const prepared = preparePipeline({
    nodes: get().nodes,
    configs: get().configs,
    recipeMetadata: get().recipeMetadata,
    definition: get().definition,
  });

  if (isPipelineError(prepared)) {
    handleValidationErrors(set, get, prepared);
    return;
  }

  initRunState(set, files);
  get().openPanel("run");

  try {
    await executePipeline(set, get, prepared, files);
  } catch (err) {
    handlePipelineError(set, err);
  }
}

/** Set state for validation errors (pre-run failure). */
function handleValidationErrors(
  set: SetState,
  get: GetState,
  prepared: {
    validationErrors: { nodeId: string; field: string; message: string }[];
    errors: string[];
  },
): void {
  const failedNodeIds = new Set(prepared.validationErrors.map((e) => e.nodeId));
  const nodeStates: Record<string, "failed"> = {};
  for (const nodeId of failedNodeIds) nodeStates[nodeId] = "failed";

  const now = Date.now();
  const validationLogs = prepared.validationErrors.map((e) => ({
    timestamp: now,
    event: {
      type: "ValidationFailed" as const,
      nodeId: e.nodeId,
      field: e.field,
      error: e.message,
    },
  }));
  for (const log of validationLogs) logEventToConsole(log.event);

  set({
    executionErrors: prepared.errors,
    executionPhase: "failed",
    executionState: { ...get().executionState, ...nodeStates },
    executionLogs: validationLogs,
  });
}

/** Reset store to running state before pipeline starts. */
function initRunState(set: SetState, files: File[]): void {
  set({
    executionState: {},
    nodeProgress: {},
    executionPhase: "running",
    executionErrors: [],
    executionResults: [],
    executionLogs: [],
    executionFileProgress: null,
    executionInputFiles: files,
  });
}

/** Run the pipeline and update state on progress and completion. */
async function executePipeline(
  set: SetState,
  get: GetState,
  prepared: RunPipelineResult,
  files: File[],
): Promise<void> {
  set({ executionState: buildPendingState(prepared.definition, prepared.initialExecutionState) });

  const onEvent = (event: PipelineEvent) => {
    logEventToConsole(event);
    set((s) => ({ executionLogs: [...s.executionLogs, { timestamp: Date.now(), event }] }));
    applyEventToState(set, get, event);
  };

  const browserResults = await core.executions.runPipeline(
    prepared.definition,
    files,
    undefined,
    onEvent,
  );
  set({
    executionState: buildFinalState(prepared.definition),
    executionResults: browserResults,
    executionPhase: "completed",
  });

  if (browserResults.length > 0 && shouldAutoDownload(get())) {
    core.executions.downloadAllResults(browserResults, "editor-results").catch(() => {});
  }
}

/** Apply a single pipeline event to execution state. */
function applyEventToState(set: SetState, get: GetState, event: PipelineEvent): void {
  const next = applyPipelineEvent(get().executionState, event);
  if (next !== get().executionState) set({ executionState: next });

  if (event.type === "NodeStarted") {
    set((s) => ({ nodeProgress: { ...s.nodeProgress, [event.nodeId]: 0 } }));
  } else if (event.type === "NodeCompleted") {
    set((s) => ({ nodeProgress: { ...s.nodeProgress, [event.nodeId]: 100 } }));
  }

  if (event.type === "FileProgress") {
    set((s) => ({ nodeProgress: { ...s.nodeProgress, [event.nodeId]: event.percent } }));
    const overallPercent = Math.round(
      ((event.fileIndex + event.percent / 100) / event.totalFiles) * 100,
    );
    set({
      executionFileProgress: {
        fileIndex: event.fileIndex,
        totalFiles: event.totalFiles,
        overallPercent,
        message: event.message,
      },
    });
  }
}

/** Handle a caught pipeline error. */
function handlePipelineError(set: SetState, err: unknown): void {
  const message = err instanceof Error ? err.message : "Pipeline execution failed";
  set((s) => ({
    executionState: buildFailedState(s.executionState),
    executionErrors: [message],
    executionPhase: "failed",
    executionLogs: [
      ...s.executionLogs,
      { timestamp: Date.now(), event: { type: "PipelineFailed", nodeId: "", error: message } },
    ],
  }));
}

/** Check the output node's autoDownload param (defaults to true). */
function shouldAutoDownload(state: EditorStore): boolean {
  const outputEntry = Object.entries(state.configs).find(
    ([, config]) => config.nodeType === "output",
  );
  if (!outputEntry) return true;
  return outputEntry[1].parameters.autoDownload !== false;
}

export { runExecution, shouldAutoDownload };
