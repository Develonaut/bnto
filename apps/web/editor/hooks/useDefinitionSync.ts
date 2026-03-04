/**
 * Incremental Definition → ReactFlow sync.
 *
 * Diffs definition.nodes against the previous render and only
 * adds/removes the changed RF nodes — avoids full re-conversion.
 * Does NOT handle full replacement (loadRecipe, undo); callers
 * must run definitionToBento + setNodes for those cases.
 *
 * Checks against current RF nodes (not just previous IDs) to
 * avoid duplicates when the palette adds a node to RF directly.
 *
 * Must be inside both EditorProvider and ReactFlowProvider.
 */

"use client";

import { useEffect, useRef } from "react";
import { useReactFlow } from "@xyflow/react";
import { useEditorStore } from "./useEditorStore";
import { definitionNodeToRfNode } from "../adapters/definitionNodeToRfNode";
import type { BentoNode } from "../adapters/types";

function useDefinitionSync(): void {
  const nodes = useEditorStore((s) => s.definition.nodes ?? []);
  const { setNodes, deleteElements, getNodes } = useReactFlow<BentoNode>();
  const prevIdsRef = useRef<Set<string>>(new Set());

  useEffect(() => {
    const currentIds = new Set(nodes.map((n) => n.id));
    const prevIds = prevIdsRef.current;

    /* Guard against duplicate IDs if another consumer has already
     * added the node to RF. */
    const rfIds = new Set(getNodes().map((n) => n.id));

    const added = nodes.filter((n) => !prevIds.has(n.id) && !rfIds.has(n.id));
    const removed = [...prevIds].filter((id) => !currentIds.has(id));

    if (added.length > 0) {
      const newRfNodes: BentoNode[] = [];
      for (const node of added) {
        const index = nodes.findIndex((n) => n.id === node.id);
        const rfNode = definitionNodeToRfNode(node, index);
        if (rfNode) newRfNodes.push(rfNode);
      }
      if (newRfNodes.length > 0) {
        setNodes((prev) => [...prev, ...newRfNodes]);
      }
    }

    if (removed.length > 0) {
      deleteElements({ nodes: removed.map((id) => ({ id })) });
    }

    prevIdsRef.current = currentIds;
  }, [nodes, setNodes, deleteElements, getNodes]);
}

export { useDefinitionSync };
