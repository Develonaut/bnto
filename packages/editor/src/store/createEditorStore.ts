/**
 * Editor store factory — state layer with simple setters (controlled mode).
 *
 * Business logic lives in pure action functions (editor/actions/).
 * Hooks are thin wrappers bridging actions to the store.
 *
 *   Pure actions → Services → Clients (EditorInstance)
 */

import { createEnhancedStore, core } from "@bnto/core";
import { applyNodeChanges, applyEdgeChanges } from "@xyflow/react";
import type { Definition } from "@bnto/nodes";
import type { EditorStore, PanelId } from "./types";
import { captureSnapshot } from "./captureSnapshot";
import { pushToStack } from "./pushToStack";
import { revalidateState } from "./revalidateState";
import { resolveInitialState } from "./resolveInitialState";
import { loadDefinition } from "../actions/loadDefinition";
import { createBlank } from "../actions/createBlank";
import { runExecution } from "../actions/runExecution";
import { expandContainer } from "../actions/expandContainer";
import { collapseContainer } from "../actions/collapseContainer";
import { autoOpenConfig, autoCloseConfig, closeSameSideSiblings } from "./panelHelpers";

// ---------------------------------------------------------------------------
// Store factory
// ---------------------------------------------------------------------------

function createEditorStore(definition?: Definition) {
  const initial = resolveInitialState(definition);

  const store = createEnhancedStore<EditorStore>()((set, get) => ({
    // --- Initial state ---
    nodes: initial.selectedNodeId
      ? initial.nodes.map((n) => (n.id === initial.selectedNodeId ? { ...n, selected: true } : n))
      : initial.nodes,
    edges: [],
    configs: initial.configs,
    definition: initial.definition,
    recipeMetadata: initial.metadata,
    isDirty: false,
    validationErrors: [],
    executionState: {},
    nodeProgress: {},
    undoStack: [],
    redoStack: [],
    selectedNodeId: initial.selectedNodeId,
    panels: {
      config: initial.selectedNodeId !== null,
      palette: false,
      run: false,
    },
    executionPhase: "idle",
    executionResults: [],
    executionErrors: [],
    executionLogs: [],
    executionFileProgress: null,
    executionInputFiles: [],
    insertAfterNodeId: null,
    insertIntoContainerId: null,
    expandedContainerIds: new Set(),

    // --- Entry points ---

    loadDefinition: (def) => {
      set(loadDefinition(def));
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
        panels: id ? autoOpenConfig(s.panels) : autoCloseConfig(s.panels),
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
      const snapshot = captureSnapshot(
        state.nodes,
        state.configs,
        state.definition,
        state.expandedContainerIds,
      );
      set({
        undoStack: pushToStack(state.undoStack, snapshot),
        redoStack: [],
      });
    },

    undo: () => {
      const state = get();
      if (state.undoStack.length === 0) return;
      const snapshot = state.undoStack[state.undoStack.length - 1]!;
      const current = captureSnapshot(
        state.nodes,
        state.configs,
        state.definition,
        state.expandedContainerIds,
      );
      set({
        nodes: snapshot.nodes,
        configs: snapshot.configs,
        definition: snapshot.definition,
        expandedContainerIds: snapshot.expandedContainerIds,
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
      const current = captureSnapshot(
        state.nodes,
        state.configs,
        state.definition,
        state.expandedContainerIds,
      );
      set({
        nodes: snapshot.nodes,
        configs: snapshot.configs,
        definition: snapshot.definition,
        expandedContainerIds: snapshot.expandedContainerIds,
        isDirty: true,
        undoStack: [...state.undoStack, current],
        redoStack: state.redoStack.slice(0, -1),
        validationErrors: revalidateState(snapshot.nodes, snapshot.configs, state.recipeMetadata),
      });
    },

    // --- Selection ---

    setSelectedNodeId: (id) => {
      set((s) => ({
        selectedNodeId: id,
        panels: id ? autoOpenConfig(s.panels) : autoCloseConfig(s.panels),
      }));
    },

    // --- Panel visibility ---

    openPanel: (id: PanelId) => {
      set((s) => ({ panels: closeSameSideSiblings(s.panels, id) }));
    },

    closePanel: (id: PanelId) => {
      set((s) => ({ panels: { ...s.panels, [id]: false } }));
    },

    togglePanel: (id: PanelId) => {
      set((s) => {
        if (s.panels[id]) return { panels: { ...s.panels, [id]: false } };
        return { panels: closeSameSideSiblings(s.panels, id) };
      });
    },

    // --- Container expansion ---

    expandContainer: (nodeId) => {
      const state = get();
      const result = expandContainer(state, nodeId);
      if (result) set(result);
    },

    collapseContainer: (nodeId) => {
      const state = get();
      const result = collapseContainer(state, nodeId);
      if (result) set(result);
    },

    toggleContainerExpanded: (nodeId) => {
      const state = get();
      if (state.expandedContainerIds.has(nodeId)) {
        const result = collapseContainer(state, nodeId);
        if (result) set(result);
      } else {
        const result = expandContainer(state, nodeId);
        if (result) set(result);
      }
    },

    // --- Insertion context ---

    setInsertAfterNodeId: (id) => {
      set({ insertAfterNodeId: id });
    },

    setInsertIntoContainerId: (id) => {
      set({ insertIntoContainerId: id });
    },

    // --- Execution lifecycle ---

    runExecution: async (files) => {
      await runExecution(set, get, files);
    },

    resetRun: () => {
      set({
        executionState: {},
        nodeProgress: {},
        executionPhase: "idle",
        executionResults: [],
        executionErrors: [],
        executionLogs: [],
        executionFileProgress: null,
        executionInputFiles: [],
      });
    },

    downloadResult: (file) => {
      core.executions.downloadResult(file);
    },

    downloadAllResults: async () => {
      const results = get().executionResults;
      if (results.length === 0) return;
      await core.executions.downloadAllResults(results, "editor-results");
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

    resetNodeStatuses: () => {
      set({ executionState: {} });
    },

    setRecipeMetadata: (newMetadata) => {
      set({ recipeMetadata: newMetadata });
    },

    resetHistory: () => {
      set({ undoStack: [], redoStack: [] });
    },
  }));

  // Auto-expand all containers so children are always visible on init
  if (definition) {
    const containerNodes = store.getState().nodes.filter((n) => n.data.isContainer);
    for (const node of containerNodes) {
      store.getState().expandContainer(node.id);
    }
    // Clear undo/redo — initial expansion isn't undoable
    store.setState({ undoStack: [], redoStack: [], isDirty: false });
  }

  return store;
}

export { createEditorStore };
