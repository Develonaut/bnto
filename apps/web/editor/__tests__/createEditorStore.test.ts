/**
 * Editor store tests — verify the controlled-mode store.
 *
 * The store owns nodes, edges, configs, metadata, undo/redo,
 * validation, execution state, and dirty flag.
 */

import { describe, it, expect, beforeEach } from "vitest";
import { createEditorStore } from "../store/createEditorStore";
import type { EditorStore } from "../store/types";
import type { StoreApi } from "zustand";
import type { BentoNode, NodeConfigs } from "../adapters/types";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function state(store: StoreApi<EditorStore>) {
  return store.getState();
}

function makeBentoNode(id: string): BentoNode {
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
    },
  };
}

function makeConfigs(ids: string[]): NodeConfigs {
  const configs: NodeConfigs = {};
  for (const id of ids) {
    configs[id] = { nodeType: "image", name: "Test", parameters: {} };
  }
  return configs;
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
      expect(s.nodes).toEqual([]);
      expect(s.configs).toEqual({});
      expect(s.undoStack).toEqual([]);
      expect(s.redoStack).toEqual([]);
    });

    it("accepts custom initial metadata (legacy signature)", () => {
      const custom = createEditorStore({
        id: "test-id",
        name: "Custom",
        type: "group",
        version: "1.0.0",
      });
      expect(state(custom).recipeMetadata.name).toBe("Custom");
    });

    it("accepts initial nodes and configs", () => {
      const nodes = [makeBentoNode("a")];
      const configs = makeConfigs(["a"]);
      const custom = createEditorStore({
        initialNodes: nodes,
        initialConfigs: configs,
      });
      expect(state(custom).nodes).toEqual(nodes);
      expect(state(custom).configs).toEqual(configs);
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
      expect(state(store).nodes).toEqual([]);
      expect(state(store).configs).toEqual({});
      expect(state(store).undoStack).toEqual([]);
    });
  });

  // --- addNode ---

  describe("addNode", () => {
    it("adds a node and its config", () => {
      const id = state(store).addNode("image");
      expect(id).toBeTruthy();
      expect(state(store).nodes.length).toBe(1);
      expect(state(store).configs[id!]).toBeDefined();
      expect(state(store).configs[id!]!.nodeType).toBe("image");
    });

    it("marks dirty and pushes undo", () => {
      state(store).addNode("image");
      expect(state(store).isDirty).toBe(true);
      expect(state(store).undoStack.length).toBe(1);
    });

    it("returns null when canvas is full", () => {
      // Fill all slots
      for (let i = 0; i < 10; i++) {
        state(store).addNode("image");
      }
      const id = state(store).addNode("image");
      expect(id).toBeNull();
    });
  });

  // --- removeNode ---

  describe("removeNode", () => {
    it("removes node and its config", () => {
      const id = state(store).addNode("image")!;
      state(store).removeNode(id);
      expect(state(store).nodes.length).toBe(0);
      expect(state(store).configs[id]).toBeUndefined();
    });

    it("pushes undo snapshot before removal", () => {
      const id = state(store).addNode("image")!;
      // addNode pushes one undo, removeNode pushes another
      state(store).removeNode(id);
      expect(state(store).undoStack.length).toBe(2);
    });
  });

  // --- updateConfigParams ---

  describe("updateConfigParams", () => {
    it("merges params into config", () => {
      const id = state(store).addNode("image")!;
      state(store).updateConfigParams(id, { quality: 80 });
      expect(state(store).configs[id]!.parameters.quality).toBe(80);
    });

    it("does not modify nodes array", () => {
      const id = state(store).addNode("image")!;
      const nodesBefore = state(store).nodes;
      state(store).updateConfigParams(id, { quality: 80 });
      // nodes reference should be the same — no RF re-render needed
      expect(state(store).nodes).toBe(nodesBefore);
    });

    it("pushes undo and marks dirty", () => {
      const id = state(store).addNode("image")!;
      const undoBefore = state(store).undoStack.length;
      state(store).updateConfigParams(id, { quality: 80 });
      expect(state(store).undoStack.length).toBe(undoBefore + 1);
      expect(state(store).isDirty).toBe(true);
    });

    it("does nothing for unknown node ID", () => {
      const configsBefore = state(store).configs;
      state(store).updateConfigParams("nonexistent", { quality: 80 });
      expect(state(store).configs).toBe(configsBefore);
    });
  });

  // --- pushUndo ---

  describe("pushUndo", () => {
    it("captures current nodes + configs as snapshot", () => {
      state(store).addNode("image");
      const undoCount = state(store).undoStack.length;

      state(store).pushUndo();
      expect(state(store).undoStack.length).toBe(undoCount + 1);
      const snapshot = state(store).undoStack[state(store).undoStack.length - 1]!;
      expect(snapshot.nodes).toBeDefined();
      expect(snapshot.configs).toBeDefined();
    });

    it("clears redo stack", () => {
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
    it("restores nodes and configs on undo", () => {
      state(store).addNode("image");
      const afterAdd = { nodes: [...state(store).nodes], configs: { ...state(store).configs } };

      state(store).addNode("transform");
      expect(state(store).nodes.length).toBe(2);

      state(store).undo();
      expect(state(store).nodes.length).toBe(afterAdd.nodes.length);
    });

    it("restores nodes and configs on redo", () => {
      state(store).addNode("image");
      state(store).addNode("transform");
      const twoNodes = state(store).nodes.length;

      state(store).undo();
      expect(state(store).nodes.length).toBeLessThan(twoNodes);

      state(store).redo();
      expect(state(store).nodes.length).toBe(twoNodes);
    });

    it("does nothing with empty undo stack", () => {
      const nodesBefore = state(store).nodes;
      state(store).undo();
      expect(state(store).nodes).toBe(nodesBefore);
    });

    it("does nothing with empty redo stack", () => {
      const nodesBefore = state(store).nodes;
      state(store).redo();
      expect(state(store).nodes).toBe(nodesBefore);
    });

    it("supports multiple undo levels", () => {
      state(store).addNode("image");
      state(store).addNode("transform");
      state(store).addNode("spreadsheet");
      expect(state(store).nodes.length).toBe(3);

      state(store).undo(); // back to 2
      expect(state(store).nodes.length).toBe(2);

      state(store).undo(); // back to 1
      expect(state(store).nodes.length).toBe(1);

      state(store).undo(); // back to 0
      expect(state(store).nodes.length).toBe(0);
    });

    it("marks dirty after undo", () => {
      state(store).addNode("image");
      state(store).resetDirty();

      state(store).undo();
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
