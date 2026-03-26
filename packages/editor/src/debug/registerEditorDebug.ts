/**
 * registerEditorDebug — attaches the debug API to globalThis.__bnto__.editor.
 *
 * Accepts a storeApi (the Zustand store React is subscribed to) and
 * domain clients. Must be called from a React effect — NOT a constructor —
 * to avoid StrictMode double-invocation creating a stale store reference.
 *
 * Returns a cleanup function that removes the property.
 */

import type { StoreApi } from "zustand";
import type {
  NodeClient,
  DefinitionClient,
  ExecutionClient,
  HistoryClient,
  PanelClient,
} from "../editorTypes";
import type { EditorStore, EditorState } from "../store/types";
import { buildDebugApi } from "./buildDebugApi";

/** What registerEditorDebug needs — satisfied by both EditorInstance and ReactEditorInstance. */
interface DebugTarget {
  nodes: NodeClient;
  definition: DefinitionClient;
  execution: ExecutionClient;
  history: HistoryClient;
  panels: PanelClient;
  getState(): EditorState;
  subscribe(listener: (state: EditorState) => void): () => void;
}

function registerEditorDebug(storeApi: StoreApi<EditorStore>, target: DebugTarget): () => void {
  const api = buildDebugApi(storeApi, target);

  const g = globalThis as Record<string, unknown>;
  if (!g.__bnto__) g.__bnto__ = {};
  (g.__bnto__ as Record<string, unknown>).editor = api;

  return () => {
    const root = g.__bnto__ as Record<string, unknown> | undefined;
    if (root) delete root.editor;
  };
}

export { registerEditorDebug };
export type { DebugTarget };
