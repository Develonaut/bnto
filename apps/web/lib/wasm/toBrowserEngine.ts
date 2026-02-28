/**
 * Adapter: BntoWorker → BrowserEngine.
 *
 * WHY THIS EXISTS:
 * @bnto/core defines a BrowserEngine interface — the "contract" that says
 * what a browser engine must look like. BntoWorker is our concrete
 * implementation (it manages a Web Worker that runs WASM). But BntoWorker's
 * API doesn't match the BrowserEngine interface exactly:
 *
 *   - BntoWorker.init() returns Promise<string> (the WASM version string)
 *   - BrowserEngine.init() returns Promise<void> (doesn't care about version)
 *
 *   - BntoWorker.processFile() requires params to be a non-optional object
 *   - BrowserEngine.processFile() has params as optional (params?: ...)
 *
 * This function is the "adapter" — it wraps BntoWorker and smooths out
 * these small differences so it fits the BrowserEngine interface perfectly.
 *
 * ANALOGY:
 * It's like a travel adapter for electrical plugs. Your laptop charger
 * (BntoWorker) works fine, but the outlet in the hotel (BrowserEngine)
 * has a different shape. The adapter doesn't change what the charger does —
 * it just makes the plug fit the outlet.
 *
 * WHY NOT JUST MAKE BNTOWORKER IMPLEMENT BROWSERENGINE DIRECTLY?
 * We could, but that would couple BntoWorker to @bnto/core's interface.
 * Keeping them separate means:
 *   1. BntoWorker can evolve independently (e.g., return version from init)
 *   2. We could swap BntoWorker for a different engine (e.g., JS-only
 *      fallback) without touching @bnto/core
 *   3. The adapter is trivially simple — a good sign that the abstraction
 *      boundary is in the right place
 */

import type { BrowserEngine } from "@bnto/core";
import type { BntoWorker } from "./BntoWorker";

/**
 * Wrap a BntoWorker instance as a BrowserEngine.
 *
 * The returned object satisfies the BrowserEngine interface from @bnto/core,
 * making the worker usable with core.wasm.registerEngine().
 *
 * @param worker - The BntoWorker instance to adapt
 * @returns A BrowserEngine-compatible object that delegates to the worker
 */
export function toBrowserEngine(worker: BntoWorker): BrowserEngine {
  return {
    // BntoWorker.init() returns the WASM version string, but BrowserEngine
    // just wants Promise<void>. We await (so errors still propagate) but
    // discard the return value.
    init: async () => {
      await worker.init();
    },

    // BrowserEngine has params as optional (?), BntoWorker requires it.
    // The ?? {} provides the default empty object when params is undefined.
    processFile: (file, nodeType, params, onProgress) =>
      worker.processFile(file, nodeType, params ?? {}, onProgress),

    processFiles: (files, nodeType, params, onProgress) =>
      worker.processFiles(files, nodeType, params ?? {}, onProgress),

    // These pass through directly — signatures already match.
    terminate: () => worker.terminate(),
    get isReady() {
      return worker.isReady;
    },
  };
}
