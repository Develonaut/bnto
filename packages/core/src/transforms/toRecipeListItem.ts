/** Transform a raw Convex recipe list projection into the RecipeListItem shape. */
import type { RawRecipeListProjection } from "../types/raw";
import type { RecipeListItem } from "../types";

export function toRecipeListItem(doc: RawRecipeListProjection): RecipeListItem {
  return {
    id: String(doc._id),
    name: doc.name,
    nodeCount: doc.nodeCount,
    nodeTypes: doc.nodeTypes ?? [],
    updatedAt: doc.updatedAt,
  };
}
