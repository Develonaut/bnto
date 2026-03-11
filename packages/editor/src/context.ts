/**
 * Editor context — provides the ReactEditorInstance and raw storeApi.
 *
 * Two access patterns:
 * - useEditor() — public. Components use domain hooks via this.
 * - useEditorStoreApi() — @internal. Rendering pipeline hooks that need
 *   raw store subscriptions (useLayoutNodes, useExecutionNodes, etc.).
 */

"use client";

import { createContext, useContext } from "react";
import type { StoreApi } from "zustand";
import type { EditorStore } from "./store/types";
import type { ReactEditorInstance } from "./reactEditorTypes";

interface EditorContextValue {
  instance: ReactEditorInstance;
  storeApi: StoreApi<EditorStore>;
}

const EditorContext = createContext<EditorContextValue | null>(null);

/** Access the ReactEditorInstance from context. Must be used within EditorProvider. */
function useEditor(): ReactEditorInstance {
  const ctx = useContext(EditorContext);
  if (!ctx) {
    throw new Error("useEditor must be used within EditorProvider");
  }
  return ctx.instance;
}

/**
 * @internal — access the raw Zustand storeApi for rendering pipeline hooks.
 *
 * Only for hooks that need direct store subscriptions (useLayoutNodes,
 * useExecutionNodes, useAddDividerNodes, useEditorSelection, useEditorNode,
 * useEditorCanvas). Components should use useEditor() instead.
 */
function useEditorStoreApi(): StoreApi<EditorStore> {
  const ctx = useContext(EditorContext);
  if (!ctx) {
    throw new Error("useEditorStoreApi must be used within EditorProvider");
  }
  return ctx.storeApi;
}

export { EditorContext, useEditor, useEditorStoreApi };
export type { EditorContextValue };
