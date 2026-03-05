"use client";

import { useEditorStore } from "../../hooks/useEditorStore";
import { useEditorSelection } from "../../hooks/useEditorSelection";

/**
 * useEditorCanvas — bridges RF selection events → store and
 * provides canvas props (nodes, edges, change handlers).
 *
 * Called once in CanvasShell. Selection bridge must be called
 * inside ReactFlowProvider, which CanvasShell lives within.
 */

function useEditorCanvas() {
  useEditorSelection();

  const nodes = useEditorStore((s) => s.nodes);
  const edges = useEditorStore((s) => s.edges);
  const onNodesChange = useEditorStore((s) => s.onNodesChange);
  const onEdgesChange = useEditorStore((s) => s.onEdgesChange);

  return { nodes, edges, onNodesChange, onEdgesChange };
}

export { useEditorCanvas };
