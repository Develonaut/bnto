/**
 * Wraps a Definition into a Recipe.
 *
 * Produces a Recipe (display metadata only — no persistence fields).
 * The caller can override any metadata field. Persistence (cloudId,
 * savedAt, syncedAt) is added by @bnto/core when creating a UserRecipe.
 */

import type { Definition } from "@bnto/nodes";
import type { Recipe, AcceptSpec } from "./recipe";
import { deriveAcceptSpec } from "./deriveAcceptSpec";

/** Metadata overrides for recipe creation. */
export interface RecipeMetadata {
  id?: string;
  slug?: string;
  name?: string;
  description?: string;
  category?: string;
  accept?: AcceptSpec;
  features?: string[];
}

/** Converts a name to a URL-safe slug (lowercase, hyphenated). */
function toSlug(name: string): string {
  const slug = name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
  return slug || "untitled";
}

/**
 * Wraps a Definition into a Recipe with metadata.
 *
 * If metadata fields are omitted, sensible defaults are derived
 * from the definition's name and type.
 */
export function definitionToRecipe(definition: Definition, metadata?: RecipeMetadata): Recipe {
  const name = metadata?.name ?? definition.name;
  const slug = metadata?.slug ?? toSlug(name);

  return {
    id: metadata?.id ?? crypto.randomUUID(),
    slug,
    name,
    description: metadata?.description ?? `Custom recipe: ${name}`,
    category: metadata?.category ?? "custom",
    accept: metadata?.accept ??
      deriveAcceptSpec(definition) ?? {
        mimeTypes: [],
        extensions: [],
        label: "Any files",
      },
    features: metadata?.features ?? [],
    definition,
  };
}
