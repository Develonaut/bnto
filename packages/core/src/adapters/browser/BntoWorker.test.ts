import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { BntoWorker } from "./BntoWorker";
import type { WorkerResponse } from "./workerProtocol";

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
  // eslint-disable-next-line @typescript-eslint/no-explicit-any -- mock Worker constructor
  vi.stubGlobal("Worker", function (this: any) {
    mockWorkerInstance = new MockWorker();
    Object.assign(this, mockWorkerInstance);
    return mockWorkerInstance;
  } as any);
});

afterEach(() => {
  vi.restoreAllMocks();
});

/** Wait one microtask tick so async operations complete. */
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

/** Find the most recent "execute-pipeline" message in mock.calls. */
function findPipelineCall() {
  const calls = mockWorkerInstance.postMessage.mock.calls;
  for (let i = calls.length - 1; i >= 0; i--) {
    if (calls[i][0]?.type === "execute-pipeline") return calls[i][0];
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
        baseUrl: "",
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

  describe("executePipeline", () => {
    it("throws if not initialized", async () => {
      const bnto = new BntoWorker();

      await expect(bnto.executePipeline("{}", [])).rejects.toThrow("BntoWorker not initialized");
    });

    it("sends pipeline request and resolves with result", async () => {
      const bnto = await initWorker();

      const defJson = JSON.stringify({
        nodes: [
          { id: "in", type: "input", params: {} },
          { id: "proc", type: "compress-images", params: { quality: 80 } },
          { id: "out", type: "output", params: {} },
        ],
      });

      const file = new File(["test data"], "photo.jpg", { type: "image/jpeg" });
      const resultPromise = bnto.executePipeline(defJson, [file]);

      await tick();

      const request = findPipelineCall();
      expect(request).not.toBeNull();
      expect(request.type).toBe("execute-pipeline");
      expect(request.definitionJson).toBe(defJson);
      expect(request.files).toHaveLength(1);

      // Simulate pipeline result
      mockWorkerInstance.simulateMessage({
        type: "pipeline-result",
        id: request.id,
        files: [
          {
            name: "photo-compressed.jpg",
            data: new ArrayBuffer(4),
            mimeType: "image/jpeg",
            metadata: '{"ratio":0.5}',
          },
        ],
        durationMs: 100,
      });

      const result = await resultPromise;
      expect(result.files).toHaveLength(1);
      expect(result.files[0].name).toBe("photo-compressed.jpg");
      expect(result.durationMs).toBe(100);
    });

    it("forwards pipeline progress events", async () => {
      const bnto = await initWorker();

      const onEvent = vi.fn();
      const resultPromise = bnto.executePipeline("{}", [new File(["a"], "a.jpg")], onEvent);

      await tick();

      const request = findPipelineCall();

      mockWorkerInstance.simulateMessage({
        type: "pipeline-progress",
        id: request.id,
        eventJson: JSON.stringify({
          type: "NodeStarted",
          nodeId: "proc",
          nodeIndex: 0,
          totalNodes: 1,
          nodeType: "compress-images",
        }),
      });

      expect(onEvent).toHaveBeenCalledWith(
        expect.objectContaining({ type: "NodeStarted", nodeId: "proc" }),
      );

      mockWorkerInstance.simulateMessage({
        type: "pipeline-result",
        id: request.id,
        files: [],
        durationMs: 50,
      });
      await resultPromise;
    });

    it("rejects on pipeline error", async () => {
      const bnto = await initWorker();

      const resultPromise = bnto.executePipeline("{}", [new File(["a"], "a.jpg")]);

      await tick();

      const request = findPipelineCall();
      mockWorkerInstance.simulateMessage({
        type: "pipeline-error",
        id: request.id,
        message: "Invalid node type: unknown",
      });

      await expect(resultPromise).rejects.toThrow("Invalid node type: unknown");
    });
  });

  describe("terminate", () => {
    it("terminates the worker and rejects pending pipelines", async () => {
      const bnto = await initWorker();

      const resultPromise = bnto.executePipeline("{}", [new File(["a"], "a.jpg")]);
      await tick();

      bnto.terminate();

      expect(mockWorkerInstance.terminate).toHaveBeenCalled();
      expect(bnto.isReady).toBe(false);
      await expect(resultPromise).rejects.toThrow("Worker terminated");
    });

    it("is safe to call multiple times", async () => {
      const bnto = await initWorker();
      bnto.terminate();
      bnto.terminate();
      expect(bnto.isReady).toBe(false);
    });

    it("can be re-initialized after terminate", async () => {
      const bnto = await initWorker();
      bnto.terminate();
      expect(bnto.isReady).toBe(false);

      const reinitPromise = bnto.init();
      mockWorkerInstance.simulateMessage({ type: "ready", version: "0.2.0" });
      const version = await reinitPromise;

      expect(version).toBe("0.2.0");
      expect(bnto.isReady).toBe(true);
    });
  });
});
