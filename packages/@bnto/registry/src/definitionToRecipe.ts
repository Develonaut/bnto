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

const FALLBACK_ACCEPT: AcceptSpec = {
  mimeTypes: [],
  extensions: [],
  label: "Any files",
};

/** Resolves name, slug, and id from metadata or definition defaults. */
function resolveIdentity(definition: Definition, metadata?: RecipeMetadata) {
  const name = metadata?.name ?? definition.name;
  const slug = metadata?.slug ?? toSlug(name);
  const id = metadata?.id ?? crypto.randomUUID();
  return { id, slug, name };
}

/** Resolves description, category, accept, and features from metadata or defaults. */
function resolveContent(definition: Definition, name: string, metadata?: RecipeMetadata) {
  const description = metadata?.description ?? `Custom recipe: ${name}`;
  const category = metadata?.category ?? "custom";
  const accept = metadata?.accept ?? deriveAcceptSpec(definition) ?? FALLBACK_ACCEPT;
  const features = metadata?.features ?? [];
  return { description, category, accept, features };
}

/**
 * Wraps a Definition into a Recipe with metadata.
 *
 * If metadata fields are omitted, sensible defaults are derived
 * from the definition's name and type.
 */
export function definitionToRecipe(definition: Definition, metadata?: RecipeMetadata): Recipe {
  const { id, slug, name } = resolveIdentity(definition, metadata);
  const { description, category, accept, features } = resolveContent(definition, name, metadata);
  return { id, slug, name, description, category, accept, features, definition };
}
