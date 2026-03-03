/**
 * Hook for the node palette — available node types grouped by category.
 *
 * Returns node types from NODE_TYPE_INFO, grouped by category, with
 * browser capability flags. Server-only nodes are included but flagged.
 */

"use client";

import { useMemo } from "react";
import type { NodeCategory, NodeTypeInfo, CategoryInfo } from "@bnto/nodes";
import { NODE_TYPE_INFO, NODE_TYPE_NAMES, CATEGORIES } from "@bnto/nodes";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface PaletteGroup {
  /** Category metadata. */
  category: CategoryInfo;
  /** Node types in this category. */
  items: NodeTypeInfo[];
}

interface NodePaletteResult {
  /** All node types grouped by category, in display order. */
  groups: PaletteGroup[];
  /** Flat list of all node types. */
  allTypes: NodeTypeInfo[];
  /** Only browser-capable node types. */
  browserTypes: NodeTypeInfo[];
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

/**
 * Returns available node types for the palette.
 *
 * @param browserOnly - When true, only include browser-capable types.
 *                      Default: false (show all, flag server-only).
 */
function useNodePalette(browserOnly = false): NodePaletteResult {
  return useMemo(() => {
    const allTypes = NODE_TYPE_NAMES.map((name) => NODE_TYPE_INFO[name]);
    const browserTypes = allTypes.filter((t) => t.browserCapable);
    const displayTypes = browserOnly ? browserTypes : allTypes;

    // Group by category in CATEGORIES display order
    const categoryMap = new Map<NodeCategory, NodeTypeInfo[]>();
    for (const nodeType of displayTypes) {
      const existing = categoryMap.get(nodeType.category) ?? [];
      existing.push(nodeType);
      categoryMap.set(nodeType.category, existing);
    }

    const groups: PaletteGroup[] = CATEGORIES
      .filter((cat) => categoryMap.has(cat.name))
      .map((cat) => ({
        category: cat,
        items: categoryMap.get(cat.name)!,
      }));

    return { groups, allTypes, browserTypes };
  }, [browserOnly]);
}

export { useNodePalette };
export type { PaletteGroup, NodePaletteResult };
