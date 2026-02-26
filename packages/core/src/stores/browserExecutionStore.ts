import { createStore } from "zustand/vanilla";
import type {
  BrowserExecution,
  BrowserFileProgress,
  BrowserFileResult,
} from "../types/browser";

// ---------------------------------------------------------------------------
// State shape
// ---------------------------------------------------------------------------

interface BrowserExecutionState extends BrowserExecution {
  /** Transition to processing. Clears prior error/results. */
  start: (id: string, startedAt: number) => void;
  /** Update per-file progress. */
  progress: (progress: BrowserFileProgress) => void;
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

export function createBrowserExecutionStore() {
  return createStore<BrowserExecutionState>()((set) => ({
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

    progress: (fileProgress) => set({ fileProgress }),

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

// ---------------------------------------------------------------------------
// Singleton store instance
// ---------------------------------------------------------------------------

export const browserExecutionStore = createBrowserExecutionStore();

export type { BrowserExecutionState };
