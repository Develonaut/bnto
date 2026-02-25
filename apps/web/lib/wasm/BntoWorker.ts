/**
 * BntoWorker — Main-thread wrapper for the WASM Web Worker.
 *
 * Provides a typed, promise-based API for processing files in the browser
 * via WASM. Manages the worker lifecycle (create, init, terminate) and
 * routes responses back to the correct request callbacks.
 *
 * Usage:
 *   const worker = new BntoWorker();
 *   await worker.init();
 *   const result = await worker.processFile(file, "compress-images", { quality: 80 }, onProgress);
 *   worker.terminate();
 */

import type {
  WorkerRequest,
  WorkerResponse,
  ProcessRequest,
  ProgressResponse,
  ResultResponse,
  ErrorResponse,
} from "./types";

/** Callback for progress updates during file processing. */
export type ProgressCallback = (percent: number, message: string) => void;

/** The result of processing a single file. */
export interface ProcessResult {
  /** Processed file as a Blob (ready for download or display). */
  blob: Blob;
  /** Output filename (e.g., "photo-compressed.jpg"). */
  filename: string;
  /** Output MIME type. */
  mimeType: string;
  /** Processing metadata (compression ratio, timing, etc.). */
  metadata: Record<string, unknown>;
}

/** Tracks a pending request waiting for a response from the worker. */
interface PendingRequest {
  resolve: (result: ProcessResult) => void;
  reject: (error: Error) => void;
  onProgress?: ProgressCallback;
}

export class BntoWorker {
  private worker: Worker | null = null;
  private initPromise: Promise<string> | null = null;
  private pending = new Map<string, PendingRequest>();
  private nextId = 0;

  /**
   * Initialize the WASM module in the worker.
   * Must be called before processFile(). Safe to call multiple times.
   * Returns the WASM engine version string.
   */
  async init(): Promise<string> {
    // Deduplicate concurrent init() calls.
    if (this.initPromise) return this.initPromise;

    this.initPromise = this.doInit();
    return this.initPromise;
  }

  /**
   * Process a single file through a WASM node.
   *
   * @param file - The File object to process.
   * @param nodeType - Node type (e.g., "compress-images").
   * @param params - Node-specific config (e.g., { quality: 80 }).
   * @param onProgress - Optional callback for progress updates.
   * @returns The processed file as a Blob with metadata.
   */
  async processFile(
    file: File,
    nodeType: string,
    params: Record<string, unknown> = {},
    onProgress?: ProgressCallback,
  ): Promise<ProcessResult> {
    if (!this.worker) {
      throw new Error("BntoWorker not initialized. Call init() first.");
    }

    const id = String(this.nextId++);
    const data = await file.arrayBuffer();

    return new Promise<ProcessResult>((resolve, reject) => {
      this.pending.set(id, { resolve, reject, onProgress });

      const request: ProcessRequest = {
        type: "process",
        id,
        data,
        filename: file.name,
        mimeType: file.type || "application/octet-stream",
        nodeType,
        params,
      };

      // Transfer the ArrayBuffer to avoid copying.
      this.worker!.postMessage(request, [data]);
    });
  }

  /**
   * Process multiple files sequentially through a WASM node.
   * Reports per-file progress with file index context.
   */
  async processFiles(
    files: File[],
    nodeType: string,
    params: Record<string, unknown> = {},
    onProgress?: (fileIndex: number, percent: number, message: string) => void,
  ): Promise<ProcessResult[]> {
    const results: ProcessResult[] = [];

    for (let i = 0; i < files.length; i++) {
      const fileProgress: ProgressCallback | undefined = onProgress
        ? (percent, message) => onProgress(i, percent, message)
        : undefined;

      const result = await this.processFile(files[i], nodeType, params, fileProgress);
      results.push(result);
    }

    return results;
  }

  /** Clean up the worker. Call when done processing. */
  terminate(): void {
    if (this.worker) {
      this.worker.terminate();
      this.worker = null;
    }
    this.initPromise = null;

    // Reject any pending requests.
    for (const [, pending] of this.pending) {
      pending.reject(new Error("Worker terminated"));
    }
    this.pending.clear();
  }

  /** Whether the worker has been initialized and is ready. */
  get isReady(): boolean {
    return this.worker !== null && this.initPromise !== null;
  }

  // ==========================================================================
  // Private
  // ==========================================================================

  private async doInit(): Promise<string> {
    return new Promise<string>((resolve, reject) => {
      // Create the worker using the URL pattern that bundlers understand.
      this.worker = new Worker(
        new URL("./bnto.worker.ts", import.meta.url),
        { type: "module" },
      );

      // Handle all messages from the worker.
      this.worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
        this.handleMessage(event.data);
      };

      // Handle worker-level errors (syntax errors, uncaught exceptions).
      this.worker.onerror = (event) => {
        const message = event.message || "Unknown worker error";
        reject(new Error(`Worker error: ${message}`));
      };

      // Set up a one-time listener for the "ready" response.
      const originalHandler = this.worker.onmessage;
      this.worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
        const response = event.data;
        if (response.type === "ready") {
          // Restore the normal message handler.
          this.worker!.onmessage = originalHandler;
          resolve(response.version);
        } else if (response.type === "worker-error") {
          reject(new Error(response.message));
        } else {
          // Forward unexpected messages to the normal handler.
          this.handleMessage(response);
        }
      };

      // Send the init request with the page origin so the worker
      // can construct absolute URLs for WASM assets. Workers created
      // from bundled module URLs may not resolve relative paths correctly.
      this.send({ type: "init", baseUrl: globalThis.location?.origin ?? "" });
    });
  }

  private handleMessage(response: WorkerResponse): void {
    switch (response.type) {
      case "progress":
        this.handleProgress(response);
        break;
      case "result":
        this.handleResult(response);
        break;
      case "error":
        this.handleError(response);
        break;
      case "worker-error":
        // Worker-level error not tied to a specific request.
        console.error("[BntoWorker]", response.message);
        break;
      case "ready":
        // Ignore — handled during init.
        break;
    }
  }

  private handleProgress(response: ProgressResponse): void {
    const pending = this.pending.get(response.id);
    pending?.onProgress?.(response.percent, response.message);
  }

  private handleResult(response: ResultResponse): void {
    const pending = this.pending.get(response.id);
    if (!pending) return;

    this.pending.delete(response.id);

    pending.resolve({
      blob: new Blob([response.data], { type: response.mimeType }),
      filename: response.filename,
      mimeType: response.mimeType,
      metadata: response.metadata,
    });
  }

  private handleError(response: ErrorResponse): void {
    const pending = this.pending.get(response.id);
    if (!pending) return;

    this.pending.delete(response.id);
    pending.reject(new Error(response.message));
  }

  private send(request: WorkerRequest): void {
    this.worker?.postMessage(request);
  }
}
