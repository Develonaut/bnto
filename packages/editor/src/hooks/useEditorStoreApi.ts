/**
 * Raw store API access — for imperative reads in callbacks.
 *
 * Returns the Zustand StoreApi without subscribing to changes.
 * Use this in event handlers and callbacks where you need the
 * latest state snapshot without causing re-renders.
 *
 * Must be used inside an EditorProvider.
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
