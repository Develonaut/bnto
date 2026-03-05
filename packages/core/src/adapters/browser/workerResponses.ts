/**
 * Worker response helpers — typed postMessage wrappers for the worker thread.
 */

import type {
  WorkerResponse,
  ProgressResponse,
  ResultResponse,
  ErrorResponse,
  WorkerErrorResponse,
} from "./workerProtocol";

// Worker postMessage with Transferable[] for zero-copy ArrayBuffer transfers.
function postMessageWithTransfer(msg: unknown, transfer: Transferable[]): void {
  (self as unknown as { postMessage(m: unknown, t: Transferable[]): void }).postMessage(
    msg,
    transfer,
  );
}

export function send(response: WorkerResponse): void {
  self.postMessage(response);
}

export function sendProgress(id: string, percent: number, message: string): void {
  self.postMessage({
    type: "progress",
    id,
    percent,
    message,
  } satisfies ProgressResponse);
}

export function sendResult(
  id: string,
  data: ArrayBuffer,
  filename: string,
  mimeType: string,
  metadata: Record<string, unknown>,
): void {
  const response: ResultResponse = {
    type: "result",
    id,
    data,
    filename,
    mimeType,
    metadata,
  };
  postMessageWithTransfer(response, [data]);
}

export function sendError(id: string, message: string): void {
  self.postMessage({ type: "error", id, message } satisfies ErrorResponse);
}

export function sendWorkerError(message: string): void {
  self.postMessage({
    type: "worker-error",
    message,
  } satisfies WorkerErrorResponse);
}
