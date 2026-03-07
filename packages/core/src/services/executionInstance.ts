"use client";

/**
 * Execution instance — isolated execution lifecycle with opaque store.
 *
 * Each call to `createExecutionInstance()` creates independent state.
 * The Zustand store is attached via Symbol — consumers access state
 * through `core.executions.useExecutionState(instance)`.
 */

import { createExecutionInstanceStore } from "../stores/executionInstanceStore";
import type {
  BrowserFileResult,
  BrowserFileProgressInput,
  BrowserRunResult,
} from "../types/browser";
import type { PipelineDefinition } from "../engine/types";

// ---------------------------------------------------------------------------
// Symbol for opaque store access
// ---------------------------------------------------------------------------

/** @internal Symbol key for accessing the Zustand store on ExecutionInstance. */
export const EXECUTION_STORE = Symbol.for("bnto:execution-store");

// ---------------------------------------------------------------------------
// Execution instance type
// ---------------------------------------------------------------------------

export interface ExecutionInstance {
  /** Execute a PipelineDefinition. Callers with a slug use slugToPipeline() first. */
  run: (definition: PipelineDefinition, files: File[]) => Promise<BrowserRunResult>;
  reset: () => void;
  /** @internal Symbol-keyed store for useExecutionState(). */
  [EXECUTION_STORE]: ReturnType<typeof createExecutionInstanceStore>;
}

// ---------------------------------------------------------------------------
// Progress throttle — ~60fps, always passes first update, 100%, and new file
// ---------------------------------------------------------------------------

const THROTTLE_MS = 16;

function createThrottledProgress(store: ReturnType<typeof createExecutionInstanceStore>) {
  let lastUpdate = 0;

  return (progress: BrowserFileProgressInput) => {
    const now = typeof performance !== "undefined" ? performance.now() : Date.now();

    const current = store.getState().fileProgress;
    const isFirst = current === null;
    const isComplete = progress.percent >= 100;
    const isNewFile = current !== null && current.fileIndex !== progress.fileIndex;

    if (isFirst || isComplete || isNewFile || now - lastUpdate >= THROTTLE_MS) {
      store.getState().progress(progress);
      lastUpdate = now;
    }
  };
}

// ---------------------------------------------------------------------------
// Instance factory
// ---------------------------------------------------------------------------

/** Generate a unique execution ID. */
function generateExecutionId(): string {
  return typeof crypto !== "undefined" && crypto.randomUUID
    ? crypto.randomUUID()
    : `exec-${Date.now()}`;
}

/** Definition-based executor signature. */
type PipelineExecutor = (
  definition: PipelineDefinition,
  files: File[],
  onProgress?: (progress: BrowserFileProgressInput) => void,
) => Promise<BrowserFileResult[]>;

export function createExecutionInstance(executePipeline: PipelineExecutor): ExecutionInstance {
  const store = createExecutionInstanceStore();
  let aborted = false;

  /** Build a progress callback that respects abort and throttles updates. */
  function buildProgressCallback(): (progress: BrowserFileProgressInput) => void {
    const throttled = createThrottledProgress(store);
    return (progress) => {
      if (!aborted) throttled(progress);
    };
  }

  return {
    [EXECUTION_STORE]: store,

    run: async (definition, files) => {
      aborted = false;
      const startedAt = Date.now();
      store.getState().start(generateExecutionId(), startedAt);

      try {
        const results = await executePipeline(definition, files, buildProgressCallback());
        const durationMs = Date.now() - startedAt;
        if (aborted) return { status: "aborted", results: [], durationMs };
        store.getState().complete(results, Date.now());
        return { status: "completed", results, durationMs };
      } catch (e) {
        const durationMs = Date.now() - startedAt;
        if (aborted) return { status: "aborted", results: [], durationMs };
        const error = e instanceof Error ? e.message : "Processing failed";
        store.getState().fail(error, Date.now());
        return { status: "failed", results: [], durationMs, error };
      }
    },

    reset: () => {
      aborted = true;
      store.getState().reset();
    },
  };
}
