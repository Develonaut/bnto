import { useEffect, useRef } from "react";
import { core } from "@bnto/core";
import { BntoWorker } from "@/lib/wasm/BntoWorker";
import { toBrowserEngine } from "@/lib/wasm/toBrowserEngine";

/** Register BntoWorker as the browser engine for WASM execution. */
export function useBrowserEngine(enabled: boolean): void {
  const workerRef = useRef<BntoWorker | null>(null);

  useEffect(() => {
    if (!enabled) return;

    const worker = new BntoWorker();
    workerRef.current = worker;
    core.browser.registerEngine(toBrowserEngine(worker));

    return () => {
      worker.terminate();
      workerRef.current = null;
    };
  }, [enabled]);
}
