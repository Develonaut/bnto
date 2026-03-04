"use client";

import { useCallback, useEffect, useRef } from "react";
import { useReactFlow } from "@xyflow/react";
import { usePrevious } from "@/components/ui/usePrevious";
import type { CompartmentNodeType } from "../canvas/CompartmentNode";

/**
 * useAutoSelect — selection management for the editor sidebar.
 *
 * Owns three concerns:
 *   1. Programmatic node selection via setNodes
 *   2. Auto-select when nodes are added (select new) or removed (select previous)
 *   3. Center-on-select via fitView when selectedNodeId changes
 */

interface UseAutoSelectOptions {
  nodes: CompartmentNodeType[];
  selectedNodeId: string | null;
}

function useAutoSelect({ nodes, selectedNodeId }: UseAutoSelectOptions) {
  const { setNodes, fitView } = useReactFlow<CompartmentNodeType>();

  /* Select a node programmatically (sidebar click, add/remove effect). */
  const handleSelectNode = useCallback(
    (nodeId: string) => {
      setNodes((prev) =>
        prev.map((n) => ({ ...n, selected: n.id === nodeId })),
      );
    },
    [setNodes],
  );

  /* Center the selected node whenever selection changes — works for
   * canvas clicks, sidebar clicks, and auto-select on add/remove. */
  useEffect(() => {
    if (!selectedNodeId) return;
    fitView({ nodes: [{ id: selectedNodeId }], duration: 400, padding: 0.5 });
  }, [selectedNodeId, fitView]);

  /* Auto-select when nodes are added or removed. */
  const prevCount = usePrevious(nodes.length);
  const prevNodesRef = useRef(nodes);
  useEffect(() => {
    if (prevCount === undefined) return;

    /* Node added — select the new (last) node. */
    if (nodes.length > prevCount && nodes.length > 0) {
      const newNode = nodes[nodes.length - 1]!;
      handleSelectNode(newNode.id);
    }

    /* Node removed — select the previous node in the list. */
    if (nodes.length < prevCount && nodes.length > 0) {
      const prevNodes = prevNodesRef.current;
      const removedIndex = prevNodes.findIndex(
        (pn) => !nodes.some((n) => n.id === pn.id),
      );
      const selectIndex = Math.min(
        removedIndex > 0 ? removedIndex - 1 : 0,
        nodes.length - 1,
      );
      handleSelectNode(nodes[selectIndex]!.id);
    }

    prevNodesRef.current = nodes;
  }, [nodes, prevCount, handleSelectNode]);

  return { handleSelectNode };
}

export { useAutoSelect };
