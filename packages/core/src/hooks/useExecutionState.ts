"use client";

import { useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import {
  EXECUTION_STORE,
  type ExecutionInstance,
} from "../services/executionInstance";
import type { BrowserFileProgress, BrowserFileResult } from "../types/browser";

/**
 * Reactive execution state from an ExecutionInstance.
 *
 * Reads the Zustand store via the opaque Symbol key — consumers never
 * access `.store` directly. Selects only the fields needed for rendering.
 *
 * Usage:
 *   const [instance] = useState(() => core.executions.createExecution());
 *   const state = core.executions.useExecutionState(instance);
 */
export interface ExecutionState {
  id: string;
  status: "idle" | "processing" | "completed" | "failed";
  fileProgress: BrowserFileProgress | null;
  results: BrowserFileResult[];
  error?: string;
  startedAt?: number;
  completedAt?: number;
}

export function useExecutionState(instance: ExecutionInstance): ExecutionState {
  const store = instance[EXECUTION_STORE];

  return useStore(
    store,
    useShallow((s) => ({
      id: s.id,
      status: s.status,
      fileProgress: s.fileProgress,
      results: s.results,
      error: s.error,
      startedAt: s.startedAt,
      completedAt: s.completedAt,
    })),
  );
}
