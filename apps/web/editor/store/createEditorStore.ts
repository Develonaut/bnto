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
import type { Definition, Position } from "@bnto/nodes";
import type { NodeTypeName, ValidationError } from "@bnto/nodes";
import {
  createBlankDefinition,
  addNode,
  removeNode,
  updateNodeParams,
  moveNode,
  validateDefinition,
  getRecipeBySlug,
} from "@bnto/nodes";

// ---------------------------------------------------------------------------
// Execution state — per-node status tracking
// ---------------------------------------------------------------------------

type NodeExecutionStatus = "idle" | "pending" | "active" | "completed" | "failed";

type ExecutionState = Record<string, NodeExecutionStatus>;

// ---------------------------------------------------------------------------
// State shape
// ---------------------------------------------------------------------------

interface EditorState {
  /** Current recipe definition tree. */
  definition: Definition;

  /** ID of the currently selected node (null = nothing selected). */
  selectedNodeId: string | null;

  /** Whether the definition has been modified since last save/load. */
  isDirty: boolean;

  /** Validation errors from the most recent operation. */
  validationErrors: ValidationError[];

  /** Per-node execution status map (node ID → status). */
  executionState: ExecutionState;

  /** History stack for undo (previous definitions). */
  undoStack: Definition[];

  /** History stack for redo (undone definitions). */
  redoStack: Definition[];
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

interface EditorActions {
  /** Load a predefined recipe by slug. Resets history. */
  loadRecipe: (slug: string) => void;

  /** Start with a blank canvas. Resets history. */
  createBlank: () => void;

  /** Add a node of the given type. Position auto-assigned if omitted. */
  addNode: (type: NodeTypeName, position?: Position) => void;

  /** Remove the node with the given ID. Clears selection if it was selected. */
  removeNode: (id: string) => void;

  /** Select a node by ID, or null to deselect. */
  selectNode: (id: string | null) => void;

  /** Update parameters on a node. Validates against schema. */
  updateParams: (nodeId: string, params: Record<string, unknown>) => void;

  /** Move a node to a new position. */
  moveNode: (nodeId: string, position: Position) => void;

  /** Undo the last definition change. */
  undo: () => void;

  /** Redo the last undone change. */
  redo: () => void;

  /** Mark the definition as clean (e.g. after save/export). */
  resetDirty: () => void;

  /** Update per-node execution status. */
  setExecutionState: (state: ExecutionState) => void;

  /** Reset execution state to all idle. */
  resetExecution: () => void;

  /** Replace the definition directly (for code editor sync). */
  setDefinition: (definition: Definition) => void;
}

// ---------------------------------------------------------------------------
// Full store type
// ---------------------------------------------------------------------------

type EditorStore = EditorState & EditorActions;

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
export type { EditorStore, EditorState, EditorActions, NodeExecutionStatus, ExecutionState };
