/**
 * useEditorExecution — convenience hook for execution lifecycle state + actions.
 *
 * All state lives in the Zustand store. This hook selects the relevant
 * slices and composes them into a single return object. Components can
 * also use useEditorStore() selectors directly for narrower subscriptions.
 */

"use client";

import { useShallow } from "@bnto/core";
import type { BrowserFileResult } from "@bnto/core";
import { useEditorStore } from "./useEditorStore";
import type { ExecutionPhase, RunLogEntry, FileProgress } from "../store/types";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface EditorExecutionResult {
  phase: ExecutionPhase;
  results: BrowserFileResult[];
  errors: string[];
  logs: RunLogEntry[];
  fileProgress: FileProgress | null;
  inputFiles: File[];
  canRun: boolean;
  run: (files: File[]) => Promise<void>;
  reset: () => void;
  downloadFile: (file: BrowserFileResult) => void;
  downloadAll: () => Promise<void>;
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

function useEditorExecution(): EditorExecutionResult {
  const phase = useEditorStore((s) => s.executionPhase);
  const results = useEditorStore((s) => s.executionResults);
  const errors = useEditorStore((s) => s.executionErrors);
  const logs = useEditorStore((s) => s.executionLogs);
  const fileProgress = useEditorStore((s) => s.executionFileProgress);
  const inputFiles = useEditorStore((s) => s.executionInputFiles);

  const actions = useEditorStore(
    useShallow((s) => ({
      run: s.runExecution,
      reset: s.resetRun,
      downloadFile: s.downloadResult,
      downloadAll: s.downloadAllResults,
    })),
  );

  const canRun = useEditorStore((s) => {
    if (s.executionPhase === "running") return false;
    return s.nodes.some((n) => {
      const config = s.configs[n.id];
      return config && config.nodeType !== "input" && config.nodeType !== "output";
    });
  });

  return {
    phase,
    results,
    errors,
    logs,
    fileProgress,
    inputFiles,
    canRun,
    ...actions,
  };
}

export { useEditorExecution };
export type { EditorExecutionResult };
