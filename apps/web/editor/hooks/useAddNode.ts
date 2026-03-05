/**
 * useAddNode — creates a new compartment node in the editor.
 *
 * Business logic hook (layer 3 in the editor architecture):
 *   1. Creates the RF node + config via createCompartmentNode
 *   2. Captures undo snapshot
 *   3. Auto-selects the new node (deselects all others)
 *   4. Revalidates the definition
 *
 * Uses storeApi for atomic state updates — all fields set in one call.
 */

"use client";

import { useCallback } from "react";
import type { NodeTypeName } from "@bnto/nodes";
import { useEditorStoreApi } from "./useEditorStoreApi";
import { createCompartmentNode } from "../adapters/createCompartmentNode";
import { captureSnapshot, pushToStack, revalidateState } from "../store/helpers";

function useAddNode() {
  const storeApi = useEditorStoreApi();

  return useCallback(
    (type: NodeTypeName, position?: { x: number; y: number }): string | null => {
      const state = storeApi.getState();
      const slotIndex = state.nodes.length;
      const result = createCompartmentNode(type, slotIndex, position);
      if (!result) return null;

      const snapshot = captureSnapshot(state.nodes, state.configs);

      // Auto-select the new node, deselect all others
      const deselected = state.nodes.map((n) =>
        n.selected ? { ...n, selected: false } : n,
      );
      const newNode = { ...result.node, selected: true };
      const nextNodes = [...deselected, newNode];
      const nextConfigs = { ...state.configs, [result.node.id]: result.config };

      storeApi.setState({
        nodes: nextNodes,
        configs: nextConfigs,
        isDirty: true,
        undoStack: pushToStack(state.undoStack, snapshot),
        redoStack: [],
        validationErrors: revalidateState(nextNodes, nextConfigs, state.recipeMetadata),
      });

      return result.node.id;
    },
    [storeApi],
  );
}

export { useAddNode };
