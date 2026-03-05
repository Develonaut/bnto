/**
 * Editor store action integration tests — addNode, removeNode, updateParams,
 * and I/O node protection.
 *
 * Actions are pure functions called via store.setState() — same pattern
 * the hooks use. See editor/actions/ for the pure action tests.
 */

import { describe, it, expect, beforeEach } from "vitest";
import { createEditorStore } from "./createEditorStore";
import { addNode } from "../actions/addNode";
import { removeNode } from "../actions/removeNode";
import { updateParams } from "../actions/updateParams";
import type { EditorStore } from "./types";
import type { StoreApi } from "zustand";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function state(store: StoreApi<EditorStore>) {
  return store.getState();
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

describe("createEditorStore — actions", () => {
  let store: StoreApi<EditorStore>;

  beforeEach(() => {
    store = createEditorStore();
  });

  // --- addNode ---

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

  // --- removeNode ---

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

  // --- updateConfigParams ---

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
});
