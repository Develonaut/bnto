"use client";

import type { WasmExecutionService, WasmExecutionInstance } from "../services/wasmExecutionService";
import type { HistoryService } from "../services/historyService";
import type { WasmRunResult } from "../types/wasm";

/**
 * WASM execution client — composes WASM execution with history recording.
 * After every run() completes, the result is auto-recorded to local history.
 */
export function createWasmClient(
  wasm: WasmExecutionService,
  history: HistoryService,
) {
  /** Wrap an instance's run() to auto-record to local history (fire-and-forget). */
  function wrapInstance(instance: WasmExecutionInstance): WasmExecutionInstance {
    const originalRun = instance.run;

    return {
      ...instance,
      run: async (
        slug: string,
        files: File[],
        params?: Record<string, unknown>,
      ): Promise<WasmRunResult> => {
        const result = await originalRun(slug, files, params);

        if (result.status !== "aborted") {
          history.record({
            id: crypto.randomUUID(),
            slug,
            status: result.status,
            timestamp: Date.now() - result.durationMs,
            durationMs: result.durationMs,
            inputFileCount: files.length,
            outputFileCount: result.results.length,
            error: result.error,
          }).catch(() => {});
        }

        return result;
      },
    };
  }

  return {
    isCapable: (slug: string) => wasm.isCapable(slug),
    hasImplementation: (slug: string) => wasm.hasImplementation(slug),
    getCapableSlugs: () => wasm.getCapableSlugs(),
    registerEngine: wasm.registerEngine,
    hasEngine: () => wasm.hasEngine(),

    /**
     * Create an isolated execution instance with its own store.
     * Results are automatically recorded to local history.
     * Usage: `const [instance] = useState(() => core.wasm.createExecution())`
     */
    createExecution: (): WasmExecutionInstance =>
      wrapInstance(wasm.createExecution()),

    execute: wasm.execute,
    downloadResult: wasm.downloadResult,
    downloadAllResults: wasm.downloadAllResults,
  } as const;
}

export type WasmClient = ReturnType<typeof createWasmClient>;
