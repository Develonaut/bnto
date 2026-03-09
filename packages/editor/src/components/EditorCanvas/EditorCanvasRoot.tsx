"use client";

import { useState, type ReactNode } from "react";
import { ReactFlowProvider } from "@xyflow/react";
import type { Definition } from "@bnto/nodes";
import { initEditorStore } from "../../store/instance";

/**
 * EditorCanvasRoot — initializes the editor store and wraps with ReactFlowProvider.
 *
 * The store is created once on mount via the module-level instance.
 * No React context needed — hooks access the store directly.
 *
 * The editor accepts a Definition directly. Slug lookup belongs at
 * the app layer (the page resolves slug → definition before rendering).
 *
 *   <EditorRoot definition={recipe.definition}>
 *     <EditorCanvas />
 *   </EditorRoot>
 */

interface EditorCanvasRootProps {
  definition?: Definition;
  children: ReactNode;
}

function EditorCanvasRoot({ definition, children }: EditorCanvasRootProps) {
  // Initialize the module-level store once on mount.
  // useState ensures this only runs once per component lifecycle.
  useState(() => initEditorStore(definition));

  return <ReactFlowProvider>{children}</ReactFlowProvider>;
}

export { EditorCanvasRoot };
