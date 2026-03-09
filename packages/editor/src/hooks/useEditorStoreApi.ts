/**
 * Raw store API access — for imperative reads in callbacks.
 *
 * Returns the Zustand StoreApi without subscribing to changes.
 * Use this in event handlers and callbacks where you need the
 * latest state snapshot without causing re-renders.
 *
 * Must be used after the store has been initialized (inside <EditorRoot>).
 */

"use client";

import type { StoreApi } from "zustand";
import { getEditorStore } from "../store/instance";
import type { EditorStore } from "../store/types";

function useEditorStoreApi(): StoreApi<EditorStore> {
  return getEditorStore();
}

export { useEditorStoreApi };
