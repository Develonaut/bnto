import type {
  RawRecipeDoc,
  RawRecipeListProjection,
} from "../types/raw";
import type { Recipe, RecipeListItem } from "../types";

export function toRecipe(doc: RawRecipeDoc): Recipe {
  return {
    id: String(doc._id),
    userId: String(doc.userId),
    name: doc.name,
    definition: doc.definition,
    version: doc.version,
    isPublic: doc.isPublic,
    createdAt: doc.createdAt,
    updatedAt: doc.updatedAt,
  };
}

export function toRecipeListItem(doc: RawRecipeListProjection): RecipeListItem {
  return {
    id: String(doc._id),
    name: doc.name,
    nodeCount: doc.nodeCount,
    updatedAt: doc.updatedAt,
  };
}
