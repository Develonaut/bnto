import { describe, it, expect, vi, beforeEach } from "vitest";
import { createBrowserExecutionService } from "./browserExecutionService";
import { EXECUTION_STORE } from "./executionInstance";
import type {
  BrowserEngine,
  BrowserFileResult,
  BrowserFileProgressInput,
} from "../types/browser";

// ---------------------------------------------------------------------------
// Mock engine
// ---------------------------------------------------------------------------

function createMockEngine(
  overrides: Partial<BrowserEngine> = {},
): BrowserEngine {
  return {
    init: vi.fn().mockResolvedValue(undefined),
    processFile: vi.fn().mockResolvedValue({
      blob: new Blob(["compressed"], { type: "image/jpeg" }),
      filename: "photo-compressed.jpg",
      mimeType: "image/jpeg",
      metadata: { compressionRatio: 0.48 },
    } satisfies BrowserFileResult),
    processFiles: vi.fn().mockResolvedValue([
      {
        blob: new Blob(["compressed"], { type: "image/jpeg" }),
        filename: "photo-compressed.jpg",
        mimeType: "image/jpeg",
        metadata: { compressionRatio: 0.48 },
      } satisfies BrowserFileResult,
    ]),
    terminate: vi.fn(),
    get isReady() {
      return true;
    },
    ...overrides,
  };
}

