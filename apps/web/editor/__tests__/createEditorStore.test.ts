/**
 * Editor store tests — verify all store operations via Vitest.
 *
 * Tests the vanilla Zustand store directly (no React rendering).
 * Each test creates a fresh store instance via the factory.
 */

import { describe, it, expect, beforeEach } from "vitest";
import { createEditorStore } from "../store/createEditorStore";
import type { EditorStore } from "../store/types";
import type { StoreApi } from "zustand";
import { createBlankDefinition } from "@bnto/nodes";
import { NODE_TYPE_NAMES } from "@bnto/nodes";

// ---------------------------------------------------------------------------
// Helper to get current state
// ---------------------------------------------------------------------------

function state(store: StoreApi<EditorStore>) {
  return store.getState();
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
    it("starts with a blank definition when no initial provided", () => {
      const s = state(store);
      expect(s.definition.type).toBe("group");
      expect(s.definition.nodes).toEqual([]);
      expect(s.isDirty).toBe(false);
      expect(s.undoStack).toEqual([]);
      expect(s.redoStack).toEqual([]);
    });

    it("accepts a custom initial definition", () => {
      const custom = createBlankDefinition();
      custom.name = "My Custom Recipe";
      const customStore = createEditorStore(custom);
      expect(state(customStore).definition.name).toBe("My Custom Recipe");
    });

    it("validates the initial definition", () => {
      const s = state(store);
      expect(s.validationErrors).toEqual([]);
    });
  });

  // --- loadRecipe ---

  describe("loadRecipe", () => {
    it("loads a predefined recipe by slug", () => {
      state(store).loadRecipe("compress-images");
      const s = state(store);
      expect(s.definition.name).toBe("Compress Images");
      expect(s.definition.nodes!.length).toBeGreaterThan(0);
      expect(s.isDirty).toBe(false);
    });

    it("resets history when loading a recipe", () => {
      state(store).addNode("image");
      expect(state(store).undoStack.length).toBe(1);

      state(store).loadRecipe("compress-images");
      expect(state(store).undoStack).toEqual([]);
      expect(state(store).redoStack).toEqual([]);
    });

    it("does nothing for an unknown slug", () => {
      const before = state(store).definition;
      state(store).loadRecipe("nonexistent-recipe");
      expect(state(store).definition).toBe(before);
    });
  });

  // --- createBlank ---

  describe("createBlank", () => {
    it("resets to a fresh blank definition", () => {
      state(store).addNode("image");
      state(store).addNode("spreadsheet");
      expect(state(store).definition.nodes!.length).toBe(2);

      state(store).createBlank();
      expect(state(store).definition.nodes).toEqual([]);
      expect(state(store).isDirty).toBe(false);
      expect(state(store).undoStack).toEqual([]);
    });
  });

  // --- addNode ---

  describe("addNode", () => {
    it("adds a node to the definition", () => {
      state(store).addNode("image");
      const s = state(store);
      expect(s.definition.nodes!.length).toBe(1);
      expect(s.definition.nodes![0]!.type).toBe("image");
      expect(s.isDirty).toBe(true);
    });

    it("adds a node with a custom position", () => {
      state(store).addNode("image", { x: 100, y: 200 });
      const node = state(store).definition.nodes![0]!;
      expect(node.position).toEqual({ x: 100, y: 200 });
    });

    it("pushes to undo stack", () => {
      state(store).addNode("image");
      expect(state(store).undoStack.length).toBe(1);
    });

    it("clears redo stack on new action", () => {
      state(store).addNode("image");
      state(store).undo();
      expect(state(store).redoStack.length).toBe(1);

      state(store).addNode("spreadsheet");
      expect(state(store).redoStack).toEqual([]);
    });

    it("works with all 10 node types", () => {
      for (const type of NODE_TYPE_NAMES) {
        const fresh = createEditorStore();
        state(fresh).addNode(type);
        expect(state(fresh).definition.nodes!.length).toBe(1);
        expect(state(fresh).definition.nodes![0]!.type).toBe(type);
      }
    });
  });

  // --- removeNode ---

  describe("removeNode", () => {
    it("removes a node by ID", () => {
      state(store).addNode("image");
      const nodeId = state(store).definition.nodes![0]!.id;

      state(store).removeNode(nodeId);
      expect(state(store).definition.nodes).toEqual([]);
    });

    it("pushes to undo stack", () => {
      state(store).addNode("image");
      const undoCount = state(store).undoStack.length;
      const nodeId = state(store).definition.nodes![0]!.id;

      state(store).removeNode(nodeId);
      expect(state(store).undoStack.length).toBe(undoCount + 1);
    });
  });

  // --- updateParams ---

  describe("updateParams", () => {
    it("updates parameters on a node", () => {
      state(store).addNode("image");
      const nodeId = state(store).definition.nodes![0]!.id;

      state(store).updateParams(nodeId, { quality: 90 });
      const node = state(store).definition.nodes![0]!;
      expect(node.parameters.quality).toBe(90);
    });

    it("merges with existing parameters", () => {
      state(store).addNode("image");
      const nodeId = state(store).definition.nodes![0]!.id;

      state(store).updateParams(nodeId, { quality: 90 });
      state(store).updateParams(nodeId, { format: "webp" });
      const node = state(store).definition.nodes![0]!;
      expect(node.parameters.quality).toBe(90);
      expect(node.parameters.format).toBe("webp");
    });

    it("pushes to undo stack", () => {
      state(store).addNode("image");
      const nodeId = state(store).definition.nodes![0]!.id;
      const undoCount = state(store).undoStack.length;

      state(store).updateParams(nodeId, { quality: 50 });
      expect(state(store).undoStack.length).toBe(undoCount + 1);
    });
  });

  // --- Undo / Redo ---

  describe("undo/redo", () => {
    it("undoes the last operation", () => {
      state(store).addNode("image");
      expect(state(store).definition.nodes!.length).toBe(1);

      state(store).undo();
      expect(state(store).definition.nodes).toEqual([]);
    });

    it("redoes an undone operation", () => {
      state(store).addNode("image");
      state(store).undo();
      expect(state(store).definition.nodes).toEqual([]);

      state(store).redo();
      expect(state(store).definition.nodes!.length).toBe(1);
    });

    it("returns null with empty undo stack", () => {
      const result = state(store).undo();
      expect(result).toBeNull();
    });

    it("returns null with empty redo stack", () => {
      const result = state(store).redo();
      expect(result).toBeNull();
    });

    it("returns the restored snapshot on undo", () => {
      state(store).addNode("image");
      const result = state(store).undo();
      expect(result).not.toBeNull();
      expect(result!.definition).toBeDefined();
      expect(result!.positions).toBeDefined();
    });

    it("returns the restored snapshot on redo", () => {
      state(store).addNode("image");
      state(store).undo();
      const result = state(store).redo();
      expect(result).not.toBeNull();
      expect(result!.definition).toBeDefined();
      expect(result!.positions).toBeDefined();
    });

    it("captures positions from positionGetter in undo snapshot", () => {
      const mockPositions = { "node-1": { x: 100, y: 200 } };
      state(store).setPositionGetter(() => mockPositions);

      state(store).addNode("image");
      const snapshot = state(store).undoStack[0]!;
      expect(snapshot.positions).toEqual(mockPositions);
    });

    it("supports multiple undo levels", () => {
      state(store).addNode("image");
      state(store).addNode("spreadsheet");
      state(store).addNode("transform");
      expect(state(store).definition.nodes!.length).toBe(3);

      state(store).undo();
      expect(state(store).definition.nodes!.length).toBe(2);
      state(store).undo();
      expect(state(store).definition.nodes!.length).toBe(1);
      state(store).undo();
      expect(state(store).definition.nodes!.length).toBe(0);
    });

    it("maintains redo chain through multiple undos", () => {
      state(store).addNode("image");
      state(store).addNode("spreadsheet");

      state(store).undo();
      state(store).undo();

      state(store).redo();
      expect(state(store).definition.nodes!.length).toBe(1);
      state(store).redo();
      expect(state(store).definition.nodes!.length).toBe(2);
    });

    it("marks dirty after undo", () => {
      state(store).addNode("image");
      state(store).resetDirty();
      expect(state(store).isDirty).toBe(false);

      state(store).undo();
      expect(state(store).isDirty).toBe(true);
    });
  });

  // --- setPositionGetter ---

  describe("setPositionGetter", () => {
    it("captures positions from the registered getter", () => {
      const positions = { "a": { x: 10, y: 20 }, "b": { x: 30, y: 40 } };
      state(store).setPositionGetter(() => positions);

      state(store).addNode("image");
      expect(state(store).undoStack[0]!.positions).toEqual(positions);
    });

    it("uses empty positions when no getter registered", () => {
      state(store).addNode("image");
      expect(state(store).undoStack[0]!.positions).toEqual({});
    });
  });

  // --- resetDirty ---

  describe("resetDirty", () => {
    it("marks the definition as clean", () => {
      state(store).addNode("image");
      expect(state(store).isDirty).toBe(true);

      state(store).resetDirty();
      expect(state(store).isDirty).toBe(false);
    });
  });

  // --- Execution state ---

  describe("execution state", () => {
    it("sets per-node execution state", () => {
      state(store).addNode("image");
      const nodeId = state(store).definition.nodes![0]!.id;

      state(store).setExecutionState({ [nodeId]: "active" });
      expect(state(store).executionState[nodeId]).toBe("active");
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

  // --- setDefinition ---

  describe("setDefinition", () => {
    it("replaces the definition directly", () => {
      const newDef = createBlankDefinition();
      newDef.name = "From Code Editor";

      state(store).setDefinition(newDef);
      expect(state(store).definition.name).toBe("From Code Editor");
      expect(state(store).isDirty).toBe(true);
    });

    it("pushes previous definition to undo stack", () => {
      const newDef = createBlankDefinition();
      state(store).setDefinition(newDef);
      expect(state(store).undoStack.length).toBe(1);
    });

    it("validates the new definition", () => {
      const newDef = createBlankDefinition();
      state(store).setDefinition(newDef);
      expect(state(store).validationErrors).toEqual([]);
    });
  });
});
