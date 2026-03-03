/**
 * Editor store factory — headless state machine for recipe editing.
 *
 * Wraps @bnto/nodes pure functions into a reactive Zustand store with
 * undo/redo history. This is the single source of truth for both the
 * visual (BentoCanvas) and code (CodeMirror) editors.
 *
 * Same factory pattern as recipeFlowStore — create per editor mount.
 */

import { createStore } from "zustand/vanilla";
import type { Definition } from "@bnto/nodes";
import {
  createBlankDefinition,
  addNode,
  removeNode,
  updateNodeParams,
  moveNode,
  validateDefinition,
  getRecipeBySlug,
} from "@bnto/nodes";
import type { EditorStore, EditorState } from "./types";

// ---------------------------------------------------------------------------
// History helpers
// ---------------------------------------------------------------------------

const MAX_UNDO_HISTORY = 50;

/** Push the current definition onto the undo stack, clear redo. */
function pushHistory(state: EditorState): Pick<EditorState, "undoStack" | "redoStack"> {
  const stack = [...state.undoStack, state.definition];
  return {
    undoStack: stack.length > MAX_UNDO_HISTORY ? stack.slice(-MAX_UNDO_HISTORY) : stack,
    redoStack: [],
  };
}

// ---------------------------------------------------------------------------
// Store factory
// ---------------------------------------------------------------------------

/**
 * Creates an editor store instance.
 *
 * @param initialDefinition - Starting definition. Defaults to a blank canvas.
 */
function createEditorStore(initialDefinition?: Definition) {
  const definition = initialDefinition ?? createBlankDefinition();

  return createStore<EditorStore>()((set, get) => ({
    // Initial state
    definition,
    selectedNodeId: null,
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
        selectedNodeId: null,
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
        selectedNodeId: null,
        isDirty: false,
        validationErrors: validateDefinition(def),
        executionState: {},
        undoStack: [],
        redoStack: [],
      });
    },

    // --- Node operations (delegate to @bnto/nodes pure functions) ---

    addNode: (type, position) => {
      const state = get();
      const history = pushHistory(state);
      const result = addNode(state.definition, type, position);
      set({
        definition: result.definition,
        validationErrors: result.errors,
        isDirty: true,
        ...history,
      });
    },

    removeNode: (id) => {
      const state = get();
      const history = pushHistory(state);
      const result = removeNode(state.definition, id);
      set({
        definition: result.definition,
        validationErrors: result.errors,
        isDirty: true,
        selectedNodeId: state.selectedNodeId === id ? null : state.selectedNodeId,
        ...history,
      });
    },

    selectNode: (id) => {
      set({ selectedNodeId: id });
    },

    updateParams: (nodeId, params) => {
      const state = get();
      const history = pushHistory(state);
      const result = updateNodeParams(state.definition, nodeId, params);
      set({
        definition: result.definition,
        validationErrors: result.errors,
        isDirty: true,
        ...history,
      });
    },

    moveNode: (nodeId, position) => {
      const state = get();
      const history = pushHistory(state);
      const result = moveNode(state.definition, nodeId, position);
      set({
        definition: result.definition,
        validationErrors: result.errors,
        isDirty: true,
        ...history,
      });
    },

    // --- History ---

    undo: () => {
      const state = get();
      if (state.undoStack.length === 0) return;
      const previous = state.undoStack[state.undoStack.length - 1]!;
      set({
        definition: previous,
        validationErrors: validateDefinition(previous),
        isDirty: true,
        undoStack: state.undoStack.slice(0, -1),
        redoStack: [...state.redoStack, state.definition],
      });
    },

    redo: () => {
      const state = get();
      if (state.redoStack.length === 0) return;
      const next = state.redoStack[state.redoStack.length - 1]!;
      set({
        definition: next,
        validationErrors: validateDefinition(next),
        isDirty: true,
        undoStack: [...state.undoStack, state.definition],
        redoStack: state.redoStack.slice(0, -1),
      });
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
      const history = pushHistory(state);
      set({
        definition: newDefinition,
        validationErrors: validateDefinition(newDefinition),
        isDirty: true,
        ...history,
      });
    },
  }));
}

export { createEditorStore };
