/**
 * Editor store history tests — pushUndo, undo, redo.
 */

import { describe, it, expect, beforeEach } from "vitest";
import type { StoreApi } from "zustand";
import { createEditorStore } from "./createEditorStore";
import type { EditorStore } from "./types";
import { state, addNodeViaStore } from "./test-helpers";

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("createEditorStore — history", () => {
  let store: StoreApi<EditorStore>;

  beforeEach(() => {
    store = createEditorStore();
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
});
