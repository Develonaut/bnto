/**
 * Editor store factory — owns state and simple setters (controlled mode).
 *
 * The store is the state layer only. Business logic (addNode, removeNode,
 * updateParams) lives in pure action functions (editor/actions/) that take
 * EditorState and return Partial<EditorState>. Hooks are thin wrappers
 * that bridge actions to the store. See editor/actions/ + editor/hooks/
 * for the three-layer pattern:
 *
 *   Pure actions → Thin wrapper hooks → Consumer hooks (useEditorActions)
 *
 * The store owns: nodes, edges, configs, recipe metadata, undo/redo,
 * validation, execution state, and dirty flag.
 *
 * ReactFlow receives nodes/edges as props (controlled mode).
 */

import { createEnhancedStore } from "@bnto/core";
import { applyNodeChanges, applyEdgeChanges } from "@xyflow/react";
import type { EditorStore } from "./types";
import { captureSnapshot } from "./captureSnapshot";
import { pushToStack } from "./pushToStack";
import { revalidateState } from "./revalidateState";
import { resolveInitialState } from "./resolveInitialState";
import { loadRecipe } from "../actions/loadRecipe";
import { createBlank } from "../actions/createBlank";

// ---------------------------------------------------------------------------
// Store factory
// ---------------------------------------------------------------------------

function createEditorStore(slug?: string) {
  const initial = resolveInitialState(slug);

  return createEnhancedStore<EditorStore>()((set, get) => ({
    // --- Initial state ---
    nodes: initial.selectedNodeId
      ? initial.nodes.map((n) => (n.id === initial.selectedNodeId ? { ...n, selected: true } : n))
      : initial.nodes,
    edges: [],
    configs: initial.configs,
    slug: initial.slug,
    recipeMetadata: initial.metadata,
    isDirty: false,
    validationErrors: [],
    executionState: {},
    undoStack: [],
    redoStack: [],
    selectedNodeId: initial.selectedNodeId,
    layersOpen: false,
    configOpen: initial.selectedNodeId !== null,
    paletteOpen: false,

    // --- Entry points ---

    loadRecipe: (slug) => {
      const result = loadRecipe(slug);
      if (result) set(result);
    },

    createBlank: () => {
      set(createBlank());
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

    // --- Graph setters ---

    setNodes: (nodes) => {
      set({ nodes });
    },

    selectNode: (id) => {
      set((s) => ({
        nodes: s.nodes.map((n) =>
          n.id === id
            ? n.selected
              ? n
              : { ...n, selected: true }
            : n.selected
              ? { ...n, selected: false }
              : n,
        ),
        /* Auto-open config panel when selecting a node. */
        ...(id ? { configOpen: true } : {}),
      }));
    },

    // --- Config setters ---

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

    // --- History ---

    pushUndo: () => {
      const state = get();
      const snapshot = captureSnapshot(state.nodes, state.configs);
      set({
        undoStack: pushToStack(state.undoStack, snapshot),
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

    // --- Selection ---

    setSelectedNodeId: (id) => {
      set({ selectedNodeId: id, ...(id ? { configOpen: true } : {}) });
    },

    // --- Panel visibility ---

    toggleLayers: () => {
      set((s) => ({ layersOpen: !s.layersOpen }));
    },

    toggleConfig: () => {
      set((s) => ({ configOpen: !s.configOpen }));
    },

    openConfig: () => {
      set({ configOpen: true });
    },

    openPalette: () => {
      set({ paletteOpen: true });
    },

    closePalette: () => {
      set({ paletteOpen: false });
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
