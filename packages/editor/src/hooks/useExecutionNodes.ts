"use client";

/**
 * useExecutionNodes — injects executionState status onto node.data.status.
 *
 * The store's executionState maps nodeId → NodeExecutionStatus. This hook
 * reads that map and the current nodes array, returning a new nodes array
 * where each node's data.status reflects the execution state. This drives
 * visual changes in CompartmentNode and IoNode (muted, failed ring, etc.).
 *
 * Returns the original nodes unchanged when executionState is empty (idle).
 */

import { useMemo } from "react";
import { useEditorStore } from "./useEditorStore";
import type { BentoNode, CompartmentNodeData } from "../adapters/types";

function useExecutionNodes(nodes: BentoNode[]): BentoNode[] {
  const executionState = useEditorStore((s) => s.executionState);

  return useMemo(() => {
    const keys = Object.keys(executionState);
    if (keys.length === 0) return nodes;

    return nodes.map((node) => {
      const status = executionState[node.id] as CompartmentNodeData["status"] | undefined;
      if (!status || status === node.data.status) return node;

      return {
        ...node,
        data: { ...node.data, status },
      };
    });
  }, [nodes, executionState]);
}

export { useExecutionNodes };
