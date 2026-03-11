/**
 * createUseNodes — hook factory for node state.
 *
 * Returns a useNodes hook closure that captures storeApi at creation time.
 * Subscribes to node-related state slices via useStore.
 */

"use client";

import type { StoreApi } from "zustand";
import { useStore } from "zustand";
import type { EditorStore } from "../../store/types";
import type { NodesHookResult } from "../../reactEditorTypes";

function createUseNodes(storeApi: StoreApi<EditorStore>) {
  return function useNodes(): NodesHookResult {
    const nodes = useStore(storeApi, (s) => s.nodes);
    const edges = useStore(storeApi, (s) => s.edges);
    const configs = useStore(storeApi, (s) => s.configs);
    const selectedNodeId = useStore(storeApi, (s) => s.selectedNodeId);
    const insertAfterNodeId = useStore(storeApi, (s) => s.insertAfterNodeId);
    const insertIntoContainerId = useStore(storeApi, (s) => s.insertIntoContainerId);

    return {
      nodes,
      edges,
      configs,
      selectedNodeId,
      insertAfterNodeId,
      insertIntoContainerId,
    };
  };
}

export { createUseNodes };
