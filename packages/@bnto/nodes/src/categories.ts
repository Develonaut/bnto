/**
 * Node categories — grouping for UI display and documentation.
 */

import type { NodeCategory } from "./nodeTypes";

/** Metadata for a node category. */
export interface CategoryInfo {
  /** Category identifier. */
  name: NodeCategory;

  /** Human-readable display label. */
  label: string;

  /** Short description. */
  description: string;
}

/**
 * All node categories with display metadata.
 *
 * Order determines display order in UI category filters.
 */
export const CATEGORIES: readonly CategoryInfo[] = [
  {
    name: "image",
    label: "Image",
    description: "Image processing — resize, convert, compress, composite.",
  },
  {
    name: "spreadsheet",
    label: "Spreadsheet",
    description: "CSV and Excel operations — read, write, transform.",
  },
  {
    name: "file",
    label: "File",
    description: "File system operations — read, write, copy, move, list.",
  },
  {
    name: "data",
    label: "Data",
    description: "Data manipulation — edit fields, transform expressions.",
  },
  {
    name: "network",
    label: "Network",
    description: "HTTP requests and API integrations.",
  },
  {
    name: "control",
    label: "Control Flow",
    description: "Orchestration — groups, loops, parallel execution.",
  },
  {
    name: "system",
    label: "System",
    description: "Shell commands and system-level operations.",
  },
] as const;

/** Returns category info by name, or undefined if not found. */
export function getCategoryInfo(
  name: string,
): CategoryInfo | undefined {
  return CATEGORIES.find((c) => c.name === name);
}
