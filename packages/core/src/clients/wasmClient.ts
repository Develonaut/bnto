"use client";

import type { WasmExecutionService } from "../services/wasmExecutionService";

/**
 * WASM execution client — public API for browser-side WASM execution.
 *
 * Wraps the WASM execution service for architectural symmetry with
 * other domain clients (workflows, executions, user, etc.).
 */
export function createWasmClient(wasm: WasmExecutionService) {
  return {
    /** The singleton Zustand store (backward-compatible). Prefer createExecution(). */
    store: wasm.store,

    // ── Capability Detection ───────────────────────────────────────
    isCapable: (slug: string) => wasm.isCapable(slug),
    hasImplementation: (slug: string) => wasm.hasImplementation(slug),
    getCapableSlugs: () => wasm.getCapableSlugs(),

    // ── Engine Registration ────────────────────────────────────────
    registerEngine: wasm.registerEngine,
    hasEngine: () => wasm.hasEngine(),

    // ── Per-Instance Factory ────────────────────────────────────────
    /**
     * Create an isolated execution instance with its own store.
     *
     * Each instance has independent state — no cross-page leaks.
     * Usage: `const [instance] = useState(() => core.wasm.createExecution())`
     */
    createExecution: wasm.createExecution,

    // ── Singleton Lifecycle (backward-compatible) ───────────────────
    /** Run via the singleton store (backward-compatible). Prefer createExecution(). */
    run: wasm.run,
    /** Reset the singleton store (backward-compatible). */
    reset: wasm.reset,

    // ── Low-level execution ────────────────────────────────────────
    /** Execute without lifecycle management. For callers managing their own state. */
    execute: wasm.execute,

    // ── Download ───────────────────────────────────────────────────
    downloadResult: wasm.downloadResult,
    downloadAllResults: wasm.downloadAllResults,
  } as const;
}

export type WasmClient = ReturnType<typeof createWasmClient>;
