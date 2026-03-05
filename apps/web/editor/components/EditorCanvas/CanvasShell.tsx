"use client";

import type { ReactNode } from "react";
import { BentoCanvas } from "../canvas/BentoCanvas";
import { EditorOverlay } from "./EditorOverlay";
import { useEditorCanvas } from "./useEditorCanvas";

/**
 * CanvasShell — the canvas surface with floating overlay children.
 *
 * Renders the ReactFlow bento canvas and wraps children in the
 * overlay container (pointer-events-none layer). Children position
 * themselves within the overlay (left panel, right panel, toolbar).
 *
 *   <Editor.Canvas>
 *     <Editor.Layers />
 *     <Editor.Config />
 *     <Editor.Toolbar />
 *   </Editor.Canvas>
 */

interface CanvasShellProps {
  children?: ReactNode;
}

function CanvasShell({ children }: CanvasShellProps) {
  const { nodes, edges, onNodesChange, onEdgesChange } = useEditorCanvas();

  return (
    <div
      className="relative h-full overflow-hidden"
      data-testid="recipe-editor"
    >
      <BentoCanvas
        nodes={nodes}
        onNodesChange={onNodesChange}
        edges={edges}
        onEdgesChange={onEdgesChange}
        interactive
        disable={{ drag: true }}
        standalone
        className="h-full rounded-none border-0"
      />
      <EditorOverlay>
        {children}
      </EditorOverlay>
    </div>
  );
}

export { CanvasShell };
