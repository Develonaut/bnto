/**
 * Core editor store hook — provides selector access to the editor store.
 *
 * The editor store must be provided via React context (EditorProvider).
 * This hook selects a slice from the store — re-renders only when
 * the selected slice changes.
 */

"use client";

import { useContext } from "react";
import { useStore } from "zustand";
import { EditorContext } from "../context";
import type { EditorStore } from "../store/createEditorStore";

/**
 * Select a slice from the editor store.
 *
 * Must be used inside an EditorProvider. Throws if no provider found.
 */
function useEditorStore<T>(selector: (state: EditorStore) => T): T {
  const store = useContext(EditorContext);
  if (!store) {
    throw new Error("useEditorStore must be used inside <EditorProvider>");
  }
  return useStore(store, selector);
}

export { useEditorStore };
