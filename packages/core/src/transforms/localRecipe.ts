import type { RecipeListItem } from "../types";

/**
 * Minimal shape of a persisted editor draft as stored in localStorage.
 * Defined here to avoid a circular dependency on @bnto/editor.
 */
export interface LocalDraft {
  definition: { nodes?: unknown[] };
  metadata: { id: string; name: string };
  savedAt: number;
}

/** Convert a localStorage draft to the same RecipeListItem shape as cloud recipes. */
export function draftToRecipeListItem(draft: LocalDraft): RecipeListItem {
  return {
    id: draft.metadata.id,
    name: draft.metadata.name,
    nodeCount: draft.definition.nodes?.length ?? 0,
    updatedAt: draft.savedAt,
  };
}
