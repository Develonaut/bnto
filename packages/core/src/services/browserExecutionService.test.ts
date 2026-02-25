import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { createBrowserExecutionService } from "./browserExecutionService";
import {
  registerBrowserEngine,
  getBrowserEngine,
} from "../adapters/browser/engineRegistry";
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

    it("returns false for slugs without browser implementations", () => {
      registerBrowserEngine(createMockEngine());
      expect(service.isCapable("resize-images")).toBe(false);
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

  describe("getCapableSlugs", () => {
    it("returns array including compress-images", () => {
      expect(service.getCapableSlugs()).toContain("compress-images");
    });
  });
});
