"use client";

import { useCallback } from "react";
import { useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import { core } from "../core";
import type { BrowserFileResult } from "../types/wasm";

/**
 * React hook for WASM execution lifecycle.
 *
 * Thin binding over the service-owned store. The service's `run()` method
 * manages all state transitions (start → progress → complete/fail).
 * This hook reads state via Zustand selectors and provides callbacks
 * that delegate to the service.
 */
export function useWasmExecution() {
  const execution = useStore(
    core.wasm.store,
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

  const run = useCallback(
    async (
      slug: string,
      files: File[],
      params: Record<string, unknown> = {},
    ) => {
      await core.wasm.run(slug, files, params);
    },
    [],
  );

  const downloadResult = useCallback((result: BrowserFileResult) => {
    core.wasm.downloadResult(result);
  }, []);

  const downloadAll = useCallback((slug?: string) => {
    core.wasm.downloadAllResults(
      core.wasm.store.getState().results,
      slug,
    );
  }, []);

  const reset = useCallback(() => {
    core.wasm.reset();
  }, []);

  return {
    execution,
    execute: run,
    downloadResult,
    downloadAll,
    reset,
    isProcessing: execution.status === "processing",
    isCompleted: execution.status === "completed",
    isFailed: execution.status === "failed",
  };
}
