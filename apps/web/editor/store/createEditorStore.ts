/**
 * Editor store factory — owns all editor state (controlled mode).
 *
 * The store owns: nodes, edges, configs, recipe metadata, undo/redo,
 * validation, execution state, and dirty flag.
 *
 * ReactFlow receives nodes/edges as props (controlled mode).
 * Parameter changes update configs only — no RF re-render.
 */

import { createStore } from "zustand/vanilla";
import { applyNodeChanges, applyEdgeChanges } from "@xyflow/react";
import {
  createBlankDefinition,
  getRecipeBySlug,
  validateDefinition,
} from "@bnto/nodes";
import type { NodeTypeName } from "@bnto/nodes";
import { definitionToBento } from "../adapters/definitionToBento";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";
import { createCompartmentNode } from "../adapters/createCompartmentNode";
import type { EditorStore, EditorSnapshot, RecipeMetadata } from "./types";
import type { BentoNode, NodeConfigs } from "../adapters/types";

// ---------------------------------------------------------------------------
// History helpers
// ---------------------------------------------------------------------------

const MAX_UNDO_HISTORY = 50;

function captureSnapshot(nodes: BentoNode[], configs: NodeConfigs): EditorSnapshot {
  return { nodes: [...nodes], configs: { ...configs } };
}

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
// Revalidation helper
// ---------------------------------------------------------------------------

function revalidateState(nodes: BentoNode[], configs: NodeConfigs, metadata: RecipeMetadata) {
  const definition = rfNodesToDefinition(nodes, {
    ...metadata,
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
  }, configs);
  return validateDefinition(definition);
}

// ---------------------------------------------------------------------------
// Store factory
// ---------------------------------------------------------------------------

interface CreateEditorStoreOptions {
  initialMetadata?: RecipeMetadata;
  initialNodes?: BentoNode[];
  initialConfigs?: NodeConfigs;
}

