"use client";

import { useMemo } from "react";
import { useEditorStore } from "./useEditorStore";
import type { BentoNode } from "../adapters/types";

/**
 * useExecutionNodes — syncs executionState into node.data.status.
 *
 * The store tracks execution status per node in `executionState`
 * (a flat Record<nodeId, NodeExecutionStatus>). RF nodes carry their
 * status in `data.status`. This hook bridges the two: it reads both
 * and returns nodes with `data.status` reflecting the current
 * execution phase.
 *
 * Called once in CanvasShell to derive display-ready nodes.
 */

function useExecutionNodes(nodes: BentoNode[]): BentoNode[] {
  const executionState = useEditorStore((s) => s.executionState);

  return useMemo(() => {
    const hasExecution = Object.keys(executionState).length > 0;
    if (!hasExecution) return nodes;

    return nodes.map((node) => {
      const status = executionState[node.id];
      if (!status || status === node.data.status) return node;
      return { ...node, data: { ...node.data, status } };
    });
  }, [nodes, executionState]);
}

export { useExecutionNodes };
