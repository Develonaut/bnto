/**
 * Worker initialization helpers — creates the Web Worker instance and
 * handles the one-time "ready" handshake during WASM init.
 */

import type { WorkerRequest, WorkerResponse } from "./workerProtocol";

/** Handler signature for routing messages after init completes. */
export type MessageHandler = (response: WorkerResponse) => void;

/** Create the Web Worker and attach default message + error handlers. */
export function createWorkerInstance(
  onMessage: MessageHandler,
  onError: (err: Error) => void,
): Worker {
  const worker = new Worker(new URL("./bnto.worker.ts", import.meta.url), {
    type: "module",
  });
  worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
    onMessage(event.data);
  };
  worker.onerror = (event) => {
    onError(new Error(`Worker error: ${event.message || "Unknown worker error"}`));
  };
  return worker;
}

/** One-time listener for the "ready" response during init. */
export function attachInitListener(
  worker: Worker,
  resolve: (version: string) => void,
  reject: (err: Error) => void,
  onMessage: MessageHandler,
): void {
  const normalHandler = worker.onmessage;
  worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
    const response = event.data;
    if (response.type === "ready") {
      worker.onmessage = normalHandler;
      resolve(response.version);
    } else if (response.type === "worker-error") {
      reject(new Error(response.message));
    } else {
      onMessage(response);
    }
  };
}

/** Send a message to the worker. */
export function sendRequest(worker: Worker | null, request: WorkerRequest): void {
  worker?.postMessage(request);
}
