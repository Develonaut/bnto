/**
 * useBrowserEngine — React hook that wires up the WASM Web Worker.
 *
 * WHY THIS EXISTS:
 * @bnto/core defines a BrowserEngine *interface* — it says "I need something
 * that can init(), processFile(), and terminate()." But core doesn't know
 * HOW files get processed. It doesn't know about WASM, Web Workers, or
 * any specific technology.
 *
 * This hook is the bridge. It creates a BntoWorker (the concrete WASM
 * implementation), wraps it in the BrowserEngine interface shape, and
 * registers it with core. After this runs, any call to
 * `core.browser.execute()` will route through our WASM worker.
 *
 * ANALOGY:
 * Think of core.browser as an electrical outlet. It defines the shape
 * of the plug (BrowserEngine interface). This hook is the power cord —
 * it plugs the WASM worker (the appliance) into the outlet so
 * electricity (file processing) can flow.
 *
 * LIFECYCLE:
 *   1. Component mounts with enabled=true
 *   2. Effect creates a new BntoWorker (spawns a Web Worker thread)
 *   3. toBrowserEngine() wraps it to match the BrowserEngine interface
 *   4. core.browser.registerEngine() makes it available globally
 *   5. When the component unmounts → cleanup terminates the worker
 *
 * WHY `enabled`?
 * Not every bnto page uses WASM. Cloud-only bntos (like shell commands)
 * don't need a Web Worker at all. The `enabled` flag avoids spawning
 * a worker we'll never use — workers consume memory and a thread.
 *
 * WHY `useRef`?
 * We store the worker in a ref so the cleanup function (the return
 * from useEffect) can access it. The ref persists across renders
 * without causing re-renders — we don't need the UI to know about
 * the worker's existence, we just need to clean it up.
 */

import { useEffect, useRef } from "react";
import { core } from "@bnto/core";
import { BntoWorker } from "./BntoWorker";
import { toBrowserEngine } from "./toBrowserEngine";

/**
 * Register BntoWorker as the browser engine for WASM execution.
 *
 * Call this once at the page level (e.g., in BntoPageShell) when the
 * current bnto supports browser execution. The worker will be created
 * on mount and terminated on unmount.
 *
 * @param enabled - Whether to create and register the worker.
 *   Pass `true` for browser-capable bntos, `false` for cloud-only.
 */
export function useBrowserEngine(enabled: boolean): void {
  // Ref keeps a handle to the worker so we can terminate it on cleanup.
  // We don't use useState because changing the worker doesn't need
  // to trigger a re-render — the UI doesn't display the worker itself.
  const workerRef = useRef<BntoWorker | null>(null);

  useEffect(() => {
    // Skip worker creation if this bnto doesn't use browser execution
    if (!enabled) return;

    // --- Step 1: Create the Web Worker ---
    // BntoWorker spawns a background thread that will load the WASM binary.
    // The actual WASM loading happens later when init() is called (lazy).
    const worker = new BntoWorker();
    workerRef.current = worker;

    // --- Step 2: Register with @bnto/core ---
    // toBrowserEngine() wraps BntoWorker in the BrowserEngine interface.
    // registerEngine() makes it the global engine that core.browser.execute()
    // will use. From this point on, any component calling useBrowserExecution()
    // will route through this worker.
    core.browser.registerEngine(toBrowserEngine(worker));

    // --- Step 3: Cleanup on unmount ---
    // When the user navigates away from the bnto page, terminate the worker
    // to free the background thread and any WASM memory it allocated.
    return () => {
      worker.terminate();
      workerRef.current = null;
    };
  }, [enabled]);
}