/** Helper — access the opaque Zustand store on an ExecutionInstance. */
function getStore(instance: ReturnType<ReturnType<typeof createBrowserExecutionService>["createExecution"]>) {
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

  describe("execute", () => {
    it("throws for unknown slug", async () => {
      await expect(
        service.execute("unknown-slug", [new File([""], "test.jpg")]),
      ).rejects.toThrow('No browser implementation for slug "unknown-slug"');
    });

    it("initializes the engine before processing", async () => {
      await service.execute("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      expect(mockEngine.init).toHaveBeenCalledOnce();
    });

    it("calls processFiles with correct arguments", async () => {
      const files = [new File(["data"], "test.jpg")];
      const params = { quality: 80 };
      const onProgress = vi.fn();

      await service.execute("compress-images", files, params, onProgress);

      expect(mockEngine.processFiles).toHaveBeenCalledWith(
        files,
        "compress-images",
        params,
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

      const engine = createMockEngine({
        processFiles: vi.fn().mockResolvedValue([expectedResult]),
      });
      const svc = createBrowserExecutionService(engine);

      const results = await svc.execute("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      expect(results).toEqual([expectedResult]);
    });

    it("forwards progress updates with totalFiles", async () => {
      const engine = createMockEngine({
        processFiles: vi.fn().mockImplementation(
          async (
            _files: File[],
            _nodeType: string,
            _params: Record<string, unknown>,
            onProgress?: (
              fileIndex: number,
              percent: number,
              message: string,
            ) => void,
          ) => {
            onProgress?.(0, 50, "Processing...");
            onProgress?.(0, 100, "Done");
            return [];
          },
        ),
      });
      const svc = createBrowserExecutionService(engine);

      const progressUpdates: BrowserFileProgressInput[] = [];
      const files = [
        new File(["a"], "a.jpg"),
        new File(["b"], "b.jpg"),
      ];

      await svc.execute("compress-images", files, {}, (progress) => {
        progressUpdates.push({ ...progress });
      });

      expect(progressUpdates).toHaveLength(2);
      expect(progressUpdates[0]).toEqual({
        fileIndex: 0,
        totalFiles: 2,
        percent: 50,
        message: "Processing...",
      });
      expect(progressUpdates[1]).toEqual({
        fileIndex: 0,
        totalFiles: 2,
        percent: 100,
        message: "Done",
      });
    });

    it("does not forward progress when no callback provided", async () => {
      await service.execute("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      expect(mockEngine.processFiles).toHaveBeenCalledWith(
        expect.anything(),
        expect.anything(),
        expect.anything(),
        undefined,
      );
    });
  });

  describe("execute — engine init failure", () => {
    it("propagates init() error without masking it", async () => {
      const engine = createMockEngine({
        init: vi.fn().mockRejectedValue(new Error("WASM module failed to load")),
      });
      const svc = createBrowserExecutionService(engine);

      await expect(
        svc.execute("compress-images", [new File(["data"], "test.jpg")]),
      ).rejects.toThrow("WASM module failed to load");

      expect(engine.processFiles).not.toHaveBeenCalled();
    });

    it("propagates non-Error init failures", async () => {
      const engine = createMockEngine({
        init: vi.fn().mockRejectedValue("string error from WASM"),
      });
      const svc = createBrowserExecutionService(engine);

      await expect(
        svc.execute("compress-images", [new File(["data"], "test.jpg")]),
      ).rejects.toBe("string error from WASM");
    });

    it("still calls init even if engine was previously used", async () => {
      const initFn = vi
        .fn()
        .mockResolvedValueOnce(undefined)
        .mockRejectedValueOnce(new Error("WASM out of memory"));
      const engine = createMockEngine({ init: initFn });
      const svc = createBrowserExecutionService(engine);

      // First call succeeds.
      await svc.execute("compress-images", [
        new File(["data"], "test.jpg"),
      ]);
      expect(initFn).toHaveBeenCalledTimes(1);

      // Second call fails at init.
      await expect(
        svc.execute("compress-images", [new File(["data"], "test.jpg")]),
      ).rejects.toThrow("WASM out of memory");
      expect(initFn).toHaveBeenCalledTimes(2);
      expect(engine.processFiles).toHaveBeenCalledTimes(1); // only first call
    });
  });

  describe("getCapableSlugs", () => {
    it("returns array including compress-images", () => {
      expect(service.getCapableSlugs()).toContain("compress-images");
    });
  });

  // ─────────────────────────────────────────────────────────────────
  // Orchestration lifecycle via createExecution()
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

      await a.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      expect(getStore(a).getState().status).toBe("completed");
      expect(getStore(a).getState().results).toHaveLength(1);

      expect(getStore(b).getState().status).toBe("idle");
      expect(getStore(b).getState().results).toEqual([]);
    });

    it("two instances don't interfere with each other", async () => {
      const a = service.createExecution();
      const b = service.createExecution();

      await a.run("compress-images", [new File(["a"], "a.jpg")]);

      expect(getStore(a).getState().status).toBe("completed");
      expect(getStore(b).getState().status).toBe("idle");

      await b.run("compress-images", [new File(["b"], "b.jpg")]);

      expect(getStore(a).getState().status).toBe("completed");
      expect(getStore(b).getState().status).toBe("completed");
    });

    it("instance reset only resets its own store", async () => {
      const a = service.createExecution();
      const b = service.createExecution();

      await a.run("compress-images", [new File(["a"], "a.jpg")]);
      await b.run("compress-images", [new File(["b"], "b.jpg")]);

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

      const engine = createMockEngine({
        processFiles: vi.fn().mockResolvedValue([expectedResult]),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      expect(getStore(instance).getState().status).toBe("idle");

      await instance.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      const state = getStore(instance).getState();
      expect(state.status).toBe("completed");
      expect(state.results).toEqual([expectedResult]);
      expect(state.id).toBeTruthy();
      expect(state.startedAt).toBeTypeOf("number");
      expect(state.completedAt).toBeTypeOf("number");
      expect(state.fileProgress).toBeNull();
    });

    it("generates a unique execution ID", async () => {
      const instance = service.createExecution();
      await instance.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      expect(getStore(instance).getState().id).toBeTruthy();
      expect(getStore(instance).getState().id.length).toBeGreaterThan(0);
    });
  });

  describe("createExecution — progress updates", () => {
    it("updates store with progress during execution", async () => {
      const progressSnapshots: Array<{
        fileIndex: number;
        percent: number;
      }> = [];

      const engine = createMockEngine({
        processFiles: vi.fn().mockImplementation(
          async (
            _files: File[],
            _nodeType: string,
            _params: Record<string, unknown>,
            onProgress?: (
              fileIndex: number,
              percent: number,
              message: string,
            ) => void,
          ) => {
            onProgress?.(0, 50, "Compressing...");
            const s = getStore(instance).getState();
            progressSnapshots.push({
              fileIndex: s.fileProgress?.fileIndex ?? -1,
              percent: s.fileProgress?.percent ?? -1,
            });
            onProgress?.(0, 100, "Done");
            return [];
          },
        ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      await instance.run("compress-images", [new File(["a"], "a.jpg")]);

      expect(progressSnapshots).toHaveLength(1);
      expect(progressSnapshots[0]).toEqual({ fileIndex: 0, percent: 50 });

      expect(getStore(instance).getState().fileProgress).toBeNull();
    });
  });

  describe("createExecution — failure lifecycle", () => {
    it("transitions store through idle → processing → failed on error", async () => {
      const engine = createMockEngine({
        processFiles: vi
          .fn()
          .mockRejectedValue(new Error("Corrupt JPEG")),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      await instance.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      const state = getStore(instance).getState();
      expect(state.status).toBe("failed");
      expect(state.error).toBe("Corrupt JPEG");
      expect(state.completedAt).toBeTypeOf("number");
      expect(state.fileProgress).toBeNull();
    });

    it("handles non-Error thrown values", async () => {
      const engine = createMockEngine({
        processFiles: vi.fn().mockRejectedValue("string error"),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      await instance.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      expect(getStore(instance).getState().status).toBe("failed");
      expect(getStore(instance).getState().error).toBe("Processing failed");
    });

    it("fails on unknown slug", async () => {
      const instance = service.createExecution();
      await instance.run("unknown-slug", [
        new File(["data"], "test.jpg"),
      ]);

      const state = getStore(instance).getState();
      expect(state.status).toBe("failed");
      expect(state.error).toContain("No browser implementation");
    });
  });

  describe("createExecution — reset", () => {
    it("returns store to idle", async () => {
      const instance = service.createExecution();
      await instance.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);
      expect(getStore(instance).getState().status).toBe("completed");

      instance.reset();
      const state = getStore(instance).getState();
      expect(state.status).toBe("idle");
      expect(state.id).toBe("");
      expect(state.results).toEqual([]);
      expect(state.error).toBeUndefined();
    });

    it("aborts progress updates during in-flight execution", async () => {
      let resolveExecution: (value: BrowserFileResult[]) => void;
      const executionPromise = new Promise<BrowserFileResult[]>(
        (resolve) => {
          resolveExecution = resolve;
        },
      );

      const engine = createMockEngine({
        processFiles: vi.fn().mockImplementation(
          async (
            _files: File[],
            _nodeType: string,
            _params: Record<string, unknown>,
            onProgress?: (
              fileIndex: number,
              percent: number,
              message: string,
            ) => void,
          ) => {
            onProgress?.(0, 50, "Processing...");
            return executionPromise;
          },
        ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      const runPromise = instance.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      await new Promise((r) => setTimeout(r, 0));

      instance.reset();
      expect(getStore(instance).getState().status).toBe("idle");

      resolveExecution!([]);
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
        processFiles: vi.fn().mockImplementation(
          async (
            _files: File[],
            _nodeType: string,
            _params: Record<string, unknown>,
            onProgress?: (
              fileIndex: number,
              percent: number,
              message: string,
            ) => void,
          ) => {
            onProgress?.(0, 10, "First");
            return [];
          },
        ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      await instance.run("compress-images", [new File(["a"], "a.jpg")]);

      expect(getStore(instance).getState().status).toBe("completed");
    });

    it("100% completion always passes through regardless of throttle", async () => {
      const engine = createMockEngine({
        processFiles: vi.fn().mockImplementation(
          async (
            _files: File[],
            _nodeType: string,
            _params: Record<string, unknown>,
            onProgress?: (
              fileIndex: number,
              percent: number,
              message: string,
            ) => void,
          ) => {
            onProgress?.(0, 10, "Start");
            onProgress?.(0, 20, "20%");
            onProgress?.(0, 30, "30%");
            onProgress?.(0, 40, "40%");
            onProgress?.(0, 50, "50%");
            onProgress?.(0, 60, "60%");
            onProgress?.(0, 70, "70%");
            onProgress?.(0, 80, "80%");
            onProgress?.(0, 90, "90%");
            onProgress?.(0, 100, "Done");
            return [];
          },
        ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();
      const store = getStore(instance);

      const originalProgress = store.getState().progress;
      const spy = vi.fn(originalProgress.bind(store.getState()));
      store.setState({ progress: spy });

      await instance.run("compress-images", [new File(["a"], "a.jpg")]);

      const percents = spy.mock.calls.map(
        (c: [{ percent: number }]) => c[0].percent,
      );
      expect(percents[0]).toBe(10);
      expect(percents[percents.length - 1]).toBe(100);
    });

    it("new file transition always passes through", async () => {
      const engine = createMockEngine({
        processFiles: vi.fn().mockImplementation(
          async (
            _files: File[],
            _nodeType: string,
            _params: Record<string, unknown>,
            onProgress?: (
              fileIndex: number,
              percent: number,
              message: string,
            ) => void,
          ) => {
            onProgress?.(0, 100, "File 1 done");
            onProgress?.(1, 0, "File 2 start");
            onProgress?.(1, 100, "File 2 done");
            return [];
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

      await instance.run("compress-images", [
        new File(["a"], "a.jpg"),
        new File(["b"], "b.jpg"),
      ]);

      const fileIndexes = progressHistory.map((p) => p.fileIndex);
      expect(fileIndexes).toContain(0);
      expect(fileIndexes).toContain(1);
    });
  });

  // ─────────────────────────────────────────────────────────────────
  // Edge cases — empty files, boundary conditions
  // ─────────────────────────────────────────────────────────────────

  describe("execute — edge cases", () => {
    it("handles empty file array", async () => {
      const engine = createMockEngine({
        processFiles: vi.fn().mockResolvedValue([]),
      });
      const svc = createBrowserExecutionService(engine);

      const results = await svc.execute("compress-images", []);

      expect(results).toEqual([]);
      expect(engine.processFiles).toHaveBeenCalledWith(
        [],
        expect.anything(),
        expect.anything(),
        undefined,
      );
    });

    it("passes totalFiles=0 in progress for empty file array", async () => {
      const engine = createMockEngine({
        processFiles: vi.fn().mockImplementation(
          async (
            _files: File[],
            _nodeType: string,
            _params: Record<string, unknown>,
            onProgress?: (
              fileIndex: number,
              percent: number,
              message: string,
            ) => void,
          ) => {
            onProgress?.(0, 100, "Done");
            return [];
          },
        ),
      });
      const svc = createBrowserExecutionService(engine);

      const progressUpdates: BrowserFileProgressInput[] = [];
      await svc.execute("compress-images", [], {}, (progress) => {
        progressUpdates.push({ ...progress });
      });

      if (progressUpdates.length > 0) {
        expect(progressUpdates[0]?.totalFiles).toBe(0);
      }
    });
  });

  describe("createExecution — edge cases", () => {
    it("run with empty file array completes with empty results", async () => {
      const engine = createMockEngine({
        processFiles: vi.fn().mockResolvedValue([]),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      await instance.run("compress-images", []);

      expect(getStore(instance).getState().status).toBe("completed");
      expect(getStore(instance).getState().results).toEqual([]);
    });

    it("run with large batch (100 files) tracks progress correctly", async () => {
      const fileCount = 100;
      const results = Array.from({ length: fileCount }, (_, i) => ({
        blob: new Blob([`data-${i}`], { type: "image/jpeg" }),
        filename: `file-${i}.jpg`,
        mimeType: "image/jpeg",
        metadata: { index: i },
      })) satisfies BrowserFileResult[];

      const engine = createMockEngine({
        processFiles: vi.fn().mockImplementation(
          async (
            files: File[],
            _nodeType: string,
            _params: Record<string, unknown>,
            onProgress?: (
              fileIndex: number,
              percent: number,
              message: string,
            ) => void,
          ) => {
            for (let i = 0; i < files.length; i++) {
              onProgress?.(i, 100, `File ${i + 1} done`);
            }
            return results;
          },
        ),
      });
      const svc = createBrowserExecutionService(engine);
      const instance = svc.createExecution();

      const files = Array.from(
        { length: fileCount },
        (_, i) => new File([`data-${i}`], `file-${i}.jpg`),
      );

      await instance.run("compress-images", files);

      expect(getStore(instance).getState().status).toBe("completed");
      expect(getStore(instance).getState().results).toHaveLength(fileCount);
    });
  });

  describe("createExecution — consecutive executions", () => {
    it("clears prior state when starting a new run", async () => {
      const instance = service.createExecution();

      await instance.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);
      expect(getStore(instance).getState().status).toBe("completed");
      expect(getStore(instance).getState().results).toHaveLength(1);

      await instance.run("compress-images", [
        new File(["data2"], "test2.jpg"),
      ]);
      expect(getStore(instance).getState().status).toBe("completed");
    });
  });
});
