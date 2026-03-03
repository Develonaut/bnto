/**
 * EditorProvider — creates and provides the editor store to the React tree.
 *
 * Factory pattern: each EditorProvider creates its own store instance.
 * Multiple editors on the same page get independent stores.
 */

"use client";

import { useState, type ReactNode } from "react";
import type { Definition } from "@bnto/nodes";
import { EditorContext } from "./context";
import { createEditorStore } from "./store/createEditorStore";

interface EditorProviderProps {
  /** Initial definition. Defaults to blank canvas if omitted. */
  initialDefinition?: Definition;
  children: ReactNode;
}

function EditorProvider({ initialDefinition, children }: EditorProviderProps) {
  // useState with initializer — creates the store once on first render.
  // Avoids useRef .current access during render (react-hooks/refs).
  const [store] = useState(() => createEditorStore(initialDefinition));

  return (
    <EditorContext.Provider value={store}>
      {children}
    </EditorContext.Provider>
  );
}

export { EditorProvider };
