import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { createBrowserExecutionService } from "./browserExecutionService";
import { registerBrowserEngine } from "../adapters/browser/engineRegistry";
import type {
  BrowserEngine,
  BrowserFileResult,
  BrowserFileProgress,
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

// ---------------------------------------------------------------------------
// Reset engine between tests
// ---------------------------------------------------------------------------

function clearEngine() {
  // Register null to reset (the type check is for testing only)
  registerBrowserEngine(null as unknown as BrowserEngine);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("browserExecutionService", () => {
  let service: ReturnType<typeof createBrowserExecutionService>;

  beforeEach(() => {
    service = createBrowserExecutionService();
    clearEngine();
  });

  afterEach(() => {
    clearEngine();
  });

  describe("isCapable", () => {
    it("returns false when no engine is registered", () => {
      expect(service.isCapable("compress-images")).toBe(false);
    });

    it("returns true for compress-images with engine registered", () => {
      registerBrowserEngine(createMockEngine());
      expect(service.isCapable("compress-images")).toBe(true);
    });

    it("returns true for all Tier 1 slugs with engine registered", () => {
      registerBrowserEngine(createMockEngine());
      expect(service.isCapable("resize-images")).toBe(true);
      expect(service.isCapable("clean-csv")).toBe(true);
      expect(service.isCapable("rename-files")).toBe(true);
    });

    it("returns false for unknown slugs", () => {
      registerBrowserEngine(createMockEngine());
      expect(service.isCapable("unknown")).toBe(false);
    });
  });

  describe("hasImplementation", () => {
    it("returns true for compress-images even without engine", () => {
      expect(service.hasImplementation("compress-images")).toBe(true);
    });

    it("returns false for slugs without implementations", () => {
      expect(service.hasImplementation("unknown")).toBe(false);
    });
  });

  describe("registerEngine / hasEngine", () => {
    it("starts without an engine", () => {
      expect(service.hasEngine()).toBe(false);
    });

    it("registers and detects the engine", () => {
      service.registerEngine(createMockEngine());
      expect(service.hasEngine()).toBe(true);
    });
  });

  describe("execute", () => {
    it("throws if no engine is registered", async () => {
      await expect(
        service.execute("compress-images", [new File([""], "test.jpg")]),
      ).rejects.toThrow("No browser engine registered");
    });

    it("throws for unknown slug", async () => {
      registerBrowserEngine(createMockEngine());
      await expect(
        service.execute("unknown-slug", [new File([""], "test.jpg")]),
      ).rejects.toThrow('No browser implementation for slug "unknown-slug"');
    });

    it("initializes the engine before processing", async () => {
      const engine = createMockEngine();
      registerBrowserEngine(engine);

      await service.execute("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      expect(engine.init).toHaveBeenCalledOnce();
    });

    it("calls processFiles with correct arguments", async () => {
      const engine = createMockEngine();
      registerBrowserEngine(engine);

      const files = [new File(["data"], "test.jpg")];
      const params = { quality: 80 };
      const onProgress = vi.fn();

      await service.execute("compress-images", files, params, onProgress);

      expect(engine.processFiles).toHaveBeenCalledWith(
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
      registerBrowserEngine(engine);

      const results = await service.execute("compress-images", [
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
      registerBrowserEngine(engine);

      const progressUpdates: BrowserFileProgress[] = [];
      const files = [
        new File(["a"], "a.jpg"),
        new File(["b"], "b.jpg"),
      ];

      await service.execute("compress-images", files, {}, (progress) => {
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
      const engine = createMockEngine();
      registerBrowserEngine(engine);

      // Should not throw
      await service.execute("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      // processFiles should have been called with undefined callback
      expect(engine.processFiles).toHaveBeenCalledWith(
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
      registerBrowserEngine(engine);

      await expect(
        service.execute("compress-images", [new File(["data"], "test.jpg")]),
      ).rejects.toThrow("WASM module failed to load");

      // processFiles should never be called when init fails.
      expect(engine.processFiles).not.toHaveBeenCalled();
    });

    it("propagates non-Error init failures", async () => {
      const engine = createMockEngine({
        init: vi.fn().mockRejectedValue("string error from WASM"),
      });
      registerBrowserEngine(engine);

      await expect(
        service.execute("compress-images", [new File(["data"], "test.jpg")]),
      ).rejects.toBe("string error from WASM");
    });

    it("still calls init even if engine was previously used", async () => {
      const initFn = vi
        .fn()
        .mockResolvedValueOnce(undefined)
        .mockRejectedValueOnce(new Error("WASM out of memory"));
      const engine = createMockEngine({ init: initFn });
      registerBrowserEngine(engine);

      // First call succeeds.
      await service.execute("compress-images", [
        new File(["data"], "test.jpg"),
      ]);
      expect(initFn).toHaveBeenCalledTimes(1);

      // Second call fails at init.
      await expect(
        service.execute("compress-images", [new File(["data"], "test.jpg")]),
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
  // Orchestration lifecycle: run() and reset()
  // ─────────────────────────────────────────────────────────────────

  describe("store ownership", () => {
    it("exposes a Zustand store", () => {
      expect(service.store).toBeDefined();
      expect(service.store.getState).toBeDefined();
      expect(service.store.getState().status).toBe("idle");
    });

    it("each service gets its own store instance", () => {
      const service2 = createBrowserExecutionService();
      expect(service.store).not.toBe(service2.store);
    });
  });

  describe("run — success lifecycle", () => {
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
      registerBrowserEngine(engine);

      // Before run
      expect(service.store.getState().status).toBe("idle");

      await service.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      // After run
      const state = service.store.getState();
      expect(state.status).toBe("completed");
      expect(state.results).toEqual([expectedResult]);
      expect(state.id).toBeTruthy();
      expect(state.startedAt).toBeTypeOf("number");
      expect(state.completedAt).toBeTypeOf("number");
      expect(state.fileProgress).toBeNull();
    });

    it("generates a unique execution ID", async () => {
      registerBrowserEngine(createMockEngine());

      await service.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      expect(service.store.getState().id).toBeTruthy();
      expect(service.store.getState().id.length).toBeGreaterThan(0);
    });
  });

  describe("run — progress updates", () => {
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
            // Capture store state after progress update
            const s = service.store.getState();
            progressSnapshots.push({
              fileIndex: s.fileProgress?.fileIndex ?? -1,
              percent: s.fileProgress?.percent ?? -1,
            });
            onProgress?.(0, 100, "Done");
            return [];
          },
        ),
      });
      registerBrowserEngine(engine);

      await service.run("compress-images", [new File(["a"], "a.jpg")]);

      expect(progressSnapshots).toHaveLength(1);
      expect(progressSnapshots[0]).toEqual({ fileIndex: 0, percent: 50 });

      // Progress is cleared after completion
      expect(service.store.getState().fileProgress).toBeNull();
    });
  });

  describe("run — failure lifecycle", () => {
    it("transitions store through idle → processing → failed on error", async () => {
      const engine = createMockEngine({
        processFiles: vi
          .fn()
          .mockRejectedValue(new Error("Corrupt JPEG")),
      });
      registerBrowserEngine(engine);

      await service.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      const state = service.store.getState();
      expect(state.status).toBe("failed");
      expect(state.error).toBe("Corrupt JPEG");
      expect(state.completedAt).toBeTypeOf("number");
      expect(state.fileProgress).toBeNull();
    });

    it("handles non-Error thrown values", async () => {
      const engine = createMockEngine({
        processFiles: vi.fn().mockRejectedValue("string error"),
      });
      registerBrowserEngine(engine);

      await service.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      expect(service.store.getState().status).toBe("failed");
      expect(service.store.getState().error).toBe("Processing failed");
    });

    it("fails on missing engine", async () => {
      // No engine registered
      await service.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      const state = service.store.getState();
      expect(state.status).toBe("failed");
      expect(state.error).toContain("No browser engine registered");
    });

    it("fails on unknown slug", async () => {
      registerBrowserEngine(createMockEngine());

      await service.run("unknown-slug", [
        new File(["data"], "test.jpg"),
      ]);

      const state = service.store.getState();
      expect(state.status).toBe("failed");
      expect(state.error).toContain("No browser implementation");
    });
  });

  describe("reset", () => {
    it("returns store to idle", async () => {
      registerBrowserEngine(createMockEngine());

      await service.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);
      expect(service.store.getState().status).toBe("completed");

      service.reset();
      const state = service.store.getState();
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
            // Emit one progress before abort
            onProgress?.(0, 50, "Processing...");

            // Reset is called externally while we wait
            return executionPromise;
          },
        ),
      });
      registerBrowserEngine(engine);

      // Start run (don't await — it's still in-flight)
      const runPromise = service.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);

      // Wait a tick for the progress callback to fire
      await new Promise((r) => setTimeout(r, 0));

      // Reset mid-execution
      service.reset();
      expect(service.store.getState().status).toBe("idle");

      // Resolve the engine — should NOT update store (aborted)
      resolveExecution!([]);
      await runPromise;

      // Store should still be idle (not completed)
      expect(service.store.getState().status).toBe("idle");
    });
  });

  describe("run — consecutive executions", () => {
    it("clears prior state when starting a new run", async () => {
      const engine = createMockEngine();
      registerBrowserEngine(engine);

      // First run succeeds
      await service.run("compress-images", [
        new File(["data"], "test.jpg"),
      ]);
      expect(service.store.getState().status).toBe("completed");
      expect(service.store.getState().results).toHaveLength(1);

      // Second run starts fresh
      await service.run("compress-images", [
        new File(["data2"], "test2.jpg"),
      ]);
      expect(service.store.getState().status).toBe("completed");
    });
  });
});
