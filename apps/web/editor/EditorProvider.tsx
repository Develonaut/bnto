/**
 * EditorProvider — creates and provides the editor store to the React tree.
 *
 * Factory pattern: each EditorProvider creates its own store instance.
 * Multiple editors on the same page get independent stores.
 */

"use client";

import { useState, type ReactNode } from "react";
import { EditorContext } from "./context";
import { createEditorStore } from "./store/createEditorStore";
import type { RecipeMetadata } from "./store/types";

interface EditorProviderProps {
  /** Initial recipe metadata. Defaults to blank if omitted. */
  initialMetadata?: RecipeMetadata;
  children: ReactNode;
}

function EditorProvider({ initialMetadata, children }: EditorProviderProps) {
  // useState with initializer — creates the store once on first render.
  // Avoids useRef .current access during render (react-hooks/refs).
  const [store] = useState(() => createEditorStore(initialMetadata));

  return (
    <EditorContext.Provider value={store}>
      {children}
    </EditorContext.Provider>
  );
}

export { EditorProvider };
