"use client";

import { useCallback, useRef } from "react";
import { useShallow } from "zustand/react/shallow";
import { core } from "../core";
import {
  browserExecutionStore,
  useBrowserExecutionStore,
} from "../stores/browserExecutionStore";
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
  const execution = useBrowserExecutionStore(
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
  const abortRef = useRef(false);

  const execute = useCallback(
    async (
      slug: string,
      files: File[],
      params: Record<string, unknown> = {},
    ) => {
      abortRef.current = false;
      const store = browserExecutionStore.getState();

      store.start(crypto.randomUUID(), Date.now());

      try {
        const onProgress = (progress: BrowserFileProgress) => {
          if (abortRef.current) return;
          browserExecutionStore.getState().progress(progress);
        };

        const results = await core.browser.execute(
          slug,
          files,
          params,
          onProgress,
        );

        if (abortRef.current) return;

        browserExecutionStore.getState().complete(results, Date.now());
      } catch (e) {
        if (abortRef.current) return;

        browserExecutionStore.getState().fail(
          e instanceof Error ? e.message : "Processing failed",
          Date.now(),
        );
      }
    },
    [],
  );

  const downloadResult = useCallback((result: BrowserFileResult) => {
    core.browser.downloadResult(result);
  }, []);

  const downloadAll = useCallback(() => {
    core.browser.downloadAllResults(browserExecutionStore.getState().results);
  }, []);

  const reset = useCallback(() => {
    abortRef.current = true;
    browserExecutionStore.getState().reset();
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
