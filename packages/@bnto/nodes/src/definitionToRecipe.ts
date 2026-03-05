/**
 * Wraps a Definition into a Recipe for export.
 *
 * Produces a complete Recipe object with the definition, metadata,
 * a generated slug, and default accept/SEO specs. The caller can
 * override any metadata field.
 */

import type { Definition } from "./definition";
import type { Recipe, AcceptSpec, SEOSpec } from "./recipe";
import { deriveAcceptSpec } from "./deriveAcceptSpec";

/** Metadata overrides for recipe export. */
export interface RecipeMetadata {
  slug?: string;
  name?: string;
  description?: string;
  category?: string;
  accept?: AcceptSpec;
  features?: string[];
  seo?: SEOSpec;
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
export function definitionToRecipe(
  definition: Definition,
  metadata?: RecipeMetadata,
): Recipe {
  const name = metadata?.name ?? definition.name;
  const slug = metadata?.slug ?? toSlug(name);

  return {
    slug,
    name,
    description:
      metadata?.description ?? `Custom recipe: ${name}`,
    category: metadata?.category ?? "custom",
    accept: metadata?.accept ?? deriveAcceptSpec(definition) ?? {
      mimeTypes: [],
      extensions: [],
      label: "Any files",
    },
    features: metadata?.features ?? [],
    seo: metadata?.seo ?? {
      title: `${name} -- bnto`,
      h1: name,
    },
    definition,
  };
}