function createEditorStore(options?: CreateEditorStoreOptions | RecipeMetadata) {
  // Support both old signature (RecipeMetadata) and new (options object).
  // RecipeMetadata has `id` + `name` + `type` + `version` as required fields.
  // CreateEditorStoreOptions has optional `initialMetadata`, `initialNodes`, `initialConfigs`.
  const isLegacy = options && "id" in options && "version" in options;
  const opts: CreateEditorStoreOptions = isLegacy
    ? { initialMetadata: options as RecipeMetadata }
    : (options as CreateEditorStoreOptions | undefined) ?? {};

  const metadata = opts.initialMetadata ?? metadataFromBlank();
  const initNodes = opts.initialNodes ?? [];
  const initConfigs = opts.initialConfigs ?? {};

  return createStore<EditorStore>()((set, get) => ({
    // --- Initial state ---
    nodes: initNodes,
    edges: [],
    configs: initConfigs,
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
      const { nodes, configs } = definitionToBento(recipe.definition);
      set({
        nodes,
        configs,
        recipeMetadata: metadataFromDefinition(recipe.definition),
        isDirty: false,
        validationErrors: validateDefinition(recipe.definition),
        executionState: {},
        undoStack: [],
        redoStack: [],
      });
    },

    createBlank: () => {
      const blank = definitionToBento(createBlankDefinition());
      set({
        nodes: blank.nodes,
        configs: blank.configs,
        recipeMetadata: metadataFromBlank(),
        isDirty: false,
        validationErrors: [],
        executionState: {},
        undoStack: [],
        redoStack: [],
      });
    },

    // --- RF controlled-mode change handlers ---

    onNodesChange: (changes) => {
      set((s) => ({
        nodes: applyNodeChanges(changes, s.nodes),
      }));
    },

    onEdgesChange: (changes) => {
      set((s) => ({
        edges: applyEdgeChanges(changes, s.edges),
      }));
    },

    // --- Graph mutations ---

    setNodes: (nodes) => {
      set({ nodes });
    },

    addNode: (type: NodeTypeName, position?) => {
      const state = get();
      const slotIndex = state.nodes.length;
      const result = createCompartmentNode(type, slotIndex, position);
      if (!result) return null;

      const snapshot = captureSnapshot(state.nodes, state.configs);
      const stack = [...state.undoStack, snapshot];

      set({
        nodes: [...state.nodes, result.node],
        configs: { ...state.configs, [result.node.id]: result.config },
        isDirty: true,
        undoStack: stack.length > MAX_UNDO_HISTORY ? stack.slice(-MAX_UNDO_HISTORY) : stack,
        redoStack: [],
        validationErrors: revalidateState(
          [...state.nodes, result.node],
          { ...state.configs, [result.node.id]: result.config },
          state.recipeMetadata,
        ),
      });
      return result.node.id;
    },

    removeNode: (id) => {
      const state = get();
      const snapshot = captureSnapshot(state.nodes, state.configs);
      const stack = [...state.undoStack, snapshot];
      const nextNodes = state.nodes.filter((n) => n.id !== id);
      const nextConfigs = { ...state.configs };
      delete nextConfigs[id];

      set({
        nodes: nextNodes,
        configs: nextConfigs,
        isDirty: true,
        undoStack: stack.length > MAX_UNDO_HISTORY ? stack.slice(-MAX_UNDO_HISTORY) : stack,
        redoStack: [],
        validationErrors: revalidateState(nextNodes, nextConfigs, state.recipeMetadata),
      });
    },

    // --- Config mutations (no RF re-render) ---

    setConfigs: (configs) => {
      set({ configs });
    },

    setConfig: (nodeId, config) => {
      set((s) => ({
        configs: { ...s.configs, [nodeId]: config },
      }));
    },

    removeConfig: (nodeId) => {
      set((s) => {
        const next = { ...s.configs };
        delete next[nodeId];
        return { configs: next };
      });
    },

    updateConfigParams: (nodeId, params) => {
      const state = get();
      const existing = state.configs[nodeId];
      if (!existing) return;

      const snapshot = captureSnapshot(state.nodes, state.configs);
      const stack = [...state.undoStack, snapshot];
      const nextConfigs = {
        ...state.configs,
        [nodeId]: {
          ...existing,
          parameters: { ...existing.parameters, ...params },
        },
      };

      set({
        configs: nextConfigs,
        isDirty: true,
        undoStack: stack.length > MAX_UNDO_HISTORY ? stack.slice(-MAX_UNDO_HISTORY) : stack,
        redoStack: [],
        validationErrors: revalidateState(state.nodes, nextConfigs, state.recipeMetadata),
      });
    },

    // --- History ---

    pushUndo: () => {
      const state = get();
      const snapshot = captureSnapshot(state.nodes, state.configs);
      const stack = [...state.undoStack, snapshot];
      set({
        undoStack: stack.length > MAX_UNDO_HISTORY ? stack.slice(-MAX_UNDO_HISTORY) : stack,
        redoStack: [],
      });
    },

    undo: () => {
      const state = get();
      if (state.undoStack.length === 0) return;
      const snapshot = state.undoStack[state.undoStack.length - 1]!;
      const current = captureSnapshot(state.nodes, state.configs);
      set({
        nodes: snapshot.nodes,
        configs: snapshot.configs,
        isDirty: true,
        undoStack: state.undoStack.slice(0, -1),
        redoStack: [...state.redoStack, current],
        validationErrors: revalidateState(snapshot.nodes, snapshot.configs, state.recipeMetadata),
      });
    },

    redo: () => {
      const state = get();
      if (state.redoStack.length === 0) return;
      const snapshot = state.redoStack[state.redoStack.length - 1]!;
      const current = captureSnapshot(state.nodes, state.configs);
      set({
        nodes: snapshot.nodes,
        configs: snapshot.configs,
        isDirty: true,
        undoStack: [...state.undoStack, current],
        redoStack: state.redoStack.slice(0, -1),
        validationErrors: revalidateState(snapshot.nodes, snapshot.configs, state.recipeMetadata),
      });
    },

    // --- Utility ---

    markDirty: () => {
      set({ isDirty: true });
    },

    revalidate: () => {
      const state = get();
      set({
        validationErrors: revalidateState(state.nodes, state.configs, state.recipeMetadata),
      });
    },

    resetDirty: () => {
      set({ isDirty: false });
    },

    setExecutionState: (executionState) => {
      set({ executionState });
    },

    resetExecution: () => {
      set({ executionState: {} });
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
