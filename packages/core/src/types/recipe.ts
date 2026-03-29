// ---------------------------------------------------------------------------
// Recipe types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

// Re-export the base Recipe type from @bnto/registry
export type { Recipe } from "@bnto/registry";

/** Projected recipe for list views. */
export interface RecipeListItem {
  id: string;
  name: string;
  nodeCount: number;
  /** Human-readable labels for the distinct processing node types (excludes I/O and containers). */
  nodeTypes: string[];
  updatedAt: number;
}
