import { describe, it, expect, vi, beforeEach } from "vitest";
import { createBrowserExecutionService } from "./browserExecutionService";
import { EXECUTION_STORE } from "./executionInstance";
import type { BrowserEngine, BrowserFileResult, BrowserFileProgressInput } from "../types/browser";
import type { PipelineDefinition } from "../engine/types";

// ---------------------------------------------------------------------------
// Shared definitions (replaces slug-based calls)
// ---------------------------------------------------------------------------

const COMPRESS_DEFINITION: PipelineDefinition = {
  nodes: [
    { id: "input", type: "input", params: {} },
    { id: "process", type: "compress-images", params: {} },
    { id: "output", type: "output", params: {} },
  ],
};

function compressDef(params: Record<string, unknown> = {}): PipelineDefinition {
  return {
    nodes: [
      { id: "input", type: "input", params: {} },
      { id: "process", type: "compress-images", params },
      { id: "output", type: "output", params: {} },
    ],
  };
}

// ---------------------------------------------------------------------------
// Mock engine
// ---------------------------------------------------------------------------

function createMockEngine(overrides: Partial<BrowserEngine> = {}): BrowserEngine {
  return {
    init: vi.fn().mockResolvedValue(undefined),
    processFile: vi.fn().mockResolvedValue({
      blob: new Blob(["compressed"], { type: "image/jpeg" }),
      filename: "photo-compressed.jpg",
      mimeType: "image/jpeg",
      metadata: { compressionRatio: 0.48 },
    } satisfies BrowserFileResult),
    terminate: vi.fn(),
    get isReady() {
      return true;
    },
    ...overrides,
  };
}

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
  let mockEngine: BrowserEngine;

  beforeEach(() => {
    mockEngine = createMockEngine();
    service = createBrowserExecutionService(mockEngine);
  });

  describe("isCapable", () => {
    it("returns true for compress-images", () => {
      expect(service.isCapable("compress-images")).toBe(true);
    });

    it("returns true for all Tier 1 slugs", () => {
      expect(service.isCapable("resize-images")).toBe(true);
      expect(service.isCapable("clean-csv")).toBe(true);
      expect(service.isCapable("rename-files")).toBe(true);
    });

    it("returns false for unknown slugs", () => {
      expect(service.isCapable("unknown")).toBe(false);
    });
  });

  describe("hasImplementation", () => {
    it("returns true for compress-images", () => {
      expect(service.hasImplementation("compress-images")).toBe(true);
    });

    it("returns false for slugs without implementations", () => {
      expect(service.hasImplementation("unknown")).toBe(false);
    });
  });

  describe("getCapableSlugs", () => {
    it("returns array including compress-images", () => {
      expect(service.getCapableSlugs()).toContain("compress-images");
    });
  });

  // ─────────────────────────────────────────────────────────────────
  // runPipeline — the single execution method
  // ─────────────────────────────────────────────────────────────────

  describe("runPipeline", () => {
    it("initializes the engine before processing", async () => {
      await service.runPipeline(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);
      expect(mockEngine.init).toHaveBeenCalledOnce();
    });

    it("calls processFile with correct node type and params", async () => {
      const files = [new File(["data"], "test.jpg")];
      await service.runPipeline(compressDef({ quality: 80 }), files);

      expect(mockEngine.processFile).toHaveBeenCalledOnce();
      expect(mockEngine.processFile).toHaveBeenCalledWith(
        expect.any(File),
        "compress-images",
        { quality: 80 },
        expect.any(Function),
      );
    });

    it("returns results from the engine", async () => {
      const expectedResult: BrowserFileResult = {
        blob: new Blob(["out"], { type: "image/jpeg" }),
        filename: "output.jpg",
        mimeType: "image/jpeg",
        metadata: { ratio: 0.5 },
      };

      const engine = createMockEngine({ processFile: vi.fn().mockResolvedValue(expectedResult) });
      const svc = createBrowserExecutionService(engine);
      const results = await svc.runPipeline(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);

      expect(results).toHaveLength(1);
      expect(results[0].filename).toBe("output.jpg");
      expect(results[0].mimeType).toBe("image/jpeg");
      expect(results[0].metadata).toEqual({ ratio: 0.5 });
    });

    it("handles multi-node pipeline", async () => {
      const def: PipelineDefinition = {
        nodes: [
          { id: "in", type: "input", params: {} },
          { id: "resize", type: "resize-images", params: { width: 800 } },
          { id: "compress", type: "compress-images", params: { quality: 75 } },
          { id: "out", type: "output", params: {} },
        ],
      };

      const results = await service.runPipeline(def, [new File(["data"], "test.jpg")]);
      expect(results).toHaveLength(1);
      expect(mockEngine.processFile).toHaveBeenCalledTimes(2);
    });

    it("forwards progress updates", async () => {
      const engine = createMockEngine({
        processFile: vi
          .fn()
          .mockImplementation(
            async (
              _file: File,
              _type: string,
              _params: Record<string, unknown>,
              onProgress?: (p: number, m: string) => void,
            ) => {
              onProgress?.(50, "Processing...");
              onProgress?.(100, "Done");
              return {
                blob: new Blob(["out"], { type: "image/jpeg" }),
                filename: "out.jpg",
                mimeType: "image/jpeg",
                metadata: {},
              } satisfies BrowserFileResult;
            },
          ),
      });
      const svc = createBrowserExecutionService(engine);

      const updates: BrowserFileProgressInput[] = [];
      await svc.runPipeline(
        COMPRESS_DEFINITION,
        [new File(["a"], "a.jpg"), new File(["b"], "b.jpg")],
        (p) => updates.push({ ...p }),
      );

      expect(updates.length).toBeGreaterThanOrEqual(2);
      expect(updates[0]).toEqual({
        fileIndex: 0,
        totalFiles: 2,
        percent: 50,
        message: "Processing...",
      });
    });

    it("does not crash when no progress callback provided", async () => {
      const results = await service.runPipeline(COMPRESS_DEFINITION, [
        new File(["data"], "test.jpg"),
      ]);
      expect(results).toHaveLength(1);
    });

    it("returns empty array for empty files", async () => {
      const results = await service.runPipeline(COMPRESS_DEFINITION, []);
      expect(results).toEqual([]);
      expect(mockEngine.processFile).not.toHaveBeenCalled();
    });

    it("does not fire progress for empty file array", async () => {
      const updates: BrowserFileProgressInput[] = [];
      await service.runPipeline(COMPRESS_DEFINITION, [], (p) => updates.push({ ...p }));
      expect(updates).toHaveLength(0);
    });
  });

  describe("runPipeline — engine init failure", () => {
    it("propagates init() error without masking it", async () => {
      const engine = createMockEngine({
        init: vi.fn().mockRejectedValue(new Error("WASM module failed to load")),
      });
      const svc = createBrowserExecutionService(engine);

      await expect(
        svc.runPipeline(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]),
      ).rejects.toThrow("WASM module failed to load");
      expect(engine.processFile).not.toHaveBeenCalled();
    });

    it("propagates non-Error init failures", async () => {
      const engine = createMockEngine({
        init: vi.fn().mockRejectedValue("string error from WASM"),
      });
      const svc = createBrowserExecutionService(engine);

      await expect(
        svc.runPipeline(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]),
      ).rejects.toBe("string error from WASM");
    });

    it("still calls init even if engine was previously used", async () => {
      const initFn = vi
        .fn()
        .mockResolvedValueOnce(undefined)
        .mockRejectedValueOnce(new Error("WASM out of memory"));
      const engine = createMockEngine({ init: initFn });
      const svc = createBrowserExecutionService(engine);

      await svc.runPipeline(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);
      expect(initFn).toHaveBeenCalledTimes(1);

      await expect(
        svc.runPipeline(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]),
      ).rejects.toThrow("WASM out of memory");
      expect(initFn).toHaveBeenCalledTimes(2);
      expect(engine.processFile).toHaveBeenCalledTimes(1);
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

    it("two instances don't interfere with each other", async () => {
      const a = service.createExecution();
      const b = service.createExecution();

      await a.run(COMPRESS_DEFINITION, [new File(["a"], "a.jpg")]);
      expect(getStore(a).getState().status).toBe("completed");
      expect(getStore(b).getState().status).toBe("idle");

      await b.run(COMPRESS_DEFINITION, [new File(["b"], "b.jpg")]);
      expect(getStore(a).getState().status).toBe("completed");
      expect(getStore(b).getState().status).toBe("completed");
    });

    it("instance reset only resets its own store", async () => {
      const a = service.createExecution();
      const b = service.createExecution();

      await a.run(COMPRESS_DEFINITION, [new File(["a"], "a.jpg")]);
      await b.run(COMPRESS_DEFINITION, [new File(["b"], "b.jpg")]);
      a.reset();

      expect(getStore(a).getState().status).toBe("idle");
      expect(getStore(b).getState().status).toBe("completed");
    });
  });

  describe("createExecution — success lifecycle", () => {
    it("transitions store through idle → processing → completed", async () => {
      const expectedResult: BrowserFileResult = {
        blob: new Blob(["out"], { type: "image/jpeg" }),
        filename: "output.jpg",
        mimeType: "image/jpeg",
        metadata: { ratio: 0.5 },
      };

      const engine = createMockEngine({ processFile: vi.fn().mockResolvedValue(expectedResult) });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      expect(getStore(instance).getState().status).toBe("idle");
      await instance.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);

      const state = getStore(instance).getState();
      expect(state.status).toBe("completed");
      expect(state.results).toHaveLength(1);
      expect(state.results[0].filename).toBe("output.jpg");
      expect(state.id).toBeTruthy();
      expect(state.startedAt).toBeTypeOf("number");
      expect(state.completedAt).toBeTypeOf("number");
      expect(state.fileProgress).toBeNull();
    });

    it("generates a unique execution ID", async () => {
      const instance = service.createExecution();
      await instance.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);

      expect(getStore(instance).getState().id).toBeTruthy();
      expect(getStore(instance).getState().id.length).toBeGreaterThan(0);
    });
  });

  describe("createExecution — progress updates", () => {
    it("updates store with progress during execution", async () => {
      const progressSnapshots: Array<{ fileIndex: number; percent: number }> = [];

      const engine = createMockEngine({
        processFile: vi
          .fn()
          .mockImplementation(
            async (
              _file: File,
              _type: string,
              _params: Record<string, unknown>,
              onProgress?: (p: number, m: string) => void,
            ) => {
              onProgress?.(50, "Compressing...");
              const s = getStore(instance).getState();
              progressSnapshots.push({
                fileIndex: s.fileProgress?.fileIndex ?? -1,
                percent: s.fileProgress?.percent ?? -1,
              });
              onProgress?.(100, "Done");
              return {
                blob: new Blob(["out"], { type: "image/jpeg" }),
                filename: "out.jpg",
                mimeType: "image/jpeg",
                metadata: {},
              } satisfies BrowserFileResult;
            },
          ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      await instance.run(COMPRESS_DEFINITION, [new File(["a"], "a.jpg")]);

      expect(progressSnapshots).toHaveLength(1);
      expect(progressSnapshots[0]).toEqual({ fileIndex: 0, percent: 50 });
      expect(getStore(instance).getState().fileProgress).toBeNull();
    });
  });

  describe("createExecution — failure lifecycle", () => {
    it("transitions store through idle → processing → failed on error", async () => {
      const engine = createMockEngine({
        processFile: vi.fn().mockRejectedValue(new Error("Corrupt JPEG")),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      await instance.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);

      const state = getStore(instance).getState();
      expect(state.status).toBe("failed");
      expect(state.error).toBe("Corrupt JPEG");
      expect(state.completedAt).toBeTypeOf("number");
      expect(state.fileProgress).toBeNull();
    });

    it("handles non-Error thrown values", async () => {
      const engine = createMockEngine({ processFile: vi.fn().mockRejectedValue("string error") });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      await instance.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);

      expect(getStore(instance).getState().status).toBe("failed");
      expect(getStore(instance).getState().error).toBe("Processing failed");
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

    it("aborts progress updates during in-flight execution", async () => {
      let resolveProcessFile: (value: BrowserFileResult) => void;
      const processPromise = new Promise<BrowserFileResult>((resolve) => {
        resolveProcessFile = resolve;
      });

      const engine = createMockEngine({
        processFile: vi
          .fn()
          .mockImplementation(
            async (
              _file: File,
              _type: string,
              _params: Record<string, unknown>,
              onProgress?: (p: number, m: string) => void,
            ) => {
              onProgress?.(50, "Processing...");
              return processPromise;
            },
          ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      const runPromise = instance.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);
      await new Promise((r) => setTimeout(r, 0));

      instance.reset();
      expect(getStore(instance).getState().status).toBe("idle");

      resolveProcessFile!({
        blob: new Blob(["out"], { type: "image/jpeg" }),
        filename: "out.jpg",
        mimeType: "image/jpeg",
        metadata: {},
      });
      await runPromise;

      expect(getStore(instance).getState().status).toBe("idle");
    });
  });

  // ─────────────────────────────────────────────────────────────────
  // Progress throttle behavior
  // ─────────────────────────────────────────────────────────────────

  describe("createExecution — progress throttle", () => {
    it("first progress update always passes through", async () => {
      const engine = createMockEngine({
        processFile: vi
          .fn()
          .mockImplementation(
            async (
              _file: File,
              _type: string,
              _params: Record<string, unknown>,
              onProgress?: (p: number, m: string) => void,
            ) => {
              onProgress?.(10, "First");
              return {
                blob: new Blob(["out"], { type: "image/jpeg" }),
                filename: "out.jpg",
                mimeType: "image/jpeg",
                metadata: {},
              } satisfies BrowserFileResult;
            },
          ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      await instance.run(COMPRESS_DEFINITION, [new File(["a"], "a.jpg")]);
      expect(getStore(instance).getState().status).toBe("completed");
    });

    it("100% completion always passes through regardless of throttle", async () => {
      const engine = createMockEngine({
        processFile: vi
          .fn()
          .mockImplementation(
            async (
              _file: File,
              _type: string,
              _params: Record<string, unknown>,
              onProgress?: (p: number, m: string) => void,
            ) => {
              for (let i = 10; i <= 100; i += 10) onProgress?.(i, `${i}%`);
              return {
                blob: new Blob(["out"], { type: "image/jpeg" }),
                filename: "out.jpg",
                mimeType: "image/jpeg",
                metadata: {},
              } satisfies BrowserFileResult;
            },
          ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();
      const store = getStore(instance);

      const originalProgress = store.getState().progress;
      const spy = vi.fn(originalProgress.bind(store.getState()));
      store.setState({ progress: spy });

      await instance.run(COMPRESS_DEFINITION, [new File(["a"], "a.jpg")]);

      const percents = spy.mock.calls.map((c: [{ percent: number }]) => c[0].percent);
      expect(percents[0]).toBe(10);
      expect(percents[percents.length - 1]).toBe(100);
    });

    it("new file transition always passes through", async () => {
      let callCount = 0;
      const engine = createMockEngine({
        processFile: vi
          .fn()
          .mockImplementation(
            async (
              _file: File,
              _type: string,
              _params: Record<string, unknown>,
              onProgress?: (p: number, m: string) => void,
            ) => {
              const idx = callCount++;
              onProgress?.(100, `File ${idx + 1} done`);
              return {
                blob: new Blob(["out"], { type: "image/jpeg" }),
                filename: `out-${idx}.jpg`,
                mimeType: "image/jpeg",
                metadata: {},
              } satisfies BrowserFileResult;
            },
          ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();
      const store = getStore(instance);

      const progressHistory: Array<{ fileIndex: number; percent: number }> = [];
      const originalProgress = store.getState().progress;
      store.setState({
        progress: (p: BrowserFileProgressInput) => {
          progressHistory.push({ fileIndex: p.fileIndex, percent: p.percent });
          originalProgress(p);
        },
      });

      await instance.run(COMPRESS_DEFINITION, [new File(["a"], "a.jpg"), new File(["b"], "b.jpg")]);

      const fileIndexes = progressHistory.map((p) => p.fileIndex);
      expect(fileIndexes).toContain(0);
      expect(fileIndexes).toContain(1);
    });
  });

  // ─────────────────────────────────────────────────────────────────
  // Edge cases
  // ─────────────────────────────────────────────────────────────────

  describe("createExecution — edge cases", () => {
    it("run with empty file array completes with empty results", async () => {
      const instance = service.createExecution();
      await instance.run(COMPRESS_DEFINITION, []);

      expect(getStore(instance).getState().status).toBe("completed");
      expect(getStore(instance).getState().results).toEqual([]);
    });

    it("run with large batch (100 files) tracks progress correctly", async () => {
      const fileCount = 100;
      const engine = createMockEngine({
        processFile: vi
          .fn()
          .mockImplementation(
            async (
              file: File,
              _type: string,
              _params: Record<string, unknown>,
              onProgress?: (p: number, m: string) => void,
            ) => {
              onProgress?.(100, "Done");
              return {
                blob: new Blob(["data"], { type: "image/jpeg" }),
                filename: file.name.replace(".jpg", "-out.jpg"),
                mimeType: "image/jpeg",
                metadata: {},
              } satisfies BrowserFileResult;
            },
          ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      const files = Array.from(
        { length: fileCount },
        (_, i) => new File([`data-${i}`], `file-${i}.jpg`),
      );
      await instance.run(COMPRESS_DEFINITION, files);

      expect(getStore(instance).getState().status).toBe("completed");
      expect(getStore(instance).getState().results).toHaveLength(fileCount);
    });
  });

  describe("createExecution — consecutive executions", () => {
    it("clears prior state when starting a new run", async () => {
      const instance = service.createExecution();

      await instance.run(COMPRESS_DEFINITION, [new File(["data"], "test.jpg")]);
      expect(getStore(instance).getState().status).toBe("completed");
      expect(getStore(instance).getState().results).toHaveLength(1);

      await instance.run(COMPRESS_DEFINITION, [new File(["data2"], "test2.jpg")]);
      expect(getStore(instance).getState().status).toBe("completed");
    });
  });
});
