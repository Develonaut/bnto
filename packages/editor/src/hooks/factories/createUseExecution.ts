/**
 * createUseExecution — hook factory for execution lifecycle state.
 *
 * Returns a useExecution hook closure that captures storeApi at creation time.
 * Subscribes to execution-related slices. Derives canRun from nodes and phase.
 * Also exposes executionState and nodeProgress for rendering pipeline consumers.
 */

"use client";

import type { StoreApi } from "zustand";
import { useStore } from "zustand";
import type { EditorStore } from "../../store/types";
import type { ExecutionHookResult } from "../../reactEditorTypes";
import { deriveFileInputAccept } from "../../actions/deriveFileInputAccept";

function createUseExecution(storeApi: StoreApi<EditorStore>) {
  return function useExecution(): ExecutionHookResult {
    const phase = useStore(storeApi, (s) => s.executionPhase);
    const results = useStore(storeApi, (s) => s.executionResults);
    const errors = useStore(storeApi, (s) => s.executionErrors);
    const logs = useStore(storeApi, (s) => s.executionLogs);
    const fileProgress = useStore(storeApi, (s) => s.executionFileProgress);
    const inputFiles = useStore(storeApi, (s) => s.executionInputFiles);
    const executionState = useStore(storeApi, (s) => s.executionState);
    const nodeProgress = useStore(storeApi, (s) => s.nodeProgress);

    const canRun = useStore(storeApi, (s) => {
      if (s.executionPhase === "running") return false;
      return s.nodes.some((n) => {
        const config = s.configs[n.id];
        return config && config.nodeType !== "input" && config.nodeType !== "output";
      });
    });

    const fileAccept = useStore(storeApi, (s) => deriveFileInputAccept(s.configs));

    return {
      phase,
      results,
      errors,
      logs,
      fileProgress,
      inputFiles,
      canRun,
      fileAccept,
      executionState,
      nodeProgress,
    };
  };
}

export { createUseExecution };
