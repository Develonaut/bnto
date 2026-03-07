"use client";

import type { ReactNode } from "react";
import { ReactFlowProvider } from "@xyflow/react";
import { EditorProvider } from "../../EditorProvider";
import { EditorExecutionProvider } from "../../hooks/EditorExecutionContext";

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
        <EditorExecutionProvider>{children}</EditorExecutionProvider>
      </ReactFlowProvider>
    </EditorProvider>
  );
}

export { EditorCanvasRoot };
