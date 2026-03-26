/** Store actions: RF change handlers, graph setters, config setters, selection. */

import { applyNodeChanges, applyEdgeChanges } from "@xyflow/react";
import type { EditorStore } from "./types";
import { autoOpenConfig, autoCloseConfig } from "./panelHelpers";

type Setter = (partial: Partial<EditorStore> | ((s: EditorStore) => Partial<EditorStore>)) => void;

function createGraphActions(set: Setter) {
  return {
    onNodesChange: (changes: Parameters<EditorStore["onNodesChange"]>[0]) => {
      set((s) => ({ nodes: applyNodeChanges(changes, s.nodes) }));
    },

    onEdgesChange: (changes: Parameters<EditorStore["onEdgesChange"]>[0]) => {
      set((s) => ({ edges: applyEdgeChanges(changes, s.edges) }));
    },

    setNodes: (nodes: Parameters<EditorStore["setNodes"]>[0]) => {
      set({ nodes });
    },

    selectNode: (id: string | null) => {
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

    setSelectedNodeId: (id: string | null) => {
      set((s) => ({
        selectedNodeId: id,
        panels: id ? autoOpenConfig(s.panels) : autoCloseConfig(s.panels),
      }));
    },
  };
}

export { createGraphActions };
