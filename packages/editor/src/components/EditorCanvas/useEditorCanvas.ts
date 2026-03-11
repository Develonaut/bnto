"use client";

import { useStore } from "zustand";
import { useEditorStoreApi } from "../../context";
import { useEditorSelection } from "../../hooks/useEditorSelection";

/**
 * useEditorCanvas — bridges RF selection events → store and
 * provides canvas props (nodes, edges, change handlers).
 *
 * Called once in CanvasShell. Selection bridge must be called
 * inside ReactFlowProvider, which CanvasShell lives within.
 */

function useEditorCanvas() {
  const storeApi = useEditorStoreApi();
  useEditorSelection();

  const nodes = useStore(storeApi, (s) => s.nodes);
  const edges = useStore(storeApi, (s) => s.edges);
  const onNodesChange = useStore(storeApi, (s) => s.onNodesChange);
  const onEdgesChange = useStore(storeApi, (s) => s.onEdgesChange);

  return { nodes, edges, onNodesChange, onEdgesChange };
}

export { useEditorCanvas };
