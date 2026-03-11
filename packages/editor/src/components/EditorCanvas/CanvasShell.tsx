"use client";

import { useCallback, type ReactNode } from "react";
import { Canvas } from "./Canvas";
import { EditorOverlay } from "./EditorOverlay";
import { useEditorCanvas } from "./useEditorCanvas";
import { useEditorStore } from "../../hooks/useEditorStore";
import { useLayoutNodes } from "../../hooks/useLayoutNodes";
import { useExecutionNodes } from "../../hooks/useExecutionNodes";
import { usePlaceholderNodes } from "../../hooks/usePlaceholderNodes";
import { useAddDividerNodes } from "../../hooks/useAddDividerNodes";

/**
 * CanvasShell — the canvas surface with floating overlay children.
 *
 * Renders the ReactFlow bento canvas and wraps children in the
 * overlay container (pointer-events-none layer). Children position
 * themselves within the overlay (left panel, right panel, toolbar).
 *
 * Node rendering pipeline (each step is a memoized hook):
 *   store.nodes → layout → execution status → placeholder → dividers → RF
 */

interface CanvasShellProps {
  children?: ReactNode;
}

function CanvasShell({ children }: CanvasShellProps) {
  const { nodes, edges, onNodesChange, onEdgesChange } = useEditorCanvas();
  const layoutedNodes = useLayoutNodes(nodes);
  const statusNodes = useExecutionNodes(layoutedNodes);
  const { displayNodes, handleNodesChange } = usePlaceholderNodes(statusNodes, onNodesChange);
  const nodesWithDividers = useAddDividerNodes(displayNodes);
  const selectNode = useEditorStore((s) => s.selectNode);

  const handleNodeClick = useCallback(
    (nodeId: string) => {
      selectNode(nodeId);
    },
    [selectNode],
  );

  return (
    <div className="relative h-full overflow-hidden" data-testid="recipe-editor">
      <Canvas
        nodes={nodesWithDividers}
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
