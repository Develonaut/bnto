/** Transform a raw Convex recipe doc into the CloudRecipeDetail shape. */
import type { RawRecipeDoc } from "../types/raw";
import type { CloudRecipeDetail } from "./CloudRecipeDetail";

export function toRecipe(doc: RawRecipeDoc): CloudRecipeDetail {
  return {
    id: String(doc._id),
    userId: String(doc.userId),
    name: doc.name,
    definition: doc.definition,
    version: doc.version,
    formatVersion: doc.formatVersion,
    isPublic: doc.isPublic,
    createdAt: doc.createdAt,
    updatedAt: doc.updatedAt,
  };
}
