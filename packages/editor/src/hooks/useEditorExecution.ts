"use client";

/**
 * useEditorExecution — manages the execution lifecycle for the editor.
 *
 * Bridges the editor store with @bnto/core's definition-based execution.
 * Reads the current editor state, builds a PipelineDefinition via
 * preparePipeline(), and runs it through core.executions.runPipeline().
 */

import { useState, useCallback, useRef } from "react";
import { core } from "@bnto/core";
import type { BrowserFileResult, PipelineEvent } from "@bnto/core";
import { useEditorStoreApi } from "./useEditorStoreApi";
import { useEditorStore } from "./useEditorStore";
import { preparePipeline, isPipelineError } from "../actions/runPipeline";
import {
  applyPipelineEvent,
  buildPendingState,
  buildFinalState,
  buildFailedState,
} from "../actions/executionState";
import type { RunLogEntry } from "../store/types";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type ExecutionPhase = "idle" | "running" | "completed" | "failed";

interface FileProgress {
  fileIndex: number;
  totalFiles: number;
  overallPercent: number;
  message: string;
}

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
  const [logs, setLogs] = useState<RunLogEntry[]>([]);
  const [fileProgress, setFileProgress] = useState<FileProgress | null>(null);
  const [inputFiles, setInputFiles] = useState<File[]>([]);

  const resultsRef = useRef<BrowserFileResult[]>([]);
  resultsRef.current = results;

  const appendLog = useCallback((entry: RunLogEntry) => {
    setLogs((prev) => [...prev, entry]);
  }, []);

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
      setLogs([]);
      setFileProgress(null);
      setInputFiles(files);

      // Auto-open the run panel when execution starts.
      storeApi.getState().openPanel("run");

      try {
        const pendingState = buildPendingState(prepared.definition, prepared.initialExecutionState);
        storeApi.setState({ executionState: pendingState });

        const onEvent = (event: PipelineEvent) => {
          appendLog({ timestamp: Date.now(), event });
          const next = applyPipelineEvent(storeApi.getState().executionState, event);
          if (next !== storeApi.getState().executionState) {
            storeApi.setState({ executionState: next });
          }

          if (event.type === "FileProgress") {
            const overallPercent = Math.round(
              ((event.fileIndex + event.percent / 100) / event.totalFiles) * 100,
            );
            setFileProgress({
              fileIndex: event.fileIndex,
              totalFiles: event.totalFiles,
              overallPercent,
              message: event.message,
            });
          }
        };

        const browserResults = await core.executions.runPipeline(
          prepared.definition,
          files,
          undefined, // onProgress (legacy)
          onEvent,
        );

        storeApi.setState({ executionState: buildFinalState(prepared.definition) });
        setResults(browserResults);
        setPhase("completed");
      } catch (err) {
        storeApi.setState({
          executionState: buildFailedState(storeApi.getState().executionState),
        });
        const message = err instanceof Error ? err.message : "Pipeline execution failed";
        setErrors([message]);
        setPhase("failed");
      }
    },
    [storeApi, appendLog],
  );

  const reset = useCallback(() => {
    storeApi.setState({ executionState: {} });
    setPhase("idle");
    setResults([]);
    setErrors([]);
    setLogs([]);
    setFileProgress(null);
    setInputFiles([]);
  }, [storeApi]);

  const downloadFile = useCallback((file: BrowserFileResult) => {
    core.executions.downloadResult(file);
  }, []);

  const downloadAll = useCallback(async () => {
    const currentResults = resultsRef.current;
    if (currentResults.length === 0) return;
    await core.executions.downloadAllResults(currentResults, "editor-results");
  }, []);

  return {
    phase,
    results,
    errors,
    logs,
    fileProgress,
    inputFiles,
    canRun: hasProcessingNodes && phase !== "running",
    run,
    reset,
    downloadFile,
    downloadAll,
  };
}

export { useEditorExecution };
export type { EditorExecutionResult, ExecutionPhase };
