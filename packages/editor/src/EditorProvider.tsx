/**
 * EditorProvider — creates a ReactEditorInstance and provides it via context.
 *
 * Wraps children with ReactFlowProvider and EditorContext. The instance
 * is created once on mount via useState initializer.
 */

"use client";

import { useState, type ReactNode } from "react";
import { ReactFlowProvider } from "@xyflow/react";
import type { Definition } from "@bnto/nodes";
import { createReactEditor } from "./createReactEditor";
import { EditorContext } from "./context";
import type { EditorContextValue } from "./context";

interface EditorProviderProps {
  definition?: Definition;
  children: ReactNode;
}

function EditorProvider({ definition, children }: EditorProviderProps) {
  const [ctx] = useState<EditorContextValue>(() => createReactEditor(definition));

  return (
    <EditorContext.Provider value={ctx}>
      <ReactFlowProvider>{children}</ReactFlowProvider>
    </EditorContext.Provider>
  );
}

export { EditorProvider };
