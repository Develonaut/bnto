import { describe, it, expect } from "vitest";
import {
  browserExecutionReducer,
  IDLE_EXECUTION,
  type BrowserExecutionAction,
} from "./browserExecutionReducer";
import type {
  BrowserExecution,
  BrowserFileProgress,
  BrowserFileResult,
} from "../types/browser";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function processingState(overrides: Partial<BrowserExecution> = {}): BrowserExecution {
  return {
    id: "test-id",
    status: "processing",
    fileProgress: null,
    results: [],
    startedAt: 1000,
    ...overrides,
  };
}

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

describe("browserExecutionReducer", () => {
  describe("start", () => {
    it("transitions from idle to processing", () => {
      const action: BrowserExecutionAction = {
        type: "start",
        id: "exec-1",
        startedAt: 1000,
      };

      const next = browserExecutionReducer(IDLE_EXECUTION, action);

      expect(next).toEqual({
        id: "exec-1",
        status: "processing",
        fileProgress: null,
        results: [],
        startedAt: 1000,
      });
    });

    it("resets prior state when restarting", () => {
      const prior: BrowserExecution = {
        id: "old",
        status: "failed",
        fileProgress: null,
        results: [],
        error: "previous error",
        completedAt: 500,
      };

      const next = browserExecutionReducer(prior, {
        type: "start",
        id: "new-exec",
        startedAt: 2000,
      });

      expect(next.status).toBe("processing");
      expect(next.id).toBe("new-exec");
      expect(next.error).toBeUndefined();
      expect(next.completedAt).toBeUndefined();
    });
  });

  describe("progress", () => {
    it("updates fileProgress on a processing state", () => {
      const state = processingState();
      const next = browserExecutionReducer(state, {
        type: "progress",
        progress: mockProgress,
      });

      expect(next.fileProgress).toEqual(mockProgress);
      expect(next.status).toBe("processing");
    });

    it("preserves other state fields", () => {
      const state = processingState({ startedAt: 1234 });
      const next = browserExecutionReducer(state, {
        type: "progress",
        progress: mockProgress,
      });

      expect(next.id).toBe("test-id");
      expect(next.startedAt).toBe(1234);
    });
  });

  describe("complete", () => {
    it("transitions from processing to completed", () => {
      const state = processingState();
      const next = browserExecutionReducer(state, {
        type: "complete",
        results: [mockResult],
        completedAt: 2000,
      });

      expect(next.status).toBe("completed");
      expect(next.results).toEqual([mockResult]);
      expect(next.fileProgress).toBeNull();
      expect(next.completedAt).toBe(2000);
    });

    it("clears in-flight progress", () => {
      const state = processingState({ fileProgress: mockProgress });
      const next = browserExecutionReducer(state, {
        type: "complete",
        results: [],
        completedAt: 3000,
      });

      expect(next.fileProgress).toBeNull();
    });
  });

  describe("fail", () => {
    it("transitions from processing to failed", () => {
      const state = processingState();
      const next = browserExecutionReducer(state, {
        type: "fail",
        message: "WASM crashed",
        completedAt: 2000,
      });

      expect(next.status).toBe("failed");
      expect(next.error).toBe("WASM crashed");
      expect(next.fileProgress).toBeNull();
      expect(next.completedAt).toBe(2000);
    });

    it("preserves the execution ID and start time", () => {
      const state = processingState({ id: "exec-99", startedAt: 500 });
      const next = browserExecutionReducer(state, {
        type: "fail",
        message: "out of memory",
        completedAt: 1500,
      });

      expect(next.id).toBe("exec-99");
      expect(next.startedAt).toBe(500);
    });
  });

  describe("reset", () => {
    it("returns to idle from any state", () => {
      const completed: BrowserExecution = {
        id: "done",
        status: "completed",
        fileProgress: null,
        results: [mockResult],
        startedAt: 1000,
        completedAt: 2000,
      };

      const next = browserExecutionReducer(completed, { type: "reset" });
      expect(next).toEqual(IDLE_EXECUTION);
    });

    it("returns to idle from processing", () => {
      const next = browserExecutionReducer(
        processingState({ fileProgress: mockProgress }),
        { type: "reset" },
      );
      expect(next).toEqual(IDLE_EXECUTION);
    });
  });
});
