/**
 * useAddDividerNodes — injects divider nodes between positioned nodes.
 *
 * Reads expandedContainerIds from the store and injects non-interactive
 * addDivider nodes in the gaps between consecutive nodes.
 *
 * Returns the original nodes unchanged when no containers are expanded
 * and no top-level gaps need dividers (referentially stable).
 */

"use client";

import { useMemo } from "react";
import { useStore } from "zustand";
import { useEditorStoreApi } from "../context";
import type { BentoNode } from "../adapters/types";
import { injectAddDividers } from "../helpers/injectAddDividers";

function useAddDividerNodes(nodes: BentoNode[]): BentoNode[] {
  const storeApi = useEditorStoreApi();
  const expandedIds = useStore(storeApi, (s) => s.expandedContainerIds);

  return useMemo(
    () => injectAddDividers(nodes, expandedIds),
    [nodes, expandedIds],
  );
}

export { useAddDividerNodes };
