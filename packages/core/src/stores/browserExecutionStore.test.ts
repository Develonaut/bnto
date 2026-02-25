import { describe, it, expect, beforeEach } from "vitest";
import { createBrowserExecutionStore } from "./browserExecutionStore";
import type {
  BrowserFileProgress,
  BrowserFileResult,
} from "../types/browser";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const mockProgress: BrowserFileProgress = {
  fileIndex: 0,
  totalFiles: 2,
  percent: 50,
  message: "Compressing...",
};

const mockResult: BrowserFileResult = {
  blob: new Blob(["data"], { type: "image/jpeg" }),
  filename: "photo-compressed.jpg",
  mimeType: "image/jpeg",
  metadata: { compressionRatio: 0.48 },
};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("browserExecutionStore", () => {
  let store: ReturnType<typeof createBrowserExecutionStore>;

  beforeEach(() => {
    store = createBrowserExecutionStore();
  });

  describe("initial state", () => {
    it("starts idle with empty results", () => {
      const state = store.getState();
      expect(state.id).toBe("");
      expect(state.status).toBe("idle");
      expect(state.fileProgress).toBeNull();
      expect(state.results).toEqual([]);
    });
  });

  describe("start", () => {
    it("transitions from idle to processing", () => {
      store.getState().start("exec-1", 1000);
      const state = store.getState();

      expect(state.id).toBe("exec-1");
      expect(state.status).toBe("processing");
      expect(state.fileProgress).toBeNull();
      expect(state.results).toEqual([]);
      expect(state.startedAt).toBe(1000);
    });

    it("resets prior state when restarting", () => {
      // Put the store in a failed state first.
      store.getState().start("old", 100);
      store.getState().fail("previous error", 500);

      // Restart with a new execution.
      store.getState().start("new-exec", 2000);
      const state = store.getState();

      expect(state.status).toBe("processing");
      expect(state.id).toBe("new-exec");
      expect(state.error).toBeUndefined();
      expect(state.completedAt).toBeUndefined();
    });
  });

  describe("progress", () => {
    it("updates fileProgress on a processing state", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress(mockProgress);
      const state = store.getState();

      expect(state.fileProgress).toEqual(mockProgress);
      expect(state.status).toBe("processing");
    });

    it("preserves other state fields", () => {
      store.getState().start("test-id", 1234);
      store.getState().progress(mockProgress);
      const state = store.getState();

      expect(state.id).toBe("test-id");
      expect(state.startedAt).toBe(1234);
    });
  });

  describe("complete", () => {
    it("transitions from processing to completed", () => {
      store.getState().start("test-id", 1000);
      store.getState().complete([mockResult], 2000);
      const state = store.getState();

      expect(state.status).toBe("completed");
      expect(state.results).toEqual([mockResult]);
      expect(state.fileProgress).toBeNull();
      expect(state.completedAt).toBe(2000);
    });

    it("clears in-flight progress", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress(mockProgress);
      store.getState().complete([], 3000);

      expect(store.getState().fileProgress).toBeNull();
    });
  });

  describe("fail", () => {
    it("transitions from processing to failed", () => {
      store.getState().start("test-id", 1000);
      store.getState().fail("WASM crashed", 2000);
      const state = store.getState();

      expect(state.status).toBe("failed");
      expect(state.error).toBe("WASM crashed");
      expect(state.fileProgress).toBeNull();
      expect(state.completedAt).toBe(2000);
    });

    it("preserves the execution ID and start time", () => {
      store.getState().start("exec-99", 500);
      store.getState().fail("out of memory", 1500);
      const state = store.getState();

      expect(state.id).toBe("exec-99");
      expect(state.startedAt).toBe(500);
    });
  });

  describe("batch progress — fileIndex increments", () => {
    it("tracks fileIndex advancing through a batch", () => {
      store.getState().start("test-id", 1000);

      // File 0 starts.
      store.getState().progress({ fileIndex: 0, totalFiles: 3, percent: 50, message: "Compressing file 1..." });
      expect(store.getState().fileProgress?.fileIndex).toBe(0);
      expect(store.getState().fileProgress?.totalFiles).toBe(3);

      // File 0 completes.
      store.getState().progress({ fileIndex: 0, totalFiles: 3, percent: 100, message: "File 1 done" });
      expect(store.getState().fileProgress?.fileIndex).toBe(0);
      expect(store.getState().fileProgress?.percent).toBe(100);

      // File 1 starts.
      store.getState().progress({ fileIndex: 1, totalFiles: 3, percent: 25, message: "Compressing file 2..." });
      expect(store.getState().fileProgress?.fileIndex).toBe(1);
      expect(store.getState().fileProgress?.percent).toBe(25);

      // File 2 starts.
      store.getState().progress({ fileIndex: 2, totalFiles: 3, percent: 10, message: "Compressing file 3..." });
      expect(store.getState().fileProgress?.fileIndex).toBe(2);

      // Status remains processing throughout.
      expect(store.getState().status).toBe("processing");
    });

    it("complete clears fileProgress after batch finishes", () => {
      store.getState().start("test-id", 1000);

      // Simulate progress for file 2 of 3.
      store.getState().progress({ fileIndex: 2, totalFiles: 3, percent: 100, message: "Done" });
      expect(store.getState().fileProgress).not.toBeNull();

      // Complete the batch.
      store.getState().complete([mockResult, mockResult, mockResult], 5000);

      expect(store.getState().status).toBe("completed");
      expect(store.getState().fileProgress).toBeNull();
      expect(store.getState().results).toHaveLength(3);
    });

    it("fail during batch clears fileProgress", () => {
      store.getState().start("test-id", 1000);

      // Progress was at file 1 when failure occurs.
      store.getState().progress({ fileIndex: 1, totalFiles: 3, percent: 40, message: "Compressing..." });
      store.getState().fail("Corrupt image at file 2", 3000);

      expect(store.getState().status).toBe("failed");
      expect(store.getState().fileProgress).toBeNull();
      expect(store.getState().error).toBe("Corrupt image at file 2");
    });
  });

  describe("reset", () => {
    it("returns to idle from completed state", () => {
      store.getState().start("done", 1000);
      store.getState().complete([mockResult], 2000);
      store.getState().reset();
      const state = store.getState();

      expect(state.id).toBe("");
      expect(state.status).toBe("idle");
      expect(state.fileProgress).toBeNull();
      expect(state.results).toEqual([]);
      expect(state.error).toBeUndefined();
      expect(state.startedAt).toBeUndefined();
      expect(state.completedAt).toBeUndefined();
    });

    it("returns to idle from processing", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress(mockProgress);
      store.getState().reset();
      const state = store.getState();

      expect(state.id).toBe("");
      expect(state.status).toBe("idle");
      expect(state.fileProgress).toBeNull();
      expect(state.results).toEqual([]);
    });
  });
});
