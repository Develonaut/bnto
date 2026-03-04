/**
 * Raw store API access for the editor store.
 *
 * Returns the Zustand StoreApi so callbacks can read/write state
 * imperatively without creating React subscriptions.
 *
 * Use useEditorStore(selector) for reactive reads in render.
 * Use useEditorStoreApi() for imperative reads in callbacks.
 */

"use client";

import { useContext } from "react";
import type { StoreApi } from "zustand";
import { EditorContext } from "../context";
import type { EditorStore } from "../store/types";

function useEditorStoreApi(): StoreApi<EditorStore> {
  const store = useContext(EditorContext);
  if (!store) {
    throw new Error("useEditorStoreApi must be used inside <EditorProvider>");
  }
  return store;
}

export { useEditorStoreApi };
