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
import type { Definition } from "@bnto/core";
import type { EditorStore, PanelId } from "./types";
import { resolveInitialState } from "./resolveInitialState";
import { revalidateState } from "./revalidateState";
import { loadDefinition } from "../actions/loadDefinition";
import { createBlank } from "../actions/createBlank";
import { runExecution } from "../actions/runExecution";
import { expandContainer } from "../actions/expandContainer";
import { collapseContainer } from "../actions/collapseContainer";
import { autoOpenConfig, autoCloseConfig, closeSameSideSiblings } from "./panelHelpers";
import { pushUndoAction, undoAction, redoAction } from "./historyActions";
import { EXECUTION_DEFAULTS } from "./executionDefaults";

// ---------------------------------------------------------------------------
// Store factory
// ---------------------------------------------------------------------------

function createEditorStore(definition?: Definition, cloudId?: string) {
  const initial = resolveInitialState(definition, cloudId);

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
    ...EXECUTION_DEFAULTS,
    undoStack: [],
    redoStack: [],
    selectedNodeId: initial.selectedNodeId,
    panels: {
      config: initial.selectedNodeId !== null,
      palette: false,
      run: false,
      help: false,
    },
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
      set((s) => ({ nodes: applyNodeChanges(changes, s.nodes) }));
    },

    onEdgesChange: (changes) => {
      set((s) => ({ edges: applyEdgeChanges(changes, s.edges) }));
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
        selectedNodeId: id,
        panels: id ? autoOpenConfig(s.panels) : autoCloseConfig(s.panels),
      }));
    },

    // --- Config setters ---

    setConfigs: (configs) => {
      set({ configs });
    },

    setConfig: (nodeId, config) => {
      set((s) => ({ configs: { ...s.configs, [nodeId]: config } }));
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
      set(pushUndoAction(get()));
    },

    undo: () => {
      const result = undoAction(get());
      if (result) set(result);
    },

    redo: () => {
      const result = redoAction(get());
      if (result) set(result);
    },

    // --- Selection ---

    setSelectedNodeId: (id) => {
      set((s) => ({
        selectedNodeId: id,
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
      const result = expandContainer(get(), nodeId);
      if (result) set(result);
    },

    collapseContainer: (nodeId) => {
      const result = collapseContainer(get(), nodeId);
      if (result) set(result);
    },

    toggleContainerExpanded: (nodeId) => {
      const state = get();
      const action = state.expandedContainerIds.has(nodeId) ? collapseContainer : expandContainer;
      const result = action(state, nodeId);
      if (result) set(result);
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
      set(EXECUTION_DEFAULTS);
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

    revalidate: () => {
      const state = get();
      set({
        validationErrors: revalidateState(state.nodes, state.configs, state.recipeMetadata),
      });
    },

    setExecutionState: (executionState) => {
      set({ executionState });
    },

    resetNodeStatuses: () => {
      set({ executionState: {} });
    },

    setRecipeMetadata: (newMetadata) => {
      set({ recipeMetadata: newMetadata, isDirty: true });
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
