/**
 * runExecution — async action that orchestrates pipeline execution.
 *
 * Extracted from createEditorStore to keep the store factory under
 * the 250-line cap and follow the Actions Pattern: complex logic
 * lives in action files, the store action is a one-line wrapper.
 *
 * This is NOT a pure state transform (it calls core.executions.runPipeline),
 * but it follows the same extraction pattern: store stays thin, logic is
 * co-located with other actions, and the async flow is testable in isolation.
 */

import { core } from "@bnto/core";
import type { PipelineEvent } from "@bnto/core";
import type { EditorStore } from "../store/types";
import { preparePipeline, isPipelineError } from "./runPipeline";
import {
  applyPipelineEvent,
  buildPendingState,
  buildFinalState,
  buildFailedState,
} from "./executionState";

// ---------------------------------------------------------------------------
// Types — match Zustand factory set/get signatures
// ---------------------------------------------------------------------------

type SetState = (
  partial: Partial<EditorStore> | ((s: EditorStore) => Partial<EditorStore>),
) => void;
type GetState = () => EditorStore;

// ---------------------------------------------------------------------------
// Action
// ---------------------------------------------------------------------------

async function runExecution(set: SetState, get: GetState, files: File[]): Promise<void> {
  const state = get();
  const prepared = preparePipeline({
    nodes: state.nodes,
    configs: state.configs,
    recipeMetadata: state.recipeMetadata,
    definition: state.definition,
  });

  if (isPipelineError(prepared)) {
    set({ executionErrors: prepared.errors, executionPhase: "failed" });
    return;
  }

  set({
    executionState: prepared.initialExecutionState,
    executionPhase: "running",
    executionErrors: [],
    executionResults: [],
    executionLogs: [],
    executionFileProgress: null,
    executionInputFiles: files,
  });

  get().openPanel("run");

  try {
    const pendingState = buildPendingState(prepared.definition, prepared.initialExecutionState);
    set({ executionState: pendingState });

    const onEvent = (event: PipelineEvent) => {
      set((s) => ({
        executionLogs: [...s.executionLogs, { timestamp: Date.now(), event }],
      }));

      const next = applyPipelineEvent(get().executionState, event);
      if (next !== get().executionState) {
        set({ executionState: next });
      }

      if (event.type === "FileProgress") {
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
  } catch (err) {
    const message = err instanceof Error ? err.message : "Pipeline execution failed";
    set({
      executionState: buildFailedState(get().executionState),
      executionErrors: [message],
      executionPhase: "failed",
    });
  }
}

export { runExecution };
