"use client";

import { useState, type ReactNode } from "react";
import { ReactFlowProvider } from "@xyflow/react";
import { initEditorStore } from "../../store/instance";

/**
 * EditorCanvasRoot — initializes the editor store and wraps with ReactFlowProvider.
 *
 * The store is created once on mount via the module-level instance.
 * No React context needed — hooks access the store directly.
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
  // Initialize the module-level store once on mount.
  // useState ensures this only runs once per component lifecycle.
  useState(() => initEditorStore(slug));

  return <ReactFlowProvider>{children}</ReactFlowProvider>;
}

export { EditorCanvasRoot };
