"use client";

import { useCallback, useEffect, useRef } from "react";
import { useReactFlow, useStore } from "@xyflow/react";
import type { CompartmentNodeType } from "../canvas/CompartmentNode";

/**
 * useAutoSelect — selection management for the editor sidebar.
 *
 * Owns three concerns:
 *   1. Programmatic node selection via setNodes
 *   2. Auto-select when nodes are added (select new) or removed (select previous)
 *   3. Center-on-select via fitView when selectedNodeId changes
 *
 * Subscribes to node COUNT (primitive) instead of the full nodes array
 * to avoid infinite loops — setNodes for selection changes the nodes
 * ref but not the count, so the add/remove effect stays quiet.
 */

interface UseAutoSelectOptions {
  selectedNodeId: string | null;
}

function useAutoSelect({ selectedNodeId }: UseAutoSelectOptions) {
  const { setNodes, fitView, getNodes } = useReactFlow<CompartmentNodeType>();
  const nodeCount = useStore((s) => s.nodes.length);

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

  /* Auto-select when nodes are added or removed.
   *
   * Uses nodeCount (primitive) as trigger — avoids re-firing when
   * setNodes changes selection without changing the count.
   * Reads actual nodes via getNodes() inside the effect for fresh data.
   * prevCountRef is read only in effects (safe from react-hooks/refs). */
  const prevCountRef = useRef(nodeCount);
  const prevNodesRef = useRef<CompartmentNodeType[]>([]);

  useEffect(() => {
    const prevCount = prevCountRef.current;
    prevCountRef.current = nodeCount;
    const currentNodes = getNodes();

    /* Node added — select the new (last) node. */
    if (nodeCount > prevCount && nodeCount > 0) {
      const newNode = currentNodes[currentNodes.length - 1]!;
      handleSelectNode(newNode.id);
    }

    /* Node removed — select the previous node in the list. */
    if (nodeCount < prevCount && nodeCount > 0) {
      const prevNodes = prevNodesRef.current;
      const removedIndex = prevNodes.findIndex(
        (pn) => !currentNodes.some((n) => n.id === pn.id),
      );
      const selectIndex = Math.min(
        removedIndex > 0 ? removedIndex - 1 : 0,
        nodeCount - 1,
      );
      handleSelectNode(currentNodes[selectIndex]!.id);
    }

    prevNodesRef.current = currentNodes;
  }, [nodeCount, handleSelectNode, getNodes]);

  return { handleSelectNode };
}

export { useAutoSelect };
