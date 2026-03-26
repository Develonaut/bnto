/**
 * Debug helper tests — TDD: tests written before implementation.
 *
 * Each helper is a pure function taking a StoreApi<EditorStore> and
 * performing a targeted state mutation for console debugging.
 */

import { describe, it, expect } from "vitest";
import { createEditorStore } from "../store/createEditorStore";
import { state, addNodeViaStore } from "../store/testHelpers";
import {
  setPhase,
  setNodeStatus,
  setAllNodeStatuses,
  simulateProgress,
  forceError,
  forceComplete,
  setDirty,
  clearValidation,
  injectValidationError,
  reset,
  clear,
  step,
  run,
} from "./debugHelpers";

function createStore() {
  return createEditorStore();
}

describe("debugHelpers", () => {
  // --- setPhase ---

  describe("setPhase", () => {
    it("sets executionPhase to running", () => {
      const store = createStore();
      setPhase(store, "running");
      expect(state(store).executionPhase).toBe("running");
    });

    it("sets executionPhase to failed", () => {
      const store = createStore();
      setPhase(store, "failed");
      expect(state(store).executionPhase).toBe("failed");
    });

    it("sets executionPhase to completed", () => {
      const store = createStore();
      setPhase(store, "completed");
      expect(state(store).executionPhase).toBe("completed");
    });

    it("sets executionPhase to idle", () => {
      const store = createStore();
      setPhase(store, "running");
      setPhase(store, "idle");
      expect(state(store).executionPhase).toBe("idle");
    });
  });

  // --- setNodeStatus ---

  describe("setNodeStatus", () => {
    it("sets one node to active", () => {
      const store = createStore();
      const id = addNodeViaStore(store, "image-compress")!;
      setNodeStatus(store, id, "active");
      expect(state(store).executionState[id]).toBe("active");
    });

    it("sets a node to failed", () => {
      const store = createStore();
      const id = addNodeViaStore(store, "image-compress")!;
      setNodeStatus(store, id, "failed");
      expect(state(store).executionState[id]).toBe("failed");
    });

    it("preserves other nodes statuses", () => {
      const store = createStore();
      const id1 = addNodeViaStore(store, "image-compress")!;
      const id2 = addNodeViaStore(store, "image-resize")!;
      setNodeStatus(store, id1, "active");
      setNodeStatus(store, id2, "completed");
      expect(state(store).executionState[id1]).toBe("active");
      expect(state(store).executionState[id2]).toBe("completed");
    });
  });

  // --- setAllNodeStatuses ---

  describe("setAllNodeStatuses", () => {
    it("sets all nodes to the given status", () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      addNodeViaStore(store, "image-resize");
      setAllNodeStatuses(store, "pending");
      const es = state(store).executionState;
      const nodeIds = state(store).nodes.map((n) => n.id);
      for (const id of nodeIds) {
        expect(es[id]).toBe("pending");
      }
    });

    it("works with zero nodes", () => {
      const store = createStore();
      // Blank editor has I/O nodes — set them all
      setAllNodeStatuses(store, "active");
      const es = state(store).executionState;
      expect(Object.values(es).every((s) => s === "active")).toBe(true);
    });
  });

  // --- simulateProgress ---

  describe("simulateProgress", () => {
    it("sets executionFileProgress with percent", () => {
      const store = createStore();
      simulateProgress(store, 50);
      const progress = state(store).executionFileProgress;
      expect(progress).not.toBeNull();
      expect(progress!.overallPercent).toBe(50);
    });

    it("sets a custom message", () => {
      const store = createStore();
      simulateProgress(store, 75, "Processing file 3 of 4");
      const progress = state(store).executionFileProgress;
      expect(progress!.message).toBe("Processing file 3 of 4");
    });

    it("uses a default message when none provided", () => {
      const store = createStore();
      simulateProgress(store, 25);
      const progress = state(store).executionFileProgress;
      expect(progress!.message).toBeTruthy();
    });

    it("clamps percent to 0-100", () => {
      const store = createStore();
      simulateProgress(store, 150);
      expect(state(store).executionFileProgress!.overallPercent).toBe(100);
      simulateProgress(store, -10);
      expect(state(store).executionFileProgress!.overallPercent).toBe(0);
    });
  });

  // --- forceError ---

  describe("forceError", () => {
    it("sets phase to failed and pushes error message", () => {
      const store = createStore();
      forceError(store, "Something went wrong");
      expect(state(store).executionPhase).toBe("failed");
      expect(state(store).executionErrors).toContain("Something went wrong");
    });

    it("appends to existing errors", () => {
      const store = createStore();
      forceError(store, "First error");
      forceError(store, "Second error");
      expect(state(store).executionErrors).toContain("First error");
      expect(state(store).executionErrors).toContain("Second error");
    });
  });

  // --- forceComplete ---

  describe("forceComplete", () => {
    it("sets phase to completed", () => {
      const store = createStore();
      forceComplete(store);
      expect(state(store).executionPhase).toBe("completed");
    });

    it("sets all existing nodes to completed", () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      forceComplete(store);
      const es = state(store).executionState;
      const nodeIds = state(store).nodes.map((n) => n.id);
      for (const id of nodeIds) {
        expect(es[id]).toBe("completed");
      }
    });

    it("clears errors", () => {
      const store = createStore();
      forceError(store, "prior error");
      forceComplete(store);
      expect(state(store).executionErrors).toEqual([]);
    });
  });

  // --- setDirty ---

  describe("setDirty", () => {
    it("sets isDirty to true", () => {
      const store = createStore();
      setDirty(store, true);
      expect(state(store).isDirty).toBe(true);
    });

    it("sets isDirty to false", () => {
      const store = createStore();
      setDirty(store, true);
      setDirty(store, false);
      expect(state(store).isDirty).toBe(false);
    });
  });

  // --- clearValidation ---

  describe("clearValidation", () => {
    it("clears validation errors to empty array", () => {
      const store = createStore();
      injectValidationError(store, { nodeId: "n1", field: "quality", message: "too low" });
      clearValidation(store);
      expect(state(store).validationErrors).toEqual([]);
    });
  });

  // --- injectValidationError ---

  describe("injectValidationError", () => {
    it("pushes a validation error", () => {
      const store = createStore();
      injectValidationError(store, { nodeId: "n1", field: "quality", message: "Invalid value" });
      expect(state(store).validationErrors).toHaveLength(1);
      expect(state(store).validationErrors[0]).toEqual({
        nodeId: "n1",
        field: "quality",
        message: "Invalid value",
      });
    });

    it("appends to existing errors", () => {
      const store = createStore();
      injectValidationError(store, { nodeId: "n1", field: "quality", message: "First" });
      injectValidationError(store, { nodeId: "n2", field: "format", message: "Second" });
      expect(state(store).validationErrors).toHaveLength(2);
    });
  });

  // --- reset ---

  describe("reset", () => {
    it("resets execution phase to idle", () => {
      const store = createStore();
      setPhase(store, "running");
      reset(store);
      expect(state(store).executionPhase).toBe("idle");
    });

    it("resets to blank recipe with only I/O nodes", () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      addNodeViaStore(store, "image-resize");
      const countBefore = state(store).nodes.length;
      reset(store);
      // Blank recipe has only I/O nodes — fewer than before adding processing nodes
      expect(state(store).nodes.length).toBeLessThan(countBefore);
      expect(state(store).executionState).toEqual({});
    });

    it("clears errors, progress, dirty, validation, and history", () => {
      const store = createStore();
      forceError(store, "bad");
      simulateProgress(store, 50);
      setDirty(store, true);
      injectValidationError(store, { nodeId: "n1", field: "f", message: "err" });
      reset(store);
      expect(state(store).executionErrors).toEqual([]);
      expect(state(store).executionFileProgress).toBeNull();
      expect(state(store).isDirty).toBe(false);
      expect(state(store).validationErrors).toEqual([]);
      expect(state(store).undoStack).toEqual([]);
      expect(state(store).redoStack).toEqual([]);
    });
  });

  // --- clear (execution only, keep recipe) ---

  describe("clear", () => {
    it("resets execution phase to idle", () => {
      const store = createStore();
      setPhase(store, "running");
      clear(store);
      expect(state(store).executionPhase).toBe("idle");
    });

    it("keeps existing nodes", () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      const countBefore = state(store).nodes.length;
      setPhase(store, "running");
      clear(store);
      expect(state(store).nodes.length).toBe(countBefore);
    });

    it("clears errors and progress", () => {
      const store = createStore();
      forceError(store, "bad");
      simulateProgress(store, 50);
      clear(store);
      expect(state(store).executionErrors).toEqual([]);
      expect(state(store).executionFileProgress).toBeNull();
    });
  });

  // --- step ---

  describe("step", () => {
    it("transitions from idle to running with all nodes pending", () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      step(store);
      expect(state(store).executionPhase).toBe("running");
      // processing nodes should be pending
      const es = state(store).executionState;
      const pendingValues = Object.values(es).filter((s) => s === "pending");
      expect(pendingValues.length).toBeGreaterThan(0);
    });

    it("activates the first processing node on second step", () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      step(store); // idle → running
      step(store); // first node → active
      const es = state(store).executionState;
      const activeValues = Object.values(es).filter((s) => s === "active");
      expect(activeValues.length).toBe(1);
    });

    it("completes active node and activates next on further steps", () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      addNodeViaStore(store, "image-convert");
      step(store); // idle → running
      step(store); // first node → active
      step(store); // first node → completed, second → active
      const es = state(store).executionState;
      const completedCount = Object.values(es).filter((s) => s === "completed").length;
      const activeCount = Object.values(es).filter((s) => s === "active").length;
      expect(completedCount).toBeGreaterThanOrEqual(1);
      expect(activeCount).toBe(1);
    });

    it("reaches completed phase after enough steps", () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      // Step until completed (max 10 to avoid infinite)
      for (let i = 0; i < 10; i++) {
        step(store);
        if (state(store).executionPhase === "completed") break;
      }
      expect(state(store).executionPhase).toBe("completed");
    });

    it("returns a description string", () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      const msg = step(store);
      expect(typeof msg).toBe("string");
      expect(msg.length).toBeGreaterThan(0);
    });

    it("returns message when no processing nodes exist", () => {
      const store = createStore();
      // Default store has only I/O nodes
      const msg = step(store);
      expect(msg).toContain("No processing nodes");
    });
  });

  // --- run ---

  describe("run", () => {
    it("auto-steps to completed with zero delay", async () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      await run(store, 0);
      expect(state(store).executionPhase).toBe("completed");
    });

    it("completes with multiple nodes", async () => {
      const store = createStore();
      addNodeViaStore(store, "image-compress");
      addNodeViaStore(store, "image-convert");
      await run(store, 0);
      expect(state(store).executionPhase).toBe("completed");
      // I/O nodes stay "idle"; processing nodes should be "completed"
      const es = state(store).executionState;
      const processingStatuses = Object.values(es).filter((s) => s !== "idle");
      expect(processingStatuses.length).toBeGreaterThan(0);
      for (const status of processingStatuses) {
        expect(status).toBe("completed");
      }
    });
  });
});
