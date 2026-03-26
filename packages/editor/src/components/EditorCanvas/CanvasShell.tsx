"use client";

import { useCallback, type ReactNode } from "react";
import { Canvas } from "./Canvas";
import { EditorOverlay } from "./EditorOverlay";
import { useEditorCanvas } from "./useEditorCanvas";
import { useEditor } from "../../context";
import { useLayoutNodes } from "../../hooks/useLayoutNodes";
import { useExecutionNodes } from "../../hooks/useExecutionNodes";
import { usePlaceholderNodes } from "../../hooks/usePlaceholderNodes";
import { useAddDividerNodes } from "../../hooks/useAddDividerNodes";
import { useEditorShortcuts } from "../../hooks/useEditorShortcuts";

interface CanvasShellProps {
  children?: ReactNode;
}

/** Compose the rendering pipeline from raw store nodes to display-ready nodes. */
function useRenderingPipeline(canvasState: ReturnType<typeof useEditorCanvas>) {
  const layouted = useLayoutNodes(canvasState.nodes);
  const status = useExecutionNodes(layouted);
  const placeholder = usePlaceholderNodes(status, canvasState.onNodesChange);
  const display = useAddDividerNodes(placeholder.displayNodes);
  return { display, handleNodesChange: placeholder.handleNodesChange };
}

function CanvasShell({ children }: CanvasShellProps) {
  const canvasState = useEditorCanvas();
  const { display, handleNodesChange } = useRenderingPipeline(canvasState);
  const editor = useEditor();
  useEditorShortcuts();

  const handleNodeClick = useCallback(
    (nodeId: string) => editor.nodes.selectNode(nodeId),
    [editor],
  );
  const handlePaneClick = useCallback(() => editor.panels.closePanel("config"), [editor]);

  return (
    <CanvasShellLayout>
      <Canvas
        nodes={display}
        onNodesChange={handleNodesChange}
        edges={canvasState.edges}
        onEdgesChange={canvasState.onEdgesChange}
        onNodeClick={handleNodeClick}
        onPaneClick={handlePaneClick}
        interactive
        disable={{ drag: true }}
        standalone
        className="h-full rounded-none border-0"
      />
      <EditorOverlay>{children}</EditorOverlay>
    </CanvasShellLayout>
  );
}

/** Layout wrapper with data-testid and a11y attributes. */
function CanvasShellLayout({ children }: { children: ReactNode }) {
  return (
    <div
      className="relative h-full overflow-hidden"
      data-testid="recipe-editor"
      role="application"
      aria-label="Recipe editor canvas"
    >
      {children}
    </div>
  );
}

export { CanvasShell };
