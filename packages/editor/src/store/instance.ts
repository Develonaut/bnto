/**
 * Module-level editor store instance.
 *
 * Replaces React context distribution. Hooks access the store directly
 * via getEditorStore() instead of useContext(EditorContext).
 *
 * Trade-off: only one editor per page (no multi-editor support).
 * This is acceptable — bnto only has one editor at a time.
 */

import type { StoreApi } from "zustand";
import type { Definition } from "@bnto/nodes";
import type { EditorStore } from "./types";
import { createEditorStore } from "./createEditorStore";

let store: StoreApi<EditorStore> | null = null;

/** Initialize (or re-initialize) the editor store. Call on mount. */
function initEditorStore(definition?: Definition): StoreApi<EditorStore> {
  store = createEditorStore(definition);
  return store;
}

/** Get the current editor store instance. Throws if not initialized. */
function getEditorStore(): StoreApi<EditorStore> {
  if (!store) {
    throw new Error("Editor store not initialized. Wrap with <EditorRoot>.");
  }
  return store;
}

export { initEditorStore, getEditorStore };
