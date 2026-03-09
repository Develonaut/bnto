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
  const nodeProgress = useEditorStore((s) => s.nodeProgress);

  return useMemo(() => {
    const hasState = Object.keys(executionState).length > 0;
    const hasProgress = Object.keys(nodeProgress).length > 0;
    if (!hasState && !hasProgress) return nodes;

    return nodes.map((node) => {
      const status = executionState[node.id] as CompartmentNodeData["status"] | undefined;
      const progress = nodeProgress[node.id];
      const statusChanged = status && status !== node.data.status;
      const progressChanged = progress !== undefined && progress !== node.data.progress;

      if (!statusChanged && !progressChanged) return node;

      return {
        ...node,
        data: {
          ...node.data,
          ...(statusChanged ? { status } : {}),
          ...(progressChanged ? { progress } : {}),
        },
      };
    });
  }, [nodes, executionState, nodeProgress]);
}

export { useExecutionNodes };
