/**
 * Recipe types — definition + display metadata.
 *
 * A Recipe is a Definition bundled with catalog metadata (name, slug,
 * category, accept spec, features). Persistence fields (cloudId, savedAt,
 * syncedAt) live on UserRecipe in @bnto/core — not here.
 */

import type { Definition } from "./definition";

/**
 * A bnto recipe — definition + display metadata.
 *
 * Predefined catalog recipes have curated slugs and hand-set metadata.
 * User-created recipes have UUID ids and slugs derived from the name.
 * Persistence state is NOT part of the recipe — it lives on UserRecipe
 * in @bnto/core, which extends this type.
 */
export interface Recipe {
  /** Unique identifier (UUID). */
  id: string;

  /** URL-safe slug (e.g., "compress-images"). Derived from name for user-created recipes. */
  slug: string;

  /** Display name (e.g., "Compress Images"). */
  name: string;

  /** User-facing description. One sentence, plain language. */
  description: string;

  /** Category for grouping (e.g., "image", "spreadsheet", "file"). */
  category: string;

  /** The full definition ready for execution. */
  definition: Definition;

  /** File types this recipe accepts as input. */
  accept: AcceptSpec;

  /** Feature tags for JSON-LD and display (e.g., ["PNG", "JPEG", "Browser-based"]). */
  features: string[];
}

/**
 * Describes the file types a recipe accepts as input.
 *
 * Used to configure the file drop zone — both MIME type matching
 * and extension filtering.
 */
export interface AcceptSpec {
  /** Specific MIME types (e.g., ["image/jpeg", "image/png"]). */
  mimeTypes: string[];

  /** File extensions with dot (e.g., [".jpg", ".png"]). */
  extensions: string[];

  /** Human-readable label (e.g., "JPEG, PNG, or WebP images"). */
  label: string;

  /** MIME type prefix for wildcard matching (e.g., "image/"). */
  mimePrefix?: string;
}
