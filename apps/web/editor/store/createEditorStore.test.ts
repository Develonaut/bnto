/**
 * Editor store tests — verify the controlled-mode store.
 *
 * The store owns state + simple setters + entry points + undo/redo.
 * Business logic (addNode, removeNode, updateConfigParams) lives in
 * action hooks — tested separately via helpers + store.setState().
 */

import { describe, it, expect, beforeEach } from "vitest";
import { createEditorStore } from "./createEditorStore";
import { addNode } from "../actions/addNode";
import { removeNode } from "../actions/removeNode";
import { updateParams } from "../actions/updateParams";
import type { EditorStore } from "./types";
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

/** Call the pure addNode action and apply to store. */
function addNodeViaStore(store: StoreApi<EditorStore>, type: string): string | null {
  const result = addNode(store.getState(), type as never);
  if (!result) return null;
  store.setState(result.nextState);
  return result.nodeId;
}

/** Call the pure removeNode action and apply to store. */
function removeNodeViaStore(store: StoreApi<EditorStore>, id: string): void {
  const nextState = removeNode(store.getState(), id);
  if (!nextState) return;
  store.setState(nextState);
}

/** Call the pure updateParams action and apply to store. */
function updateParamsViaStore(store: StoreApi<EditorStore>, nodeId: string, params: Record<string, unknown>): void {
  const nextState = updateParams(store.getState(), nodeId, params);
  if (!nextState) return;
  store.setState(nextState);
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

  // --- addNode (via hook simulation) ---

  describe("addNode", () => {
    it("adds a node and its config", () => {
      const before = state(store).nodes.length;
      const id = addNodeViaStore(store, "image");
      expect(id).toBeTruthy();
      expect(state(store).nodes.length).toBe(before + 1);
      expect(state(store).configs[id!]).toBeDefined();
      expect(state(store).configs[id!]!.nodeType).toBe("image");
    });

    it("marks dirty and pushes undo", () => {
      addNodeViaStore(store, "image");
      expect(state(store).isDirty).toBe(true);
      expect(state(store).undoStack.length).toBe(1);
    });

    it("returns null when canvas is full", () => {
      for (let i = 0; i < 10; i++) {
        addNodeViaStore(store, "image");
      }
      const id = addNodeViaStore(store, "image");
      expect(id).toBeNull();
    });
  });

  // --- removeNode (via hook simulation) ---

  describe("removeNode", () => {
    it("removes node and its config", () => {
      const before = state(store).nodes.length;
      const id = addNodeViaStore(store, "image")!;
      removeNodeViaStore(store, id);
      expect(state(store).nodes.length).toBe(before);
      expect(state(store).configs[id]).toBeUndefined();
    });

    it("pushes undo snapshot before removal", () => {
      const id = addNodeViaStore(store, "image")!;
      // addNode pushes one undo, removeNode pushes another
      removeNodeViaStore(store, id);
      expect(state(store).undoStack.length).toBe(2);
    });
  });

  // --- I/O node protection ---

  describe("I/O node protection", () => {
    it("prevents deletion of input nodes", () => {
      state(store).createBlank(); // loads input + output
      const before = state(store).nodes.length;
      removeNodeViaStore(store, "input");
      expect(state(store).nodes.length).toBe(before);
    });

    it("prevents deletion of output nodes", () => {
      state(store).createBlank();
      const before = state(store).nodes.length;
      removeNodeViaStore(store, "output");
      expect(state(store).nodes.length).toBe(before);
    });

    it("prevents adding a second input node", () => {
      state(store).createBlank();
      const id = addNodeViaStore(store, "input");
      expect(id).toBeNull();
    });

    it("prevents adding a second output node", () => {
      state(store).createBlank();
      const id = addNodeViaStore(store, "output");
      expect(id).toBeNull();
    });

    it("allows deleting non-I/O nodes", () => {
      state(store).createBlank();
      const id = addNodeViaStore(store, "image")!;
      const before = state(store).nodes.length;
      removeNodeViaStore(store, id);
      expect(state(store).nodes.length).toBe(before - 1);
    });
  });

  // --- updateConfigParams (via hook simulation) ---

  describe("updateConfigParams", () => {
    it("merges params into config", () => {
      const id = addNodeViaStore(store, "image")!;
      updateParamsViaStore(store, id, { quality: 80 });
      expect(state(store).configs[id]!.parameters.quality).toBe(80);
    });

    it("does not modify nodes array", () => {
      const id = addNodeViaStore(store, "image")!;
      const nodesBefore = state(store).nodes;
      updateParamsViaStore(store, id, { quality: 80 });
      // nodes reference should be the same — no RF re-render needed
      expect(state(store).nodes).toBe(nodesBefore);
    });

    it("pushes undo and marks dirty", () => {
      const id = addNodeViaStore(store, "image")!;
      const undoBefore = state(store).undoStack.length;
      updateParamsViaStore(store, id, { quality: 80 });
      expect(state(store).undoStack.length).toBe(undoBefore + 1);
      expect(state(store).isDirty).toBe(true);
    });

    it("does nothing for unknown node ID", () => {
      const configsBefore = state(store).configs;
      updateParamsViaStore(store, "nonexistent", { quality: 80 });
      expect(state(store).configs).toBe(configsBefore);
    });
  });

  // --- pushUndo ---

  describe("pushUndo", () => {
    it("captures current nodes + configs as snapshot", () => {
      addNodeViaStore(store, "image");
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
      addNodeViaStore(store, "image");
      const afterAdd = state(store).nodes.length;

      addNodeViaStore(store, "transform");
      expect(state(store).nodes.length).toBe(afterAdd + 1);

      state(store).undo();
      expect(state(store).nodes.length).toBe(afterAdd);
    });

    it("restores nodes and configs on redo", () => {
      addNodeViaStore(store, "image");
      addNodeViaStore(store, "transform");
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
      const baseline = state(store).nodes.length;
      addNodeViaStore(store, "image");
      addNodeViaStore(store, "transform");
      addNodeViaStore(store, "spreadsheet");
      expect(state(store).nodes.length).toBe(baseline + 3);

      state(store).undo();
      expect(state(store).nodes.length).toBe(baseline + 2);

      state(store).undo();
      expect(state(store).nodes.length).toBe(baseline + 1);

      state(store).undo();
      expect(state(store).nodes.length).toBe(baseline);
    });

    it("marks dirty after undo", () => {
      addNodeViaStore(store, "image");
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
