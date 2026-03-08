import { describe, it, expect, vi, beforeEach } from "vitest";
import { createBrowserExecutionService } from "./browserExecutionService";
import { EXECUTION_STORE } from "./executionInstance";
import type { BrowserFileResult } from "../types/browser";
import type { PipelineDefinition } from "../types/pipeline";

// ---------------------------------------------------------------------------
// Shared definitions
// ---------------------------------------------------------------------------

const COMPRESS_DEFINITION: PipelineDefinition = {
  nodes: [
    { id: "input", type: "input", params: {} },
    { id: "process", type: "compress-images", params: {} },
    { id: "output", type: "output", params: {} },
  ],
};

// ---------------------------------------------------------------------------
// Mock BntoWorker at module level
// ---------------------------------------------------------------------------

// vi.hoisted ensures these are available when the hoisted vi.mock runs.
const { mockExecutePipeline, mockInit, mockTerminate } = vi.hoisted(() => ({
  mockExecutePipeline: vi.fn(),
  mockInit: vi.fn().mockResolvedValue("1.0.0"),
  mockTerminate: vi.fn(),
}));

vi.mock("../adapters/browser/BntoWorker", () => {
  class MockBntoWorker {
    init = mockInit;
    executePipeline = mockExecutePipeline;
    terminate = mockTerminate;
    get isReady() {
      return true;
    }
  }
  return { BntoWorker: MockBntoWorker };
});

/** Helper — access the opaque Zustand store on an ExecutionInstance. */
function getStore(
  instance: ReturnType<ReturnType<typeof createBrowserExecutionService>["createExecution"]>,
) {
  return instance[EXECUTION_STORE];
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("browserExecutionService", () => {
  let service: ReturnType<typeof createBrowserExecutionService>;

  beforeEach(() => {
    vi.clearAllMocks();

    // Default: return one file result
    mockExecutePipeline.mockResolvedValue({
      files: [
        {
          name: "photo-compressed.jpg",
          data: new ArrayBuffer(8),
          mimeType: "image/jpeg",
          metadata: JSON.stringify({ compressionRatio: 0.48 }),
        },
      ],
      durationMs: 100,
    });

    // Mock window for browser environment check
    vi.stubGlobal("window", {});
    service = createBrowserExecutionService();
  });

  // ─────────────────────────────────────────────────────────────────
  // runPipeline
  // ─────────────────────────────────────────────────────────────────

  describe("runPipeline", () => {
    it("initializes the worker before processing", async () => {
      await service.runPipeline(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);
      expect(mockInit).toHaveBeenCalledOnce();
    });

    it("calls executePipeline with serialized definition", async () => {
      const files = [new File(["data"], "test.jpg")];
      await service.runPipeline(COMPRESS_DEFINITION, files);

      expect(mockExecutePipeline).toHaveBeenCalledOnce();
      const [defJson, sentFiles] = mockExecutePipeline.mock.calls[0];
      expect(JSON.parse(defJson)).toEqual(COMPRESS_DEFINITION);
      expect(sentFiles).toHaveLength(1);
    });

    it("returns BrowserFileResult[] from WASM results", async () => {
      const results = await service.runPipeline(COMPRESS_DEFINITION, [
        new File(["data"], "test.jpg"),
      ]);

      expect(results).toHaveLength(1);
      expect(results[0].filename).toBe("photo-compressed.jpg");
      expect(results[0].mimeType).toBe("image/jpeg");
      expect(results[0].metadata).toEqual({ compressionRatio: 0.48 });
      expect(results[0].blob).toBeInstanceOf(Blob);
    });

    it("returns empty array for empty files", async () => {
      const results = await service.runPipeline(COMPRESS_DEFINITION, []);
      expect(results).toEqual([]);
      expect(mockExecutePipeline).not.toHaveBeenCalled();
    });

    it("propagates init() error without masking it", async () => {
      mockInit.mockRejectedValueOnce(new Error("WASM module failed to load"));

      await expect(
        service.runPipeline(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]),
      ).rejects.toThrow("WASM module failed to load");
      expect(mockExecutePipeline).not.toHaveBeenCalled();
    });

    it("handles results without metadata", async () => {
      mockExecutePipeline.mockResolvedValueOnce({
        files: [
          {
            name: "out.jpg",
            data: new ArrayBuffer(4),
            mimeType: "image/jpeg",
          },
        ],
        durationMs: 50,
      });

      const results = await service.runPipeline(COMPRESS_DEFINITION, [
        new File(["data"], "test.jpg"),
      ]);

      expect(results[0].metadata).toEqual({});
    });
  });

  // ─────────────────────────────────────────────────────────────────
  // createExecution — instance lifecycle
  // ─────────────────────────────────────────────────────────────────

  describe("createExecution — instance isolation", () => {
    it("returns an instance with an opaque store", () => {
      const instance = service.createExecution();
      const store = getStore(instance);
      expect(store).toBeDefined();
      expect(store.getState).toBeDefined();
      expect(store.getState().status).toBe("idle");
    });

    it("each instance gets a separate store", () => {
      const a = service.createExecution();
      const b = service.createExecution();
      expect(getStore(a)).not.toBe(getStore(b));
    });

    it("instance run does not affect other instances", async () => {
      const a = service.createExecution();
      const b = service.createExecution();

      await a.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);

      expect(getStore(a).getState().status).toBe("completed");
      expect(getStore(a).getState().results).toHaveLength(1);
      expect(getStore(b).getState().status).toBe("idle");
      expect(getStore(b).getState().results).toEqual([]);
    });
  });

  describe("createExecution — success lifecycle", () => {
    it("transitions store through idle → processing → completed", async () => {
      const instance = service.createExecution();

      expect(getStore(instance).getState().status).toBe("idle");
      await instance.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);

      const state = getStore(instance).getState();
      expect(state.status).toBe("completed");
      expect(state.results).toHaveLength(1);
      expect(state.results[0].filename).toBe("photo-compressed.jpg");
      expect(state.id).toBeTruthy();
      expect(state.startedAt).toBeTypeOf("number");
      expect(state.completedAt).toBeTypeOf("number");
    });

    it("run with empty file array completes with empty results", async () => {
      const instance = service.createExecution();
      await instance.run(COMPRESS_DEFINITION, []);

      expect(getStore(instance).getState().status).toBe("completed");
      expect(getStore(instance).getState().results).toEqual([]);
    });
  });

  describe("createExecution — failure lifecycle", () => {
    it("transitions store to failed on WASM error", async () => {
      mockExecutePipeline.mockRejectedValueOnce(new Error("Corrupt JPEG"));
      const instance = service.createExecution();

      await instance.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);

      const state = getStore(instance).getState();
      expect(state.status).toBe("failed");
      expect(state.error).toBe("Corrupt JPEG");
      expect(state.completedAt).toBeTypeOf("number");
    });
  });

  describe("createExecution — reset", () => {
    it("returns store to idle", async () => {
      const instance = service.createExecution();
      await instance.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);
      expect(getStore(instance).getState().status).toBe("completed");

      instance.reset();
      const state = getStore(instance).getState();
      expect(state.status).toBe("idle");
      expect(state.id).toBe("");
      expect(state.results).toEqual([]);
      expect(state.error).toBeUndefined();
    });
  });

  describe("createExecution — consecutive executions", () => {
    it("clears prior state when starting a new run", async () => {
      const instance = service.createExecution();

      await instance.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);
      expect(getStore(instance).getState().status).toBe("completed");

      await instance.run(COMPRESS_DEFINITION, [new File(["data2"], "test2.jpg")]);
      expect(getStore(instance).getState().status).toBe("completed");
    });
  });
});
