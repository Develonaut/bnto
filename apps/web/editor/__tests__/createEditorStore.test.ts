/**
 * Editor store tests — verify the thin RF-first store.
 *
 * The store no longer owns node state — ReactFlow does.
 * It manages: recipe metadata, undo/redo history, validation,
 * execution state, and dirty flag.
 */

import { describe, it, expect, beforeEach } from "vitest";
import { createEditorStore } from "../store/createEditorStore";
import type { EditorStore } from "../store/types";
import type { StoreApi } from "zustand";
import type { BentoNode } from "../adapters/types";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function state(store: StoreApi<EditorStore>) {
  return store.getState();
}

function makeBentoNode(id: string, type = "image"): BentoNode {
  return {
    id,
    type: "compartment",
    position: { x: 0, y: 0 },
    data: {
      label: "Test",
      variant: "primary",
      width: 200,
      height: 200,
      status: "idle",
      nodeType: type,
      name: "Test",
      parameters: {},
    },
  };
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("createEditorStore", () => {
  let store: StoreApi<EditorStore>;

  beforeEach(() => {
    store = createEditorStore();
  });

  // --- Initialization ---

  describe("initialization", () => {
    it("starts with blank recipe metadata", () => {
      const s = state(store);
      expect(s.recipeMetadata.type).toBe("group");
      expect(s.isDirty).toBe(false);
      expect(s.undoStack).toEqual([]);
      expect(s.redoStack).toEqual([]);
    });

    it("accepts custom initial metadata", () => {
      const custom = createEditorStore({
        id: "test-id",
        name: "Custom",
        type: "group",
        version: "1.0.0",
      });
      expect(state(custom).recipeMetadata.name).toBe("Custom");
    });

    it("starts with empty validation errors", () => {
      expect(state(store).validationErrors).toEqual([]);
    });
  });

  // --- loadRecipe ---

  describe("loadRecipe", () => {
    it("loads recipe metadata by slug", () => {
      state(store).loadRecipe("compress-images");
      expect(state(store).recipeMetadata.name).toBe("Compress Images");
      expect(state(store).isDirty).toBe(false);
    });

    it("resets history when loading a recipe", () => {
      // Simulate some history
      state(store).setNodeGetter(() => [makeBentoNode("a")]);
      state(store).pushUndo();
      expect(state(store).undoStack.length).toBe(1);

      state(store).loadRecipe("compress-images");
      expect(state(store).undoStack).toEqual([]);
      expect(state(store).redoStack).toEqual([]);
    });

    it("does nothing for an unknown slug", () => {
      const before = state(store).recipeMetadata;
      state(store).loadRecipe("nonexistent-recipe");
      expect(state(store).recipeMetadata).toBe(before);
    });
  });

  // --- createBlank ---

  describe("createBlank", () => {
    it("resets to blank metadata", () => {
      state(store).loadRecipe("compress-images");
      state(store).createBlank();
      expect(state(store).recipeMetadata.type).toBe("group");
      expect(state(store).isDirty).toBe(false);
      expect(state(store).undoStack).toEqual([]);
    });
  });

  // --- pushUndo ---

  describe("pushUndo", () => {
    it("captures current RF nodes via nodeGetter", () => {
      const nodes = [makeBentoNode("a"), makeBentoNode("b")];
      state(store).setNodeGetter(() => nodes);

      state(store).pushUndo();
      expect(state(store).undoStack.length).toBe(1);
      expect(state(store).undoStack[0]).toEqual(nodes);
    });

    it("uses empty array when no getter registered", () => {
      state(store).pushUndo();
      expect(state(store).undoStack[0]).toEqual([]);
    });

    it("clears redo stack", () => {
      state(store).setNodeGetter(() => []);
      state(store).pushUndo();
      state(store).undo();
      expect(state(store).redoStack.length).toBe(1);

      state(store).pushUndo();
      expect(state(store).redoStack).toEqual([]);
    });
  });

  // --- markDirty ---

  describe("markDirty", () => {
    it("sets isDirty to true", () => {
      expect(state(store).isDirty).toBe(false);
      state(store).markDirty();
      expect(state(store).isDirty).toBe(true);
    });
  });

  // --- Undo / Redo ---

  describe("undo/redo", () => {
    it("returns BentoNode[] snapshot on undo", () => {
      const nodes = [makeBentoNode("a")];
      state(store).setNodeGetter(() => nodes);
      state(store).pushUndo();

      // Simulate mutation — now getter returns different state
      state(store).setNodeGetter(() => [makeBentoNode("a"), makeBentoNode("b")]);

      const snapshot = state(store).undo();
      expect(snapshot).toEqual(nodes);
    });

    it("returns BentoNode[] snapshot on redo", () => {
      const nodes = [makeBentoNode("a")];
      state(store).setNodeGetter(() => nodes);
      state(store).pushUndo();
      state(store).setNodeGetter(() => []);
      state(store).undo();

      // Now redo should give back the empty state that was captured
      state(store).setNodeGetter(() => nodes); // simulate caller restored undo
      const snapshot = state(store).redo();
      expect(snapshot).toEqual([]);
    });

    it("returns null with empty undo stack", () => {
      expect(state(store).undo()).toBeNull();
    });

    it("returns null with empty redo stack", () => {
      expect(state(store).redo()).toBeNull();
    });

    it("supports multiple undo levels", () => {
      const nodesA = [makeBentoNode("a")];
      const nodesAB = [makeBentoNode("a"), makeBentoNode("b")];
      const nodesABC = [makeBentoNode("a"), makeBentoNode("b"), makeBentoNode("c")];

      state(store).setNodeGetter(() => []);
      state(store).pushUndo(); // snapshot: []

      state(store).setNodeGetter(() => nodesA);
      state(store).pushUndo(); // snapshot: [a]

      state(store).setNodeGetter(() => nodesAB);
      state(store).pushUndo(); // snapshot: [a, b]

      state(store).setNodeGetter(() => nodesABC);

      const snap1 = state(store).undo();
      expect(snap1).toEqual(nodesAB);

      state(store).setNodeGetter(() => nodesAB);
      const snap2 = state(store).undo();
      expect(snap2).toEqual(nodesA);

      state(store).setNodeGetter(() => nodesA);
      const snap3 = state(store).undo();
      expect(snap3).toEqual([]);
    });

    it("marks dirty after undo", () => {
      state(store).setNodeGetter(() => []);
      state(store).pushUndo();
      state(store).resetDirty();

      state(store).undo();
      expect(state(store).isDirty).toBe(true);
    });
  });

  // --- setNodeGetter ---

  describe("setNodeGetter", () => {
    it("captures nodes from the registered getter in pushUndo", () => {
      const nodes = [makeBentoNode("x"), makeBentoNode("y")];
      state(store).setNodeGetter(() => nodes);

      state(store).pushUndo();
      expect(state(store).undoStack[0]).toEqual(nodes);
    });
  });

  // --- resetDirty ---

  describe("resetDirty", () => {
    it("marks as clean", () => {
      state(store).markDirty();
      expect(state(store).isDirty).toBe(true);

      state(store).resetDirty();
      expect(state(store).isDirty).toBe(false);
    });
  });

  // --- Execution state ---

  describe("execution state", () => {
    it("sets per-node execution state", () => {
      state(store).setExecutionState({ "node-1": "active" });
      expect(state(store).executionState["node-1"]).toBe("active");
    });

    it("resets execution state", () => {
      state(store).setExecutionState({ "node-1": "completed" });
      state(store).resetExecution();
      expect(state(store).executionState).toEqual({});
    });

    it("loadRecipe resets execution state", () => {
      state(store).setExecutionState({ "node-1": "active" });
      state(store).loadRecipe("compress-images");
      expect(state(store).executionState).toEqual({});
    });
  });

  // --- setRecipeMetadata ---

  describe("setRecipeMetadata", () => {
    it("updates recipe metadata", () => {
      state(store).setRecipeMetadata({
        id: "new-id",
        name: "New Name",
        type: "group",
        version: "2.0.0",
      });
      expect(state(store).recipeMetadata.name).toBe("New Name");
    });
  });

  // --- resetHistory ---

  describe("resetHistory", () => {
    it("clears undo and redo stacks", () => {
      state(store).setNodeGetter(() => []);
      state(store).pushUndo();
      state(store).pushUndo();

      state(store).resetHistory();
      expect(state(store).undoStack).toEqual([]);
      expect(state(store).redoStack).toEqual([]);
    });
  });
});
