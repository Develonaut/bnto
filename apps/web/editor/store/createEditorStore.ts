/**
 * Editor store factory — thin state machine for recipe editing.
 *
 * ReactFlow is the single source of truth for node state.
 * This store only owns: recipe metadata, undo/redo history,
 * validation errors, execution state, and dirty flag.
 *
 * Mutations (add/remove/update) happen directly in RF via hooks.
 * The hooks call pushUndo/markDirty/revalidate on this store.
 */

import { createStore } from "zustand/vanilla";
import {
  createBlankDefinition,
  getRecipeBySlug,
  validateDefinition,
} from "@bnto/nodes";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";
import type { EditorStore, RecipeMetadata, NodeGetter } from "./types";
import type { BentoNode } from "../adapters/types";

// ---------------------------------------------------------------------------
// History helpers
// ---------------------------------------------------------------------------

const MAX_UNDO_HISTORY = 50;

// ---------------------------------------------------------------------------
// Metadata helpers
// ---------------------------------------------------------------------------

function metadataFromBlank(): RecipeMetadata {
  const def = createBlankDefinition();
  return { id: def.id, name: def.name, type: def.type, version: def.version };
}

function metadataFromDefinition(def: { id: string; name: string; type: string; version: string }): RecipeMetadata {
  return { id: def.id, name: def.name, type: def.type, version: def.version };
}

// ---------------------------------------------------------------------------
// Store factory
// ---------------------------------------------------------------------------

function createEditorStore(initialMetadata?: RecipeMetadata) {
  const metadata = initialMetadata ?? metadataFromBlank();

  // Closure — not in Zustand state, avoids serialization issues.
  let nodeGetter: NodeGetter | null = null;

  return createStore<EditorStore>()((set, get) => ({
    // Initial state
    recipeMetadata: metadata,
    isDirty: false,
    validationErrors: [],
    executionState: {},
    undoStack: [],
    redoStack: [],

    // --- Entry points ---

    loadRecipe: (slug) => {
      const recipe = getRecipeBySlug(slug);
      if (!recipe) return;
      set({
        recipeMetadata: metadataFromDefinition(recipe.definition),
        isDirty: false,
        validationErrors: validateDefinition(recipe.definition),
        executionState: {},
        undoStack: [],
        redoStack: [],
      });
    },

    createBlank: () => {
      set({
        recipeMetadata: metadataFromBlank(),
        isDirty: false,
        validationErrors: [],
        executionState: {},
        undoStack: [],
        redoStack: [],
      });
    },

    // --- Thin actions for RF-first hooks ---

    pushUndo: () => {
      const nodes = nodeGetter ? nodeGetter() : [];
      const state = get();
      const stack = [...state.undoStack, nodes];
      set({
        undoStack: stack.length > MAX_UNDO_HISTORY ? stack.slice(-MAX_UNDO_HISTORY) : stack,
        redoStack: [],
      });
    },

    markDirty: () => {
      set({ isDirty: true });
    },

    revalidate: () => {
      const nodes = nodeGetter ? nodeGetter() : [];
      const definition = rfNodesToDefinition(nodes as BentoNode[], {
        ...get().recipeMetadata,
        position: { x: 0, y: 0 },
        metadata: {},
        parameters: {},
        inputPorts: [],
        outputPorts: [],
      });
      set({ validationErrors: validateDefinition(definition) });
    },

    // --- History ---

    undo: () => {
      const state = get();
      if (state.undoStack.length === 0) return null;
      const snapshot = state.undoStack[state.undoStack.length - 1]!;
      const currentNodes = nodeGetter ? nodeGetter() : [];
      set({
        isDirty: true,
        undoStack: state.undoStack.slice(0, -1),
        redoStack: [...state.redoStack, currentNodes],
      });
      return snapshot;
    },

    redo: () => {
      const state = get();
      if (state.redoStack.length === 0) return null;
      const snapshot = state.redoStack[state.redoStack.length - 1]!;
      const currentNodes = nodeGetter ? nodeGetter() : [];
      set({
        isDirty: true,
        undoStack: [...state.undoStack, currentNodes],
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

    setNodeGetter: (getter) => {
      nodeGetter = getter;
    },

    setRecipeMetadata: (newMetadata) => {
      set({ recipeMetadata: newMetadata });
    },

    resetHistory: () => {
      set({ undoStack: [], redoStack: [] });
    },
  }));
}

export { createEditorStore };
