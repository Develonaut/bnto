/**
 * Adapter: BntoWorker → BrowserEngine.
 *
 * The @bnto/core BrowserEngine interface expects init() → Promise<void>,
 * but BntoWorker.init() returns Promise<string> (the WASM version).
 * This adapter bridges the type mismatch cleanly without casting.
 */

import type { BrowserEngine } from "@bnto/core";
import type { BntoWorker } from "./BntoWorker";

export function toBrowserEngine(worker: BntoWorker): BrowserEngine {
  return {
    init: async () => {
      await worker.init();
    },
    processFile: (file, nodeType, params, onProgress) =>
      worker.processFile(file, nodeType, params ?? {}, onProgress),
    processFiles: (files, nodeType, params, onProgress) =>
      worker.processFiles(files, nodeType, params ?? {}, onProgress),
    terminate: () => worker.terminate(),
    get isReady() {
      return worker.isReady;
    },
  };
}
