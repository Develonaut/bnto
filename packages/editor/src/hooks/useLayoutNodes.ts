/**
 * useLayoutNodes — repositions nodes based on container expansion state.
 *
 * Reads store.nodes and store.expandedContainerIds, passes them through
 * the layoutNodes algorithm, and returns positioned nodes.
 *
 * When no containers are expanded, returns the original nodes unchanged
 * (referentially stable — no unnecessary re-renders).
 */

"use client";

import { useMemo } from "react";
import { useEditorStore } from "./useEditorStore";
import type { BentoNode } from "../adapters/types";
import { layoutNodes } from "../adapters/layoutNodes";

function useLayoutNodes(nodes: BentoNode[]): BentoNode[] {
  const expandedIds = useEditorStore((s) => s.expandedContainerIds);

  return useMemo(
    () => layoutNodes(nodes, expandedIds),
    [nodes, expandedIds],
  );
}

export { useLayoutNodes };
