"use client";

/**
 * usePipelineEdges — derives visual pipeline edges from positioned nodes.
 *
 * Reads execution state reactively from the store so edges update
 * their progress fill as the engine emits pipeline events.
 */

import { useMemo } from "react";
import { useStore } from "zustand";
import type { Edge } from "@xyflow/react";
import { useEditorStoreApi } from "../context";
import { generatePipelineEdges } from "../adapters/generatePipelineEdges";
import type { BentoNode } from "../adapters/types";

function usePipelineEdges(nodes: BentoNode[]): Edge[] {
  const storeApi = useEditorStoreApi();
  const executionState = useStore(storeApi, (s) => s.executionState);
  const nodeProgress = useStore(storeApi, (s) => s.nodeProgress);

  return useMemo(
    () => generatePipelineEdges(nodes, executionState, nodeProgress),
    [nodes, executionState, nodeProgress],
  );
}

export { usePipelineEdges };
