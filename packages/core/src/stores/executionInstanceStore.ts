import { createStore } from "zustand/vanilla";
import type {
  BrowserExecution,
  BrowserFileProgressInput,
  BrowserFileResult,
} from "../types/browser";

// ---------------------------------------------------------------------------
// State shape
// ---------------------------------------------------------------------------

interface ExecutionInstanceState extends BrowserExecution {
  /** Transition to processing. Clears prior error/results. */
  start: (id: string, startedAt: number) => void;
  /** Update per-file progress. Computes overallPercent and enforces monotonic guard. */
  progress: (progress: BrowserFileProgressInput) => void;
  /** Transition to completed with results. Clears progress. */
  complete: (results: BrowserFileResult[], completedAt: number) => void;
  /** Transition to failed with error. Clears progress. */
  fail: (message: string, completedAt: number) => void;
  /** Reset to idle. */
  reset: () => void;
}

// ---------------------------------------------------------------------------
// Initial state
// ---------------------------------------------------------------------------

const INITIAL_STATE: BrowserExecution = {
  id: "",
  status: "idle",
  fileProgress: null,
  results: [],
};

// ---------------------------------------------------------------------------
// Store factory
// ---------------------------------------------------------------------------

export function createExecutionInstanceStore() {
  return createStore<ExecutionInstanceState>()((set) => ({
    ...INITIAL_STATE,

    start: (id, startedAt) =>
      set({
        id,
        status: "processing",
        fileProgress: null,
        results: [],
        startedAt,
        error: undefined,
        completedAt: undefined,
      }),

    progress: (fileProgress) =>
      set((state) => {
        const prev = state.fileProgress;

        // Monotonic guard: reject backwards progress for the same file.
        // This prevents progress bar regression (e.g., 50→30) caused by
        // duplicate WASM calls or out-of-order callbacks.
        if (
          prev &&
          fileProgress.fileIndex === prev.fileIndex &&
          fileProgress.percent < prev.percent
        ) {
          return state;
        }

        // Compute overall batch progress: how far through ALL files.
        // Formula: ((completedFiles * 100) + currentFilePercent) / totalFiles
        const total = fileProgress.totalFiles || 1;
        const overallPercent = Math.round(
          ((fileProgress.fileIndex * 100) + fileProgress.percent) / total,
        );

        return {
          fileProgress: { ...fileProgress, overallPercent },
        };
      }),

    complete: (results, completedAt) =>
      set({
        status: "completed",
        fileProgress: null,
        results,
        completedAt,
      }),

    fail: (message, completedAt) =>
      set({
        status: "failed",
        fileProgress: null,
        error: message,
        completedAt,
      }),

    reset: () =>
      set({
        ...INITIAL_STATE,
        error: undefined,
        startedAt: undefined,
        completedAt: undefined,
      }),
  }));
}

export type { ExecutionInstanceState };
