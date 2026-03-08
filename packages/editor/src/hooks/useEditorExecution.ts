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

  const resultsRef = useRef<BrowserFileResult[]>([]);
  resultsRef.current = results;

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
        // Mark all processing nodes as pending before execution starts.
        const pendingState: ExecutionState = { ...prepared.initialExecutionState };
        for (const node of prepared.definition.nodes) {
          if (node.type !== "input" && node.type !== "output") {
            pendingState[node.id] = "pending";
          }
        }
        storeApi.setState({ executionState: pendingState });

        // Handle structured PipelineEvents from the Rust executor.
        // These drive real-time per-node status in the editor.
        const onEvent = (event: PipelineEvent) => {
          const current = storeApi.getState().executionState;
          switch (event.type) {
            case "NodeStarted":
              storeApi.setState({
                executionState: { ...current, [event.nodeId]: "active" },
              });
              break;
            case "NodeCompleted":
              storeApi.setState({
                executionState: { ...current, [event.nodeId]: "completed" },
              });
              break;
            case "NodeFailed":
              storeApi.setState({
                executionState: { ...current, [event.nodeId]: "failed" },
              });
              break;
          }
        };

        // Run via core's definition-based execution path with structured events.
        const browserResults = await core.executions.runPipeline(
          prepared.definition,
          files,
          undefined, // onProgress (legacy)
          onEvent,
        );

        // Finalize: mark output node as completed, input as idle.
        const finalState: ExecutionState = {};
        for (const node of prepared.definition.nodes) {
          if (node.type === "input") finalState[node.id] = "idle";
          else finalState[node.id] = "completed";
        }
        storeApi.setState({ executionState: finalState });

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

  return {
    phase,
    results,
    errors,
    canRun: hasProcessingNodes && phase !== "running",
    run,
    reset,
    downloadFile,
    downloadAll,
  };
}

export { useEditorExecution };
export type { EditorExecutionResult, ExecutionPhase };
