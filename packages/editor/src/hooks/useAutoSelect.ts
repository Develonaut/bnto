"use client";

import { useCallback, useEffect } from "react";
import { useReactFlow } from "@xyflow/react";
import { useEditorActions } from "./useEditorActions";
import { FIT_VIEW_OPTIONS } from "../constants";

/**
 * useAutoSelect — selection management for the editor.
 *
 * Two concerns:
 *   1. Programmatic node selection via the store's selectNode action
 *   2. Center-on-select via fitView (genuine DOM side effect)
 *
 * Auto-select on add/remove is handled in the useAddNode and
 * useRemoveNode action hooks — no effects needed here.
 */

interface UseAutoSelectOptions {
  selectedNodeId: string | null;
}

function useAutoSelect({ selectedNodeId }: UseAutoSelectOptions) {
  const { fitView } = useReactFlow();
  const { selectNode } = useEditorActions();

  /* Select a node programmatically (sidebar click). Updates the store
   * directly — no RF roundtrip via setNodes. */
  const handleSelectNode = useCallback(
    (nodeId: string) => {
      selectNode(nodeId);
    },
    [selectNode],
  );

  /* Center the selected node whenever selection changes. This is a
   * genuine DOM side effect — fitView requires measured node dimensions. */
  useEffect(() => {
    if (!selectedNodeId) return;
    fitView({ ...FIT_VIEW_OPTIONS, nodes: [{ id: selectedNodeId }], duration: 400 });
  }, [selectedNodeId, fitView]);

  return { handleSelectNode };
}

export { useAutoSelect };
