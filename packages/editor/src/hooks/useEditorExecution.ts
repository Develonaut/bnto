"use client";

/**
 * useEditorExecution — manages the execution lifecycle for the editor.
 *
 * Bridges the editor store with @bnto/core's definition-based execution.
 * Reads the current editor state, builds a PipelineDefinition via
 * preparePipeline(), and runs it through core.executions.runPipeline().
 */

import { useState, useCallback, useRef, useEffect, useMemo } from "react";
import { core } from "@bnto/core";
import type { BrowserFileResult } from "@bnto/core";
import { useEditorStoreApi } from "./useEditorStoreApi";
import { useEditorStore } from "./useEditorStore";
import { preparePipeline, isPipelineError, buildExecutionState } from "../actions/runPipeline";
import type { ExecutionState } from "../store/types";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type ExecutionPhase = "idle" | "running" | "completed" | "failed";

interface EditorExecutionResult {
  phase: ExecutionPhase;
  results: BrowserFileResult[];
  errors: string[];
  canRun: boolean;
  autoDownload: boolean;
  setAutoDownload: (enabled: boolean) => void;
  run: (files: File[]) => Promise<void>;
  reset: () => void;
  downloadFile: (file: BrowserFileResult) => void;
  downloadAll: () => Promise<void>;
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

function useEditorExecution(): EditorExecutionResult {
  const storeApi = useEditorStoreApi();
  const hasProcessingNodes = useEditorStore((s) => {
    const configs = s.configs;
    return s.nodes.some((n) => {
      const config = configs[n.id];
      return config && config.nodeType !== "input" && config.nodeType !== "output";
    });
  });

  const [phase, setPhase] = useState<ExecutionPhase>("idle");
  const [results, setResults] = useState<BrowserFileResult[]>([]);
  const [errors, setErrors] = useState<string[]>([]);
  const [autoDownload, setAutoDownload] = useState(false);

  const resultsRef = useRef<BrowserFileResult[]>([]);
  resultsRef.current = results;

  // Auto-download when execution completes
  useEffect(() => {
    if (phase === "completed" && autoDownload && resultsRef.current.length > 0) {
      if (resultsRef.current.length === 1) {
        core.executions.downloadResult(resultsRef.current[0]!);
      } else {
        core.executions.downloadAllResults(resultsRef.current, "editor-results");
      }
    }
  }, [phase, autoDownload]);

  const run = useCallback(
    async (files: File[]) => {
      const state = storeApi.getState();

      const prepared = preparePipeline({
        nodes: state.nodes,
        configs: state.configs,
        recipeMetadata: state.recipeMetadata,
      });

      if (isPipelineError(prepared)) {
        setErrors(prepared.errors);
        setPhase("failed");
        return;
      }

      storeApi.setState({ executionState: prepared.initialExecutionState });
      setPhase("running");
      setErrors([]);
      setResults([]);

      try {
        storeApi.setState({ executionState: buildExecutionState(prepared.definition, "active") });

        // Run via core's definition-based execution path
        const browserResults = await core.executions.runPipeline(prepared.definition, files);

        storeApi.setState({
          executionState: buildExecutionState(prepared.definition, "completed"),
        });

        setResults(browserResults);
        setPhase("completed");
      } catch (err) {
        const currentExec = storeApi.getState().executionState;
        const failedState: ExecutionState = { ...currentExec };
        for (const [nodeId, status] of Object.entries(failedState)) {
          if (status === "active") failedState[nodeId] = "failed";
          if (status === "pending") failedState[nodeId] = "idle";
        }
        storeApi.setState({ executionState: failedState });

        const message = err instanceof Error ? err.message : "Pipeline execution failed";
        console.error("[editor execution]", message, err);
        setErrors([message]);
        setPhase("failed");
      }
    },
    [storeApi],
  );

  const reset = useCallback(() => {
    storeApi.setState({ executionState: {} });
    setPhase("idle");
    setResults([]);
    setErrors([]);
  }, [storeApi]);

  const downloadFile = useCallback((file: BrowserFileResult) => {
    core.executions.downloadResult(file);
  }, []);

  const downloadAll = useCallback(async () => {
    const currentResults = resultsRef.current;
    if (currentResults.length === 0) return;
    await core.executions.downloadAllResults(currentResults, "editor-results");
  }, []);

  const canRun = hasProcessingNodes && phase !== "running";

  return useMemo(
    () => ({
      phase,
      results,
      errors,
      canRun,
      autoDownload,
      setAutoDownload,
      run,
      reset,
      downloadFile,
      downloadAll,
    }),
    [phase, results, errors, canRun, autoDownload, run, reset, downloadFile, downloadAll],
  );
}

export { useEditorExecution };
export type { EditorExecutionResult, ExecutionPhase };
