"use client";

import { type ReactNode, useCallback } from "react";
import { Canvas } from "./Canvas";
import { EditorOverlay } from "./EditorOverlay";
import { useEditorCanvas } from "./useEditorCanvas";
import { usePlaceholderNodes } from "../../hooks/usePlaceholderNodes";
import { useExecutionNodes } from "../../hooks/useExecutionNodes";
import { useEditorStore } from "../../hooks/useEditorStore";
import { PLACEHOLDER_ID } from "../../helpers/injectPlaceholder";
import type { BentoNode } from "../../adapters/types";

/**
 * CanvasShell — the canvas surface with floating overlay children.
 *
 * Renders the ReactFlow bento canvas and wraps children in the
 * overlay container (pointer-events-none layer). Children position
 * themselves within the overlay (left panel, right panel, toolbar).
 */

interface CanvasShellProps {
  children?: ReactNode;
}

function CanvasShell({ children }: CanvasShellProps) {
  const { nodes, edges, onNodesChange, onEdgesChange } = useEditorCanvas();
  const statusNodes = useExecutionNodes(nodes);
  const { displayNodes, handleNodesChange } = usePlaceholderNodes(statusNodes, onNodesChange);
  const setSelectedNodeId = useEditorStore((s) => s.setSelectedNodeId);

  const handleNodeClick = useCallback(
    (event: React.MouseEvent, node: BentoNode) => {
      // Skip placeholder clicks — it's not a real node.
      if (node.id === PLACEHOLDER_ID) return;
      // Skip clicks originating from interactive elements inside nodes
      // (buttons, inputs, etc.) — those have their own handlers.
      const target = event.target as HTMLElement;
      if (target.closest("button, input, select, textarea, [role='button']")) return;
      setSelectedNodeId(node.id);
    },
    [setSelectedNodeId],
  );

  return (
    <div className="relative h-full overflow-hidden" data-testid="recipe-editor">
      <Canvas
        nodes={displayNodes}
        onNodesChange={handleNodesChange}
        edges={edges}
        onEdgesChange={onEdgesChange}
        onNodeClick={handleNodeClick}
        interactive
        disable={{ drag: true }}
        standalone
        className="h-full rounded-none border-0"
      />
      <EditorOverlay>{children}</EditorOverlay>
    </div>
  );
}

export { CanvasShell };
