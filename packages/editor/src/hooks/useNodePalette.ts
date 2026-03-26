/**
 * Hook for the node palette — available node types grouped by category.
 *
 * Each node type maps directly to one palette item. The type IS the
 * operation (e.g. "image-compress", not "image" with operation param).
 */

"use client";

import { useMemo } from "react";
import type { NodeCategory, NodeTypeName, NodeTypeInfo, CategoryInfo } from "@bnto/core";
import { core } from "@bnto/core";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface PaletteItem {
  /** Node type name (e.g. "image-compress"). */
  type: NodeTypeName;
  /** Human-readable label (e.g. "Compress Images"). */
  label: string;
  /** One-sentence description. */
  description: string;
  /** Category for grouping. */
  category: NodeCategory;
  /** Lucide icon name. */
  icon: string;
  /** Whether this node is available in the browser. */
  browserCapable: boolean;
  /** Whether this node is a container (group, loop, parallel). */
  isContainer: boolean;
}

interface PaletteGroup {
  /** Category metadata. */
  category: CategoryInfo;
  /** Palette items in this category. */
  items: PaletteItem[];
}

interface NodePaletteResult {
  /** All palette items grouped by category, in display order. */
  groups: PaletteGroup[];
  /** Flat list of all palette items. */
  allItems: PaletteItem[];
  /** Only browser-capable palette items. */
  browserItems: PaletteItem[];
}

// ---------------------------------------------------------------------------
// Pure computation
// ---------------------------------------------------------------------------

function toItem(info: NodeTypeInfo): PaletteItem {
  return {
    type: info.name,
    label: info.label,
    description: info.description,
    category: info.category,
    icon: info.icon,
    browserCapable: info.browserCapable,
    isContainer: info.isContainer,
  };
}

function groupByCategory(
  items: PaletteItem[],
  categories: readonly CategoryInfo[],
): PaletteGroup[] {
  const categoryMap = new Map<NodeCategory, PaletteItem[]>();
  for (const item of items) {
    const existing = categoryMap.get(item.category) ?? [];
    existing.push(item);
    categoryMap.set(item.category, existing);
  }
  return categories
    .filter((cat) => categoryMap.has(cat.name))
    .map((cat) => ({ category: cat, items: categoryMap.get(cat.name) ?? [] }));
}

function computePalette(
  nodeTypes: Record<NodeTypeName, NodeTypeInfo>,
  categories: readonly CategoryInfo[],
  browserOnly: boolean,
): NodePaletteResult {
  const allItems = Object.values(nodeTypes)
    .filter((t) => t.category !== "io")
    .map(toItem);

  const browserItems = allItems.filter((t) => t.browserCapable);
  const displayItems = browserOnly ? browserItems : allItems;
  const groups = groupByCategory(displayItems, categories);

  return { groups, allItems, browserItems };
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

/**
 * Returns available palette items for the node palette.
 *
 * @param browserOnly - When true, only include browser-capable items.
 *                      Default: false (show all, flag server-only).
 */
function useNodePalette(browserOnly = false): NodePaletteResult {
  const nodeTypes = core.registry.useNodeTypes();
  const categories = core.registry.useCategories();

  return useMemo(
    () => computePalette(nodeTypes, categories, browserOnly),
    [nodeTypes, categories, browserOnly],
  );
}

export { useNodePalette, computePalette };
export type { PaletteItem, PaletteGroup, NodePaletteResult };
