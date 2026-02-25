import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { BntoWorker } from "../BntoWorker";
import type { WorkerResponse } from "../types";

// ---------------------------------------------------------------------------
// Mock Worker
// ---------------------------------------------------------------------------

/** Minimal mock of the Web Worker API for testing BntoWorker. */
class MockWorker {
  onmessage: ((event: MessageEvent<WorkerResponse>) => void) | null = null;
  onerror: ((event: { message: string }) => void) | null = null;
  private terminated = false;

  postMessage = vi.fn();
  terminate = vi.fn(() => {
    this.terminated = true;
  });

  /** Simulate the worker sending a message back to the main thread. */
  simulateMessage(data: WorkerResponse): void {
    if (this.terminated) return;
    this.onmessage?.(new MessageEvent("message", { data }));
  }

  /** Simulate a worker-level error. */
  simulateError(message: string): void {
    if (this.terminated) return;
    this.onerror?.({ message });
  }
}

// Capture the most recent MockWorker instance created by the constructor.
let mockWorkerInstance: MockWorker;

beforeEach(() => {
  // Use a class so `new Worker(...)` works correctly.
  vi.stubGlobal(
    "Worker",
    class {
      constructor() {
        mockWorkerInstance = new MockWorker();
        return mockWorkerInstance as unknown;
      }
    },
  );
});

afterEach(() => {
  vi.restoreAllMocks();
});

/** Wait one microtask tick so async operations (like file.arrayBuffer()) complete. */
function tick(): Promise<void> {
  return new Promise((r) => setTimeout(r, 0));
}

/** Helper: init a BntoWorker and resolve the "ready" handshake. */
async function initWorker(): Promise<BntoWorker> {
  const bnto = new BntoWorker();
  const initPromise = bnto.init();
  mockWorkerInstance.simulateMessage({ type: "ready", version: "0.1.0" });
  await initPromise;
  return bnto;
}

