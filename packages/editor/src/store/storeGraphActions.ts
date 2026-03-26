/** Store actions: RF change handlers, graph setters, config setters, selection. */

import { applyNodeChanges, applyEdgeChanges } from "@xyflow/react";
import type { EditorStore } from "./types";
import type { BentoNode } from "../adapters/types";
import { autoOpenConfig, autoCloseConfig } from "./panelHelpers";

type Setter = (partial: Partial<EditorStore> | ((s: EditorStore) => Partial<EditorStore>)) => void;

/** Toggle selection on a single node, deselect all others. */
function applyNodeSelection(nodes: BentoNode[], id: string | null): BentoNode[] {
  return nodes.map((n) =>
    n.id === id
      ? n.selected
        ? n
        : { ...n, selected: true }
      : n.selected
        ? { ...n, selected: false }
        : n,
  );
}

/** Resolve panel state for a selection change. */
function selectionPanels(panels: EditorStore["panels"], id: string | null) {
  return id ? autoOpenConfig(panels) : autoCloseConfig(panels);
}

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
        nodes: applyNodeSelection(s.nodes, id),
        panels: selectionPanels(s.panels, id),
      }));
    },
    setSelectedNodeId: (id: string | null) => {
      set((s) => ({ selectedNodeId: id, panels: selectionPanels(s.panels, id) }));
    },
  };
}

export { createGraphActions };
