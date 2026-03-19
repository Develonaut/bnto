// ---------------------------------------------------------------------------
// Recipe types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

import type { Recipe } from "@bnto/nodes";

// Re-export the base Recipe type from @bnto/nodes
export type { Recipe } from "@bnto/nodes";

/**
 * A user's recipe — Recipe + persistence state.
 *
 * Extends the base Recipe (display metadata only) with fields that track
 * where and when the recipe was saved and synced. This type lives in
 * @bnto/core because persistence is a core concern, not a node concern.
 *
 * The layered type chain: Definition (@bnto/nodes) → Recipe (@bnto/nodes)
 * → UserRecipe (@bnto/core).
 */
export interface UserRecipe extends Recipe {
  /** Convex document _id. Null if not synced to cloud. */
  cloudId: string | null;

  /** Timestamp when last saved locally. Null if never saved. */
  savedAt: number | null;

  /** Timestamp when last synced to cloud. Null if never synced. */
  syncedAt: number | null;
}

/** Projected recipe for list views. */
export interface RecipeListItem {
  id: string;
  name: string;
  nodeCount: number;
  /** Human-readable labels for the distinct processing node types (excludes I/O and containers). */
  nodeTypes: string[];
  updatedAt: number;
  /** When last synced to Convex. null = never synced, undefined = cloud recipe (always synced). */
  syncedAt?: number | null;
}
