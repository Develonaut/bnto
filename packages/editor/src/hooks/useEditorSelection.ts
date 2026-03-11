/**
 * useEditorSelection — bridge from ReactFlow selection → editor store.
 *
 * Registers a single `useOnSelectionChange` listener that syncs the
 * selected node ID into the Zustand store. Returns `selectedNodeId`
 * from the store so callers get a stable reference.
 *
 * IMPORTANT: Call this hook exactly ONCE per editor instance (inside
 * useEditorCanvas). All other consumers should read `selectedNodeId`
 * via `editor.nodes.useNodes()` or `useEditorStoreApi()`.
 *
 * Must be used inside a ReactFlowProvider + EditorRoot.
 */

"use client";

import { useCallback } from "react";
import { useOnSelectionChange } from "@xyflow/react";
import type { Node } from "@xyflow/react";
import { useStore } from "zustand";
import { useEditorStoreApi } from "../context";

function useEditorSelection(): { selectedNodeId: string | null } {
  const storeApi = useEditorStoreApi();
  const selectedNodeId = useStore(storeApi, (s) => s.selectedNodeId);
  const setSelectedNodeId = useStore(storeApi, (s) => s.setSelectedNodeId);

  const onChange = useCallback(
    ({ nodes }: { nodes: Node[] }) => {
      setSelectedNodeId(nodes.length === 1 ? nodes[0]!.id : null);
    },
    [setSelectedNodeId],
  );

  useOnSelectionChange({ onChange });

  return { selectedNodeId };
}

export { useEditorSelection };
