"use client";

import type { ReactNode } from "react";
import { ReactFlowProvider } from "@xyflow/react";
import { EditorProvider } from "../../EditorProvider";

/**
 * EditorCanvasRoot — editor providers shell.
 *
 * Wraps children with EditorProvider (store) + ReactFlowProvider.
 * Compose visual parts as children:
 *
 *   <EditorRoot slug="compress-images">
 *     <EditorCanvas />
 *   </EditorRoot>
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
