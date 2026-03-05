/**
 * Shared test helpers for editor store tests.
 *
 * Used by createEditorStore.test.ts, createEditorStore.actions.test.ts,
 * and createEditorStore.history.test.ts.
 */

import type { StoreApi } from "zustand";
import type { EditorStore } from "./types";
import { addNode } from "../actions/addNode";
import { removeNode } from "../actions/removeNode";
import { updateParams } from "../actions/updateParams";

/** Read the current store state snapshot. */
export function state(store: StoreApi<EditorStore>) {
  return store.getState();
}

/** Call the pure addNode action and apply to store. */
export function addNodeViaStore(store: StoreApi<EditorStore>, type: string): string | null {
  const result = addNode(store.getState(), type as never);
  if (!result) return null;
  store.setState(result.nextState);
  return result.nodeId;
}

/** Call the pure removeNode action and apply to store. */
export function removeNodeViaStore(store: StoreApi<EditorStore>, id: string): void {
  const nextState = removeNode(store.getState(), id);
  if (!nextState) return;
  store.setState(nextState);
}

/** Call the pure updateParams action and apply to store. */
export function updateParamsViaStore(
  store: StoreApi<EditorStore>,
  nodeId: string,
  params: Record<string, unknown>,
): void {
  const nextState = updateParams(store.getState(), nodeId, params);
  if (!nextState) return;
  store.setState(nextState);
}
