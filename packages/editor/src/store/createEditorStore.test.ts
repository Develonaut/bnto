/**
 * Editor store tests — initialization, entry points, and basic setters.
 *
 * Action integration tests: createEditorStore.actions.test.ts
 * Undo/redo tests: createEditorStore.history.test.ts
 */

import { describe, it, expect, beforeEach } from "vitest";
import type { StoreApi } from "zustand";
import { getRecipeBySlug } from "@bnto/core";
import { createEditorStore } from "./createEditorStore";
import type { EditorStore } from "./types";
import { state } from "./testHelpers";

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/** Helper to get a known recipe definition for testing. */
function compressImagesDefinition() {
  const recipe = getRecipeBySlug("compress-images");
  if (!recipe) throw new Error("compress-images recipe not found");
  return recipe.definition;
}

describe("createEditorStore", () => {
  let store: StoreApi<EditorStore>;

  beforeEach(() => {
    store = createEditorStore();
  });

  // --- Initialization ---

  describe("initialization", () => {
    it("starts with blank recipe metadata and I/O nodes", () => {
      const s = state(store);
      expect(s.recipeMetadata.slug).toBe("new-recipe");
      expect(s.isDirty).toBe(false);
      expect(s.nodes.length).toBe(2); // input + output
      expect(Object.keys(s.configs).length).toBe(2);
      expect(s.undoStack).toEqual([]);
      expect(s.redoStack).toEqual([]);
    });

    it("initializes from a definition with recipe nodes and metadata", () => {
      const def = compressImagesDefinition();
      const custom = createEditorStore(def);
      const s = state(custom);
      expect(s.recipeMetadata.name).toBe("Compress Images");
      expect(s.nodes.length).toBeGreaterThan(0);
      expect(Object.keys(s.configs).length).toBeGreaterThan(0);
    });

    it("starts with empty validation errors", () => {
      expect(state(store).validationErrors).toEqual([]);
    });
  });

  // --- loadDefinition ---

  describe("loadDefinition", () => {
    it("loads definition nodes, configs, and metadata", () => {
      const def = compressImagesDefinition();
      state(store).loadDefinition(def);
      const s = state(store);
      expect(s.recipeMetadata.name).toBe("Compress Images");
      expect(s.isDirty).toBe(false);
      expect(s.nodes.length).toBeGreaterThan(0);
      expect(Object.keys(s.configs).length).toBeGreaterThan(0);
    });

    it("resets history when loading a definition", () => {
      state(store).pushUndo();
      expect(state(store).undoStack.length).toBe(1);

      state(store).loadDefinition(compressImagesDefinition());
      expect(state(store).undoStack).toEqual([]);
      expect(state(store).redoStack).toEqual([]);
    });
  });

  // --- createBlank ---

  describe("createBlank", () => {
    it("resets to blank metadata and empty nodes/configs", () => {
      state(store).loadDefinition(compressImagesDefinition());
      state(store).createBlank();
      expect(state(store).recipeMetadata.slug).toBe("new-recipe");
      expect(state(store).isDirty).toBe(false);
      // createBlank loads I/O nodes from createBlankDefinition()
      expect(state(store).nodes.length).toBe(2); // input + output
      expect(Object.keys(state(store).configs).length).toBe(2);
      expect(state(store).undoStack).toEqual([]);
    });

    it("resets selection, panels, and insertion context", () => {
      state(store).loadDefinition(compressImagesDefinition());
      // Dirty up selection and panel state
      const nodeId = state(store).nodes[0].id;
      state(store).selectNode(nodeId);
      state(store).openPanel("run");
      state(store).setInsertAfterNodeId("some-id");
      state(store).setInsertIntoContainerId("some-container");

      state(store).createBlank();
      expect(state(store).selectedNodeId).toBeNull();
      expect(state(store).panels).toEqual({
        config: false,
        palette: false,
        run: false,
        help: false,
      });
      expect(state(store).insertAfterNodeId).toBeNull();
      expect(state(store).insertIntoContainerId).toBeNull();
    });
  });

  // --- Execution state ---

  describe("execution state", () => {
    it("sets per-node execution state", () => {
      state(store).setExecutionState({ "node-1": "active" });
      expect(state(store).executionState["node-1"]).toBe("active");
    });

    it("resets node statuses", () => {
      state(store).setExecutionState({ "node-1": "completed" });
      state(store).resetNodeStatuses();
      expect(state(store).executionState).toEqual({});
    });

    it("loadDefinition resets execution state", () => {
      state(store).setExecutionState({ "node-1": "active" });
      state(store).loadDefinition(compressImagesDefinition());
      expect(state(store).executionState).toEqual({});
    });
  });

  // --- setRecipeMetadata ---

  describe("setRecipeMetadata", () => {
    it("updates recipe metadata and marks dirty", () => {
      expect(state(store).isDirty).toBe(false);
      state(store).setRecipeMetadata({
        id: "new-id",
        name: "New Name",
        slug: "new-name",
        cloudId: null,
      });
      expect(state(store).recipeMetadata.name).toBe("New Name");
      expect(state(store).isDirty).toBe(true);
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
