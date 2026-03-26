/** Transform a raw Convex recipe doc into a UserRecipe for the local store. */

import { definitionToRecipe } from "@bnto/registry";
import type { RawRecipeDoc } from "../types/raw";
import type { UserRecipe } from "../types/recipe";

export function cloudRecipeToUserRecipe(doc: RawRecipeDoc): UserRecipe {
  const recipe = definitionToRecipe(doc.definition, {
    id: crypto.randomUUID(),
    name: doc.name,
  });
  return {
    ...recipe,
    cloudId: String(doc._id),
    savedAt: doc.updatedAt,
    syncedAt: Date.now(),
  };
}
