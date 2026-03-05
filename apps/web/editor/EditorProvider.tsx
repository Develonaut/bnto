/**
 * EditorProvider — creates and provides the editor store to the React tree.
 *
 * Factory pattern: each EditorProvider creates its own store instance.
 * Multiple editors on the same page get independent stores.
 *
 * Pass `slug` to pre-populate with a predefined recipe, or omit for blank.
 * The store is created with the recipe data already loaded — no render-cycle
 * init hacks needed.
 */

"use client";

import { useState, type ReactNode } from "react";
import { EditorContext } from "./context";
import { createEditorStore } from "./store/createEditorStore";

interface EditorProviderProps {
  /** Predefined recipe slug. Omit for blank canvas. */
  slug?: string;
  children: ReactNode;
}

function EditorProvider({ slug, children }: EditorProviderProps) {
  const [store] = useState(() => createEditorStore(slug));

  return (
    <EditorContext.Provider value={store}>
      {children}
    </EditorContext.Provider>
  );
}

export { EditorProvider };
