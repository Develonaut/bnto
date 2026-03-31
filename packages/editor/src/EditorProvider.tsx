/**
 * EditorProvider — creates a ReactEditorInstance and provides it via context.
 *
 * Wraps children with ReactFlowProvider and EditorContext. The instance
 * is created once on mount via useState initializer.
 */

"use client";

import { useState, useEffect, type ReactNode } from "react";
import { ReactFlowProvider } from "@xyflow/react";
import type { Definition } from "@bnto/core";
import { createReactEditor } from "./createReactEditor";
import { EditorContext } from "./context";
import type { EditorContextValue } from "./context";
import { registerEditorDebug } from "./debug/registerEditorDebug";

interface EditorProviderProps {
  definition?: Definition;
  initialFiles?: File[];
  children: ReactNode;
}

function EditorProvider({ definition, initialFiles, children }: EditorProviderProps) {
  const [ctx] = useState<EditorContextValue>(() => createReactEditor(definition));

  useEffect(() => {
    return registerEditorDebug(ctx.storeApi, ctx.instance);
  }, [ctx]);

  useEffect(() => {
    if (initialFiles?.length) {
      ctx.instance.execution.setInputFiles(initialFiles);
      ctx.instance.panels.openPanel("run");
    }
  }, []);

  return (
    <EditorContext.Provider value={ctx}>
      <ReactFlowProvider>{children}</ReactFlowProvider>
    </EditorContext.Provider>
  );
}

export { EditorProvider };
