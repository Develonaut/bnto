import type {
  BrowserExecution,
  BrowserFileProgress,
  BrowserFileResult,
} from "../types/browser";

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

export const IDLE_EXECUTION: BrowserExecution = {
  id: "",
  status: "idle",
  fileProgress: null,
  results: [],
};

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

export type BrowserExecutionAction =
  | { type: "start"; id: string; startedAt: number }
  | { type: "progress"; progress: BrowserFileProgress }
  | { type: "complete"; results: BrowserFileResult[]; completedAt: number }
  | { type: "fail"; message: string; completedAt: number }
  | { type: "reset" };

// ---------------------------------------------------------------------------
// Reducer (pure function — no React dependency)
// ---------------------------------------------------------------------------

export function browserExecutionReducer(
  state: BrowserExecution,
  action: BrowserExecutionAction,
): BrowserExecution {
  switch (action.type) {
    case "start":
      return {
        id: action.id,
        status: "processing",
        fileProgress: null,
        results: [],
        startedAt: action.startedAt,
      };

    case "progress":
      return { ...state, fileProgress: action.progress };

    case "complete":
      return {
        ...state,
        status: "completed",
        fileProgress: null,
        results: action.results,
        completedAt: action.completedAt,
      };

    case "fail":
      return {
        ...state,
        status: "failed",
        fileProgress: null,
        error: action.message,
        completedAt: action.completedAt,
      };

    case "reset":
      return IDLE_EXECUTION;
  }
}
