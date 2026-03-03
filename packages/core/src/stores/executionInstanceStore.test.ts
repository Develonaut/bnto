import { describe, it, expect, beforeEach } from "vitest";
import { createExecutionInstanceStore } from "./executionInstanceStore";
import type {
  BrowserFileProgressInput,
  BrowserFileResult,
} from "../types/browser";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const mockProgress: BrowserFileProgressInput = {
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

describe("executionInstanceStore", () => {
  let store: ReturnType<typeof createExecutionInstanceStore>;

  beforeEach(() => {
    store = createExecutionInstanceStore();
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

      // Store computes overallPercent: ((0 * 100) + 50) / 2 = 25
      expect(state.fileProgress).toEqual({ ...mockProgress, overallPercent: 25 });
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

  describe("monotonic guard", () => {
    it("rejects backwards progress for same file", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({ fileIndex: 0, totalFiles: 1, percent: 50, message: "Half done" });
      store.getState().progress({ fileIndex: 0, totalFiles: 1, percent: 30, message: "Regressed" });

      expect(store.getState().fileProgress?.percent).toBe(50);
      expect(store.getState().fileProgress?.message).toBe("Half done");
    });

    it("allows progress reset on new file", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({ fileIndex: 0, totalFiles: 3, percent: 100, message: "Done" });
      store.getState().progress({ fileIndex: 1, totalFiles: 3, percent: 10, message: "Starting next" });

      expect(store.getState().fileProgress?.fileIndex).toBe(1);
      expect(store.getState().fileProgress?.percent).toBe(10);
    });

    it("allows equal percent updates (different message)", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({ fileIndex: 0, totalFiles: 1, percent: 50, message: "Step A" });
      store.getState().progress({ fileIndex: 0, totalFiles: 1, percent: 50, message: "Step B" });

      expect(store.getState().fileProgress?.percent).toBe(50);
      expect(store.getState().fileProgress?.message).toBe("Step B");
    });

    it("allows 0% on first progress", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({ fileIndex: 0, totalFiles: 1, percent: 0, message: "Starting..." });

      expect(store.getState().fileProgress?.percent).toBe(0);
    });

    it("rapid forward progress updates all applied", () => {
      store.getState().start("test-id", 1000);

      for (const pct of [10, 20, 30, 40, 50]) {
        store.getState().progress({ fileIndex: 0, totalFiles: 1, percent: pct, message: `${pct}%` });
      }

      expect(store.getState().fileProgress?.percent).toBe(50);
      expect(store.getState().fileProgress?.message).toBe("50%");
    });

    it("rejects extreme regression", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({ fileIndex: 0, totalFiles: 1, percent: 95, message: "Almost done" });
      store.getState().progress({ fileIndex: 0, totalFiles: 1, percent: 5, message: "Restart?!" });

      expect(store.getState().fileProgress?.percent).toBe(95);
    });
  });

  describe("overallPercent computation", () => {
    it("equals percent for single file", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({ fileIndex: 0, totalFiles: 1, percent: 60, message: "Processing" });

      expect(store.getState().fileProgress?.overallPercent).toBe(60);
    });

    it("computes correctly for multi-file batch", () => {
      store.getState().start("test-id", 1000);

      // File 0 at 50%: ((0 * 100) + 50) / 3 ≈ 17
      store.getState().progress({ fileIndex: 0, totalFiles: 3, percent: 50, message: "File 1" });
      expect(store.getState().fileProgress?.overallPercent).toBe(17);

      // File 0 at 100%: ((0 * 100) + 100) / 3 ≈ 33
      store.getState().progress({ fileIndex: 0, totalFiles: 3, percent: 100, message: "File 1 done" });
      expect(store.getState().fileProgress?.overallPercent).toBe(33);

      // File 1 at 50%: ((1 * 100) + 50) / 3 = 50
      store.getState().progress({ fileIndex: 1, totalFiles: 3, percent: 50, message: "File 2" });
      expect(store.getState().fileProgress?.overallPercent).toBe(50);

      // File 2 at 100%: ((2 * 100) + 100) / 3 = 100
      store.getState().progress({ fileIndex: 2, totalFiles: 3, percent: 100, message: "File 3 done" });
      expect(store.getState().fileProgress?.overallPercent).toBe(100);
    });

    it("is monotonic across file boundaries", () => {
      store.getState().start("test-id", 1000);
      const percentages: number[] = [];

      const updates = [
        { fileIndex: 0, totalFiles: 3, percent: 0 },
        { fileIndex: 0, totalFiles: 3, percent: 50 },
        { fileIndex: 0, totalFiles: 3, percent: 100 },
        { fileIndex: 1, totalFiles: 3, percent: 0 },
        { fileIndex: 1, totalFiles: 3, percent: 50 },
        { fileIndex: 1, totalFiles: 3, percent: 100 },
        { fileIndex: 2, totalFiles: 3, percent: 0 },
        { fileIndex: 2, totalFiles: 3, percent: 50 },
        { fileIndex: 2, totalFiles: 3, percent: 100 },
      ];

      for (const u of updates) {
        store.getState().progress({ ...u, message: "Processing" });
        percentages.push(store.getState().fileProgress?.overallPercent ?? 0);
      }

      for (let i = 1; i < percentages.length; i++) {
        expect(percentages[i]).toBeGreaterThanOrEqual(percentages[i - 1]);
      }

      expect(percentages[percentages.length - 1]).toBe(100);
    });
  });

  // ─────────────────────────────────────────────────────────────────
  // Edge Cases — Defensive Boundaries
  // ─────────────────────────────────────────────────────────────────

  describe("edge cases — totalFiles boundary", () => {
    it("handles totalFiles=0 without division by zero", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({
        fileIndex: 0,
        totalFiles: 0,
        percent: 50,
        message: "Processing",
      });

      const fp = store.getState().fileProgress;
      expect(fp).not.toBeNull();
      // totalFiles=0 falls back to 1 via `|| 1` guard
      expect(fp?.overallPercent).toBe(50);
      expect(Number.isFinite(fp?.overallPercent)).toBe(true);
    });

    it("handles totalFiles=1 (single file) correctly at 0%", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({
        fileIndex: 0,
        totalFiles: 1,
        percent: 0,
        message: "Starting",
      });

      expect(store.getState().fileProgress?.overallPercent).toBe(0);
    });

    it("handles totalFiles=1 (single file) correctly at 100%", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({
        fileIndex: 0,
        totalFiles: 1,
        percent: 100,
        message: "Done",
      });

      expect(store.getState().fileProgress?.overallPercent).toBe(100);
    });
  });

  describe("edge cases — overallPercent boundaries", () => {
    it("computes overallPercent > 100 when fileIndex exceeds totalFiles", () => {
      // Document the behavior: if fileIndex is out of bounds, overallPercent
      // can exceed 100. The service layer prevents this from happening in
      // practice, but the store doesn't clamp.
      store.getState().start("test-id", 1000);
      store.getState().progress({
        fileIndex: 3,
        totalFiles: 3,
        percent: 50,
        message: "Out of bounds",
      });

      // ((3 * 100) + 50) / 3 = 116.67 → 117
      const fp = store.getState().fileProgress;
      expect(fp?.overallPercent).toBeGreaterThan(100);
    });

    it("computes overallPercent=0 at the very start of a batch", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({
        fileIndex: 0,
        totalFiles: 10,
        percent: 0,
        message: "Starting",
      });

      expect(store.getState().fileProgress?.overallPercent).toBe(0);
    });

    it("reaches exactly 100 at the end of a large batch", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({
        fileIndex: 9,
        totalFiles: 10,
        percent: 100,
        message: "Done",
      });

      // ((9 * 100) + 100) / 10 = 100
      expect(store.getState().fileProgress?.overallPercent).toBe(100);
    });
  });

  describe("edge cases — invalid percent values", () => {
    it("accepts negative percent on first call (no guard)", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({
        fileIndex: 0,
        totalFiles: 1,
        percent: -10,
        message: "Negative",
      });

      // Store doesn't clamp — documents current behavior
      expect(store.getState().fileProgress?.percent).toBe(-10);
    });

    it("accepts percent > 100 (no upper clamp)", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({
        fileIndex: 0,
        totalFiles: 1,
        percent: 150,
        message: "Overshoot",
      });

      // Store doesn't clamp — documents current behavior
      expect(store.getState().fileProgress?.percent).toBe(150);
      expect(store.getState().fileProgress?.overallPercent).toBe(150);
    });

    it("monotonic guard still works with unusual values", () => {
      store.getState().start("test-id", 1000);
      store.getState().progress({
        fileIndex: 0,
        totalFiles: 1,
        percent: 150,
        message: "Overshoot",
      });
      store.getState().progress({
        fileIndex: 0,
        totalFiles: 1,
        percent: 100,
        message: "Correct",
      });

      // 100 < 150, so monotonic guard rejects it
      expect(store.getState().fileProgress?.percent).toBe(150);
    });
  });

  describe("edge cases — progress in non-processing states", () => {
    it("updates fileProgress even when status is idle", () => {
      // Store doesn't guard against status — service layer manages this.
      // Document the behavior: store is a dumb state container.
      store.getState().progress({
        fileIndex: 0,
        totalFiles: 1,
        percent: 50,
        message: "Stale update",
      });

      expect(store.getState().fileProgress).not.toBeNull();
      expect(store.getState().status).toBe("idle");
    });

    it("updates fileProgress when status is completed", () => {
      store.getState().start("test-id", 1000);
      store.getState().complete([mockResult], 2000);

      store.getState().progress({
        fileIndex: 0,
        totalFiles: 1,
        percent: 50,
        message: "Late arrival",
      });

      // Store accepts it — service abort flag prevents this in practice
      expect(store.getState().fileProgress).not.toBeNull();
      expect(store.getState().status).toBe("completed");
    });

    it("updates fileProgress when status is failed", () => {
      store.getState().start("test-id", 1000);
      store.getState().fail("Error", 2000);

      store.getState().progress({
        fileIndex: 0,
        totalFiles: 1,
        percent: 50,
        message: "Late arrival",
      });

      // Store accepts it — service abort flag prevents this in practice
      expect(store.getState().fileProgress).not.toBeNull();
      expect(store.getState().status).toBe("failed");
    });
  });

  describe("edge cases — state transitions from unexpected states", () => {
    it("complete from idle sets completed status", () => {
      // No start() call — go directly to complete
      store.getState().complete([mockResult], 2000);

      expect(store.getState().status).toBe("completed");
      expect(store.getState().results).toEqual([mockResult]);
    });

    it("fail from idle sets failed status", () => {
      // No start() call — go directly to fail
      store.getState().fail("Unexpected", 2000);

      expect(store.getState().status).toBe("failed");
      expect(store.getState().error).toBe("Unexpected");
    });

    it("double start resets to clean processing state", () => {
      store.getState().start("first", 1000);
      store.getState().progress({
        fileIndex: 0,
        totalFiles: 1,
        percent: 50,
        message: "In progress",
      });

      store.getState().start("second", 2000);

      expect(store.getState().id).toBe("second");
      expect(store.getState().fileProgress).toBeNull();
      expect(store.getState().results).toEqual([]);
    });

    it("double complete overwrites results", () => {
      store.getState().start("test-id", 1000);
      store.getState().complete([mockResult], 2000);

      const secondResult: BrowserFileResult = {
        ...mockResult,
        filename: "second.jpg",
      };
      store.getState().complete([secondResult], 3000);

      expect(store.getState().results).toEqual([secondResult]);
      expect(store.getState().completedAt).toBe(3000);
    });

    it("reset from idle is a safe no-op", () => {
      store.getState().reset();

      expect(store.getState().status).toBe("idle");
      expect(store.getState().id).toBe("");
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
