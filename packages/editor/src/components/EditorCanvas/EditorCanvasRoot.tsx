"use client";

import type { ReactNode } from "react";
import type { Definition } from "@bnto/nodes";
import { EditorProvider } from "../../EditorProvider";

/**
 * EditorCanvasRoot — the entry point for the recipe editor.
 *
 * Delegates to EditorProvider which creates the editor instance,
 * provides it via context, wraps with ReactFlowProvider, and sets
 * the compat bridge so old hooks continue to work.
 *
 *   <EditorRoot definition={recipe.definition}>
 *     <EditorCanvas />
 *   </EditorRoot>
 */

interface EditorCanvasRootProps {
  definition?: Definition;
  cloudId?: string;
  children: ReactNode;
}

function EditorCanvasRoot({ definition, cloudId, children }: EditorCanvasRootProps) {
  return (
    <EditorProvider definition={definition} cloudId={cloudId}>
      {children}
    </EditorProvider>
  );
}

export { EditorCanvasRoot };
