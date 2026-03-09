/**
 * Core editor store hook — provides selector access to the editor store.
 *
 * Uses the module-level store instance directly (no React context).
 * Re-renders only when the selected slice changes.
 */

"use client";

import { useStore } from "zustand";
import { getEditorStore } from "../store/instance";
import type { EditorStore } from "../store/types";

/**
 * Select a slice from the editor store.
 *
 * Must be used after the store has been initialized (inside <EditorRoot>).
 */
function useEditorStore<T>(selector: (state: EditorStore) => T): T {
  return useStore(getEditorStore(), selector);
}

export { useEditorStore };
