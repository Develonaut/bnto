import { createEnhancedStore } from "./createEnhancedStore";
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

/** Apply a progress update with monotonic guard and overall percent computation. */
function applyProgress(
  state: Pick<BrowserExecution, "fileProgress">,
  fileProgress: BrowserFileProgressInput,
): void {
  const prev = state.fileProgress;

  // Monotonic guard: reject backwards progress for the same file.
  if (prev && fileProgress.fileIndex === prev.fileIndex && fileProgress.percent < prev.percent) {
    return;
  }

  const total = fileProgress.totalFiles || 1;
  const overallPercent = Math.round((fileProgress.fileIndex * 100 + fileProgress.percent) / total);
  state.fileProgress = { ...fileProgress, overallPercent };
}

export function createExecutionInstanceStore() {
  return createEnhancedStore<ExecutionInstanceState>()((set) => ({
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

    progress: (fileProgress) => set((state) => applyProgress(state, fileProgress)),

    complete: (results, completedAt) =>
      set({ status: "completed", fileProgress: null, results, completedAt }),

    fail: (message, completedAt) =>
      set({ status: "failed", fileProgress: null, error: message, completedAt }),

    reset: () =>
      set({ ...INITIAL_STATE, error: undefined, startedAt: undefined, completedAt: undefined }),
  }));
}

export type { ExecutionInstanceState };
