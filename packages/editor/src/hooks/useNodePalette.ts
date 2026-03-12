/**
 * Hook for the node palette — available node types grouped by category.
 *
 * Multi-operation node types (image, spreadsheet, file-system) are split
 * into one palette item per processor so each operation is pre-set when
 * added — valid from the start. Non-processor types pass through as-is.
 */

"use client";

import { useMemo } from "react";
import type { NodeCategory, NodeTypeName, CategoryInfo } from "@bnto/nodes";
import { NODE_TYPE_INFO, NODE_TYPE_NAMES, CATEGORIES, PROCESSORS } from "@bnto/nodes";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface PaletteItem {
  /** Node type name (e.g. "image"). */
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
  /** Pre-set parameters to merge on creation (e.g. { operation: "compress" }). */
  defaultParams?: Record<string, unknown>;
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
// Processor lookup — keyed by node type for O(1) grouping
// ---------------------------------------------------------------------------

const processorsByType = new Map<string, typeof PROCESSORS[number][]>();
for (const proc of PROCESSORS) {
  const existing = processorsByType.get(proc.nodeType) ?? [];
  existing.push(proc);
  processorsByType.set(proc.nodeType, existing);
}

// ---------------------------------------------------------------------------
// Pure computation (operates on module-level constants)
// ---------------------------------------------------------------------------

function nodeTypeToItem(info: typeof NODE_TYPE_INFO[NodeTypeName]): PaletteItem {
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

function computePalette(browserOnly: boolean): NodePaletteResult {
  // I/O nodes are structural (always present) — never shown in the palette.
  const nonIoTypes = NODE_TYPE_NAMES.map((name) => NODE_TYPE_INFO[name]).filter(
    (t) => t.category !== "io",
  );

  const allItems: PaletteItem[] = [];
  for (const info of nonIoTypes) {
    const procs = processorsByType.get(info.name);
    if (procs && procs.length > 0) {
      // Split: one palette item per processor operation
      for (const proc of procs) {
        allItems.push({
          type: info.name,
          label: proc.name,
          description: proc.description,
          category: info.category,
          icon: info.icon,
          browserCapable: info.browserCapable,
          isContainer: info.isContainer,
          defaultParams: { operation: proc.operation },
        });
      }
    } else {
      // No processors — pass through as-is
      allItems.push(nodeTypeToItem(info));
    }
  }

  const browserItems = allItems.filter((t) => t.browserCapable);
  const displayItems = browserOnly ? browserItems : allItems;

  // Group by category in CATEGORIES display order
  const categoryMap = new Map<NodeCategory, PaletteItem[]>();
  for (const item of displayItems) {
    const existing = categoryMap.get(item.category) ?? [];
    existing.push(item);
    categoryMap.set(item.category, existing);
  }

  const groups: PaletteGroup[] = CATEGORIES.filter((cat) => categoryMap.has(cat.name)).map(
    (cat) => ({
      category: cat,
      items: categoryMap.get(cat.name)!,
    }),
  );

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
  return useMemo(() => computePalette(browserOnly), [browserOnly]);
}

export { useNodePalette, computePalette };
export type { PaletteItem, PaletteGroup, NodePaletteResult };
