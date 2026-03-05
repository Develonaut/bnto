/**
 * Editor store tests — initialization, entry points, and basic setters.
 *
 * Action integration tests: createEditorStore.actions.test.ts
 * Undo/redo tests: createEditorStore.history.test.ts
 */

import { describe, it, expect, beforeEach } from "vitest";
import type { StoreApi } from "zustand";
import { createEditorStore } from "./createEditorStore";
import type { EditorStore } from "./types";
import { state } from "./test-helpers";

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
    it("starts with blank recipe metadata and I/O nodes", () => {
      const s = state(store);
      expect(s.recipeMetadata.type).toBe("group");
      expect(s.isDirty).toBe(false);
      expect(s.nodes.length).toBe(2); // input + output
      expect(Object.keys(s.configs).length).toBe(2);
      expect(s.undoStack).toEqual([]);
      expect(s.redoStack).toEqual([]);
    });

    it("resolves a slug into recipe nodes and metadata", () => {
      const custom = createEditorStore("compress-images");
      const s = state(custom);
      expect(s.recipeMetadata.name).toBe("Compress Images");
      expect(s.slug).toBe("compress-images");
      expect(s.nodes.length).toBeGreaterThan(0);
      expect(Object.keys(s.configs).length).toBeGreaterThan(0);
    });

    it("falls back to blank for unknown slug", () => {
      const custom = createEditorStore("nonexistent");
      const s = state(custom);
      expect(s.slug).toBe("nonexistent");
      expect(s.recipeMetadata.type).toBe("group");
    });

    it("starts with empty validation errors", () => {
      expect(state(store).validationErrors).toEqual([]);
    });
  });

  // --- loadRecipe ---

  describe("loadRecipe", () => {
    it("loads recipe nodes, configs, and metadata by slug", () => {
      state(store).loadRecipe("compress-images");
      const s = state(store);
      expect(s.recipeMetadata.name).toBe("Compress Images");
      expect(s.isDirty).toBe(false);
      expect(s.nodes.length).toBeGreaterThan(0);
      expect(Object.keys(s.configs).length).toBeGreaterThan(0);
    });

    it("resets history when loading a recipe", () => {
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
    it("resets to blank metadata and empty nodes/configs", () => {
      state(store).loadRecipe("compress-images");
      state(store).createBlank();
      expect(state(store).recipeMetadata.type).toBe("group");
      expect(state(store).isDirty).toBe(false);
      // createBlank loads I/O nodes from createBlankDefinition()
      expect(state(store).nodes.length).toBe(2); // input + output
      expect(Object.keys(state(store).configs).length).toBe(2);
      expect(state(store).undoStack).toEqual([]);
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
      state(store).pushUndo();
      state(store).pushUndo();

      state(store).resetHistory();
      expect(state(store).undoStack).toEqual([]);
      expect(state(store).redoStack).toEqual([]);
    });
  });
});
