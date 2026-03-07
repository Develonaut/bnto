"use client";

import type { ReactNode } from "react";
import { Canvas } from "./Canvas";
import { EditorOverlay } from "./EditorOverlay";
import { useEditorCanvas } from "./useEditorCanvas";
import { usePlaceholderNodes } from "../../hooks/usePlaceholderNodes";
import { useExecutionNodes } from "../../hooks/useExecutionNodes";

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

  return (
    <div className="relative h-full overflow-hidden" data-testid="recipe-editor">
      <Canvas
        nodes={displayNodes}
        onNodesChange={handleNodesChange}
        edges={edges}
        onEdgesChange={onEdgesChange}
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
