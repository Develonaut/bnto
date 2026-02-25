"use client";

import { useCallback, useReducer, useRef } from "react";
import { core } from "../core";
import {
  browserExecutionReducer,
  IDLE_EXECUTION,
} from "./browserExecutionReducer";
import type {
  BrowserFileProgress,
  BrowserFileResult,
} from "../types/browser";

/**
 * React hook for browser-side execution lifecycle.
 *
 * Manages the full browser execution flow:
 *   1. User calls execute(slug, files, params)
 *   2. Engine initializes and processes each file via WASM
 *   3. Progress updates flow via state (fileProgress)
 *   4. Results are available as in-memory blobs
 *   5. User can download results via downloadResult/downloadAll
 */
export function useBrowserExecution() {
  const [execution, dispatch] = useReducer(
    browserExecutionReducer,
    IDLE_EXECUTION,
  );
  const abortRef = useRef(false);

  const execute = useCallback(
    async (
      slug: string,
      files: File[],
      params: Record<string, unknown> = {},
    ) => {
      abortRef.current = false;

      dispatch({
        type: "start",
        id: crypto.randomUUID(),
        startedAt: Date.now(),
      });

      try {
        const onProgress = (progress: BrowserFileProgress) => {
          if (abortRef.current) return;
          dispatch({ type: "progress", progress });
        };

        const results = await core.browser.execute(
          slug,
          files,
          params,
          onProgress,
        );

        if (abortRef.current) return;

        dispatch({
          type: "complete",
          results,
          completedAt: Date.now(),
        });
      } catch (e) {
        if (abortRef.current) return;

        dispatch({
          type: "fail",
          message: e instanceof Error ? e.message : "Processing failed",
          completedAt: Date.now(),
        });
      }
    },
    [],
  );

  const downloadResult = useCallback((result: BrowserFileResult) => {
    core.browser.downloadResult(result);
  }, []);

  const downloadAll = useCallback(() => {
    core.browser.downloadAllResults(execution.results);
  }, [execution.results]);

  const reset = useCallback(() => {
    abortRef.current = true;
    dispatch({ type: "reset" });
  }, []);

  return {
    execution,
    execute,
    downloadResult,
    downloadAll,
    reset,
    isProcessing: execution.status === "processing",
    isCompleted: execution.status === "completed",
    isFailed: execution.status === "failed",
  };
}
