/** Main-thread wrapper for the WASM Web Worker — typed, promise-based pipeline execution. */

import type {
  ExecutePipelineRequest,
  PipelineProgressResponse,
  PipelineResultResponse,
  PipelineErrorResponse,
  WorkerResponse,
} from "./workerProtocol";
import { createWorkerInstance, attachInitListener, sendRequest } from "./initWorker";
import type { PendingPipelineRequest, PipelineExecutionResult } from "./workerTypes";
import type { PipelineEvent } from "../../types/pipelineEvents";

export type { PipelineExecutionResult } from "./workerTypes";

export class BntoWorker {
  private worker: Worker | null = null;
  private initPromise: Promise<string> | null = null;
  private pendingPipelines = new Map<string, PendingPipelineRequest>();
  private nextId = 0;

  /**
   * Initialize the WASM module in the worker.
   * Must be called before executePipeline(). Safe to call multiple times.
   * Returns the WASM engine version string.
   */
  async init(): Promise<string> {
    // Deduplicate concurrent init() calls.
    if (this.initPromise) return this.initPromise;

    this.initPromise = this.doInit();
    return this.initPromise;
  }

  /**
   * Execute a full pipeline through the WASM executor.
   *
   * Sends the pipeline definition and files to the worker, which delegates
   * to the Rust executor. Progress events are forwarded via the onEvent callback.
   *
   * @param definitionJson - JSON string of the pipeline definition.
   * @param files - Input files to process.
   * @param onEvent - Optional callback for structured PipelineEvent updates.
   * @returns The pipeline execution result with all output files.
   */
  async executePipeline(
    definitionJson: string,
    files: File[],
    onEvent?: (event: PipelineEvent) => void,
  ): Promise<PipelineExecutionResult> {
    if (!this.worker) {
      throw new Error("BntoWorker not initialized. Call init() first.");
    }

    const worker = this.worker;
    const id = String(this.nextId++);

    // Convert File objects to transferable ArrayBuffer format.
    const fileData = await Promise.all(
      files.map(async (f) => ({
        name: f.name,
        data: await f.arrayBuffer(),
        mimeType: f.type || "application/octet-stream",
      })),
    );

    return new Promise<PipelineExecutionResult>((resolve, reject) => {
      this.pendingPipelines.set(id, { resolve, reject, onEvent });

      const request: ExecutePipelineRequest = {
        type: "execute-pipeline",
        id,
        definitionJson,
        files: fileData,
      };

      // Transfer all file ArrayBuffers to avoid copying.
      const transferables = fileData.map((f) => f.data);
      worker.postMessage(request, transferables);
    });
  }

  /** Clean up the worker. Call when done processing. */
  terminate(): void {
    if (this.worker) {
      this.worker.terminate();
      this.worker = null;
    }
    this.initPromise = null;

    for (const [, pending] of this.pendingPipelines) {
      pending.reject(new Error("Worker terminated"));
    }
    this.pendingPipelines.clear();
  }

  /** Whether the worker has been initialized and is ready. */
  get isReady(): boolean {
    return this.worker !== null && this.initPromise !== null;
  }

  // ==========================================================================
  // Private
  // ==========================================================================

  private async doInit(): Promise<string> {
    const onMessage = (r: WorkerResponse) => this.handleMessage(r);
    return new Promise<string>((resolve, reject) => {
      this.worker = createWorkerInstance(onMessage, reject);
      attachInitListener(this.worker, resolve, reject, onMessage);
      sendRequest(this.worker, {
        type: "init",
        baseUrl: globalThis.location?.origin ?? "",
      });
    });
  }

  private handleMessage(response: WorkerResponse): void {
    switch (response.type) {
      case "pipeline-progress":
        return this.handlePipelineProgress(response);
      case "pipeline-result":
        return this.handlePipelineResult(response);
      case "pipeline-error":
        return this.handlePipelineError(response);
      case "worker-error":
        console.error("[BntoWorker]", response.message);
        return;
      case "ready":
        return; // Handled during init.
    }
  }

  private handlePipelineProgress(response: PipelineProgressResponse): void {
    const pending = this.pendingPipelines.get(response.id);
    if (!pending?.onEvent) return;

    try {
      const event = JSON.parse(response.eventJson) as PipelineEvent;
      pending.onEvent(event);
    } catch {
      // Ignore malformed event JSON — progress is best-effort.
    }
  }

  private handlePipelineResult(response: PipelineResultResponse): void {
    const pending = this.pendingPipelines.get(response.id);
    if (!pending) return;

    this.pendingPipelines.delete(response.id);
    pending.resolve({
      files: response.files,
      durationMs: response.durationMs,
    });
  }

  private handlePipelineError(response: PipelineErrorResponse): void {
    const pending = this.pendingPipelines.get(response.id);
    if (!pending) return;

    this.pendingPipelines.delete(response.id);
    pending.reject(new Error(response.message));
  }
}
