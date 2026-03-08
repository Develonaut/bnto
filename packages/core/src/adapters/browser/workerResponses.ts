/**
 * Worker response helpers — typed postMessage wrappers for the worker thread.
 */

import type {
  WorkerResponse,
  WorkerErrorResponse,
  PipelineProgressResponse,
  PipelineResultResponse,
  PipelineErrorResponse,
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

export function sendWorkerError(message: string): void {
  self.postMessage({
    type: "worker-error",
    message,
  } satisfies WorkerErrorResponse);
}

export function sendPipelineProgress(id: string, eventJson: string): void {
  self.postMessage({
    type: "pipeline-progress",
    id,
    eventJson,
  } satisfies PipelineProgressResponse);
}

export function sendPipelineResult(
  id: string,
  files: PipelineResultResponse["files"],
  durationMs: number,
): void {
  const transferables = files.map((f) => f.data);
  postMessageWithTransfer(
    { type: "pipeline-result", id, files, durationMs } satisfies PipelineResultResponse,
    transferables,
  );
}

export function sendPipelineError(id: string, message: string): void {
  self.postMessage({
    type: "pipeline-error",
    id,
    message,
  } satisfies PipelineErrorResponse);
}
