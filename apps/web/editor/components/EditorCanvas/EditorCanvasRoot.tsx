"use client";

import type { ReactNode } from "react";
import { ReactFlowProvider } from "@xyflow/react";
import { EditorProvider } from "@/editor/EditorProvider";

/**
 * EditorCanvasRoot — editor providers shell.
 *
 * Wraps children with EditorProvider (store) + ReactFlowProvider.
 * Compose visual parts as children:
 *
 *   <Editor.Root slug="compress-images">
 *     <Editor.Canvas />
 *   </Editor.Root>
 */

interface EditorCanvasRootProps {
  slug?: string;
  children: ReactNode;
}

function EditorCanvasRoot({ slug, children }: EditorCanvasRootProps) {
  return (
    <EditorProvider slug={slug}>
      <ReactFlowProvider>
        {children}
      </ReactFlowProvider>
    </EditorProvider>
  );
}

export { EditorCanvasRoot };