/** Find the most recent "process" message in mock.calls. */
function findProcessCall() {
  const calls = mockWorkerInstance.postMessage.mock.calls;
  for (let i = calls.length - 1; i >= 0; i--) {
    if (calls[i][0]?.type === "process") return calls[i][0];
  }
  return null;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("BntoWorker", () => {
  describe("init", () => {
    it("sends an init request to the worker", async () => {
      const bnto = new BntoWorker();
      const initPromise = bnto.init();

      expect(mockWorkerInstance.postMessage).toHaveBeenCalledWith({
        type: "init",
      });

      mockWorkerInstance.simulateMessage({
        type: "ready",
        version: "0.1.0",
      });

      const version = await initPromise;
      expect(version).toBe("0.1.0");
    });

    it("deduplicates concurrent init() calls", async () => {
      const bnto = new BntoWorker();
      const p1 = bnto.init();
      const p2 = bnto.init();

      mockWorkerInstance.simulateMessage({
        type: "ready",
        version: "0.1.0",
      });

      const [v1, v2] = await Promise.all([p1, p2]);
      expect(v1).toBe("0.1.0");
      expect(v2).toBe("0.1.0");
      expect(mockWorkerInstance.postMessage).toHaveBeenCalledTimes(1);
    });

    it("rejects on worker error during init", async () => {
      const bnto = new BntoWorker();
      const initPromise = bnto.init();

      mockWorkerInstance.simulateMessage({
        type: "worker-error",
        message: "WASM load failed",
      });

      await expect(initPromise).rejects.toThrow("WASM load failed");
    });

    it("rejects on worker-level onerror", async () => {
      const bnto = new BntoWorker();
      const initPromise = bnto.init();

      mockWorkerInstance.simulateError("Script error");

      await expect(initPromise).rejects.toThrow("Worker error: Script error");
    });
  });

  describe("isReady", () => {
    it("returns false before init", () => {
      const bnto = new BntoWorker();
      expect(bnto.isReady).toBe(false);
    });

    it("returns true after successful init", async () => {
      const bnto = await initWorker();
      expect(bnto.isReady).toBe(true);
    });
  });

  describe("processFile", () => {
    it("throws if not initialized", async () => {
      const bnto = new BntoWorker();
      const file = new File(["test"], "test.jpg", { type: "image/jpeg" });

      await expect(
        bnto.processFile(file, "compress-images"),
      ).rejects.toThrow("BntoWorker not initialized");
    });

    it("sends a process request and resolves with the result", async () => {
      const bnto = await initWorker();

      const file = new File(["test data"], "photo.jpg", {
        type: "image/jpeg",
      });
      const resultPromise = bnto.processFile(file, "compress-images", {
        quality: 80,
      });

      // Wait for file.arrayBuffer() to resolve so postMessage fires.
      await tick();

      const request = findProcessCall();
      expect(request).not.toBeNull();
      expect(request.type).toBe("process");
      expect(request.filename).toBe("photo.jpg");
      expect(request.mimeType).toBe("image/jpeg");
      expect(request.nodeType).toBe("compress-images");
      expect(request.params).toEqual({ quality: 80 });

      // Simulate the worker responding with result.
      const outputData = new ArrayBuffer(4);
      mockWorkerInstance.simulateMessage({
        type: "result",
        id: request.id,
        data: outputData,
        filename: "photo-compressed.jpg",
        mimeType: "image/jpeg",
        metadata: { ratio: 0.5 },
      });

      const result = await resultPromise;
      expect(result.filename).toBe("photo-compressed.jpg");
      expect(result.mimeType).toBe("image/jpeg");
      expect(result.metadata).toEqual({ ratio: 0.5 });
      expect(result.blob).toBeInstanceOf(Blob);
      expect(result.blob.size).toBe(4);
    });

    it("forwards progress callbacks", async () => {
      const bnto = await initWorker();

      const file = new File(["test"], "test.jpg", { type: "image/jpeg" });
      const onProgress = vi.fn();
      const resultPromise = bnto.processFile(
        file,
        "compress-images",
        {},
        onProgress,
      );

      await tick();

      const request = findProcessCall();
      expect(request).not.toBeNull();

      // Simulate progress updates.
      mockWorkerInstance.simulateMessage({
        type: "progress",
        id: request.id,
        percent: 25,
        message: "Decoding...",
      });
      mockWorkerInstance.simulateMessage({
        type: "progress",
        id: request.id,
        percent: 75,
        message: "Compressing...",
      });

      expect(onProgress).toHaveBeenCalledTimes(2);
      expect(onProgress).toHaveBeenCalledWith(25, "Decoding...");
      expect(onProgress).toHaveBeenCalledWith(75, "Compressing...");

      // Complete the request.
      mockWorkerInstance.simulateMessage({
        type: "result",
        id: request.id,
        data: new ArrayBuffer(2),
        filename: "test-compressed.jpg",
        mimeType: "image/jpeg",
        metadata: {},
      });
      await resultPromise;
    });

    it("rejects on error response", async () => {
      const bnto = await initWorker();

      const file = new File(["bad"], "bad.xyz", {
        type: "application/octet-stream",
      });
      const resultPromise = bnto.processFile(file, "compress-images");

      await tick();

      const request = findProcessCall();
      expect(request).not.toBeNull();

      mockWorkerInstance.simulateMessage({
        type: "error",
        id: request.id,
        message: "Unsupported format",
      });

      await expect(resultPromise).rejects.toThrow("Unsupported format");
    });
  });

  describe("terminate", () => {
    it("terminates the worker and rejects pending requests", async () => {
      const bnto = await initWorker();

      const file = new File(["test"], "test.jpg", { type: "image/jpeg" });
      const resultPromise = bnto.processFile(file, "compress-images");

      // Wait for the process message to be queued before terminating.
      await tick();

      bnto.terminate();

      expect(mockWorkerInstance.terminate).toHaveBeenCalled();
      expect(bnto.isReady).toBe(false);
      await expect(resultPromise).rejects.toThrow("Worker terminated");
    });

    it("is safe to call multiple times", async () => {
      const bnto = await initWorker();
      bnto.terminate();
      bnto.terminate(); // Should not throw.
      expect(bnto.isReady).toBe(false);
    });
  });
});
