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
import type { PipelineDefinition } from "../types/pipeline";

// ---------------------------------------------------------------------------
// Symbol for opaque store access
// ---------------------------------------------------------------------------

/** @internal Symbol key for accessing the Zustand store on ExecutionInstance. */
export const EXECUTION_STORE = Symbol.for("bnto:execution-store");

// ---------------------------------------------------------------------------
// Execution instance type
// ---------------------------------------------------------------------------

export interface ExecutionInstance {
  /** Execute a PipelineDefinition with File[]. */
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

/** Build a progress callback that respects abort state and throttles updates. */
function buildProgressCallback(
  store: ReturnType<typeof createExecutionInstanceStore>,
  isAborted: () => boolean,
): (progress: BrowserFileProgressInput) => void {
  const throttled = createThrottledProgress(store);
  return (progress) => {
    if (!isAborted()) throttled(progress);
  };
}

/** Finalize a run — record completed or failed into the store. */
function finalizeRun(
  store: ReturnType<typeof createExecutionInstanceStore>,
  results: BrowserFileResult[],
  durationMs: number,
  error?: unknown,
): BrowserRunResult {
  if (error) {
    const message = error instanceof Error ? error.message : "Processing failed";
    store.getState().fail(message, Date.now());
    return { status: "failed", results: [], durationMs, error: message };
  }
  store.getState().complete(results, Date.now());
  return { status: "completed", results, durationMs };
}

/** Execute a pipeline run with abort awareness and store updates. */
async function executeRun(
  store: ReturnType<typeof createExecutionInstanceStore>,
  executePipeline: PipelineExecutor,
  isAborted: () => boolean,
  definition: PipelineDefinition,
  files: File[],
): Promise<BrowserRunResult> {
  const startedAt = Date.now();
  store.getState().start(generateExecutionId(), startedAt);

  try {
    const results = await executePipeline(
      definition,
      files,
      buildProgressCallback(store, isAborted),
    );
    const durationMs = Date.now() - startedAt;
    return isAborted()
      ? { status: "aborted", results: [], durationMs }
      : finalizeRun(store, results, durationMs);
  } catch (e) {
    const durationMs = Date.now() - startedAt;
    return isAborted()
      ? { status: "aborted", results: [], durationMs }
      : finalizeRun(store, [], durationMs, e);
  }
}

export function createExecutionInstance(executePipeline: PipelineExecutor): ExecutionInstance {
  const store = createExecutionInstanceStore();
  let aborted = false;

  return {
    [EXECUTION_STORE]: store,
    run: (definition, files) => {
      aborted = false;
      return executeRun(store, executePipeline, () => aborted, definition, files);
    },
    reset: () => {
      aborted = true;
      store.getState().reset();
    },
  };
}
