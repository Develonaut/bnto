import type { StoredRecipe } from "@bnto/core";
import type { Draft } from "./draftTypes";

/** Convert an editor Draft to a StoredRecipe for the recipes store. */
export function draftToStoredRecipe(draft: Draft): StoredRecipe {
  return {
    definition: draft.definition,
    metadata: {
      id: draft.metadata.id,
      name: draft.metadata.name,
      cloudId: draft.metadata.cloudId ?? undefined,
      type: draft.metadata.type,
      version: draft.metadata.version,
    },
    savedAt: draft.savedAt,
    syncedAt: draft.syncedAt,
  };
}
