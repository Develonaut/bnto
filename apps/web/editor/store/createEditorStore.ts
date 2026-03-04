/**
 * Editor store factory — headless state machine for recipe editing.
 *
 * Wraps @bnto/nodes pure functions into a reactive Zustand store with
 * undo/redo history. This is the single source of truth for both the
 * visual (BentoCanvas) and code (CodeMirror) editors.
 *
 * ReactFlow owns visual state (positions, selection, viewport).
 * This store owns domain structure (node types, params, validation).
 */

import { createStore } from "zustand/vanilla";
import type { Definition } from "@bnto/nodes";
import {
  createBlankDefinition,
  addNode,
  removeNode,
  updateNodeParams,
  validateDefinition,
  getRecipeBySlug,
} from "@bnto/nodes";
import type { EditorStore, EditorState, UndoSnapshot, PositionGetter } from "./types";

// ---------------------------------------------------------------------------
// History helpers
// ---------------------------------------------------------------------------

const MAX_UNDO_HISTORY = 50;

/** Capture current definition + RF positions into an undo snapshot. */
function captureSnapshot(
  definition: Definition,
  positionGetter: PositionGetter | null,
): UndoSnapshot {
  return {
    definition,
    positions: positionGetter ? positionGetter() : {},
  };
}

/** Push a snapshot onto the undo stack, clear redo. */
function pushHistory(
  state: EditorState,
  positionGetter: PositionGetter | null,
): Pick<EditorState, "undoStack" | "redoStack"> {
  const snapshot = captureSnapshot(state.definition, positionGetter);
  const stack = [...state.undoStack, snapshot];
  return {
    undoStack: stack.length > MAX_UNDO_HISTORY ? stack.slice(-MAX_UNDO_HISTORY) : stack,
    redoStack: [],
  };
}

// ---------------------------------------------------------------------------
// Store factory
// ---------------------------------------------------------------------------

function createEditorStore(initialDefinition?: Definition) {
  const definition = initialDefinition ?? createBlankDefinition();

  // Closure — not in Zustand state, avoids serialization issues.
  let positionGetter: PositionGetter | null = null;

  return createStore<EditorStore>()((set, get) => ({
    // Initial state
    definition,
    isDirty: false,
    validationErrors: validateDefinition(definition),
    executionState: {},
    undoStack: [],
    redoStack: [],

    // --- Entry points ---

    loadRecipe: (slug) => {
      const recipe = getRecipeBySlug(slug);
      if (!recipe) return;
      const def = recipe.definition;
      set({
        definition: def,
        isDirty: false,
        validationErrors: validateDefinition(def),
        executionState: {},
        undoStack: [],
        redoStack: [],
      });
    },

    createBlank: () => {
      const def = createBlankDefinition();
      set({
        definition: def,
        isDirty: false,
        validationErrors: validateDefinition(def),
        executionState: {},
        undoStack: [],
        redoStack: [],
      });
    },

    // --- Node operations ---

    addNode: (type, position) => {
      const state = get();
      const history = pushHistory(state, positionGetter);
      const result = addNode(state.definition, type, position);
      set({
        definition: result.definition,
        validationErrors: result.errors,
        isDirty: true,
        ...history,
      });
      /* Return the new node's ID so callers can immediately
       * add it to ReactFlow and fitView without async. */
      const nodes = result.definition.nodes ?? [];
      return nodes.length > 0 ? nodes[nodes.length - 1]!.id : null;
    },

    removeNode: (id) => {
      const state = get();
      const history = pushHistory(state, positionGetter);
      const result = removeNode(state.definition, id);
      set({
        definition: result.definition,
        validationErrors: result.errors,
        isDirty: true,
        ...history,
      });
    },

    updateParams: (nodeId, params) => {
      const state = get();
      const history = pushHistory(state, positionGetter);
      const result = updateNodeParams(state.definition, nodeId, params);
      set({
        definition: result.definition,
        validationErrors: result.errors,
        isDirty: true,
        ...history,
      });
    },

    // --- History ---
    // undo/redo return the restored snapshot so the caller can apply
    // position data back to ReactFlow.

    undo: () => {
      const state = get();
      if (state.undoStack.length === 0) return null;
      const snapshot = state.undoStack[state.undoStack.length - 1]!;
      const currentSnapshot = captureSnapshot(state.definition, positionGetter);
      set({
        definition: snapshot.definition,
        validationErrors: validateDefinition(snapshot.definition),
        isDirty: true,
        undoStack: state.undoStack.slice(0, -1),
        redoStack: [...state.redoStack, currentSnapshot],
      });
      return snapshot;
    },

    redo: () => {
      const state = get();
      if (state.redoStack.length === 0) return null;
      const snapshot = state.redoStack[state.redoStack.length - 1]!;
      const currentSnapshot = captureSnapshot(state.definition, positionGetter);
      set({
        definition: snapshot.definition,
        validationErrors: validateDefinition(snapshot.definition),
        isDirty: true,
        undoStack: [...state.undoStack, currentSnapshot],
        redoStack: state.redoStack.slice(0, -1),
      });
      return snapshot;
    },

    // --- Utility ---

    resetDirty: () => {
      set({ isDirty: false });
    },

    setExecutionState: (executionState) => {
      set({ executionState });
    },

    resetExecution: () => {
      set({ executionState: {} });
    },

    setDefinition: (newDefinition) => {
      const state = get();
      const history = pushHistory(state, positionGetter);
      set({
        definition: newDefinition,
        validationErrors: validateDefinition(newDefinition),
        isDirty: true,
        ...history,
      });
    },

    setPositionGetter: (getter) => {
      positionGetter = getter;
    },
  }));
}

export { createEditorStore };
