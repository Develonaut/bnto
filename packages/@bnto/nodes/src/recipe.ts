/**
 * Recipe types — predefined bnto definitions with metadata.
 *
 * A recipe bundles SEO metadata, file acceptance rules, and a full
 * definition into a single JSON blob. The engine owns these
 * definitions; consumers (web, desktop, CLI) read them.
 *
 * Go source: engine/pkg/menu/types.go
 */

import type { Definition } from "./definition";

/**
 * A predefined bnto recipe — metadata + acceptance rules + definition.
 *
 * Recipes are the "pre-assembled bento boxes" in the catalog. Each one
 * maps to a public URL at `/{slug}` and drives static generation,
 * metadata, and the file drop zone.
 */
export interface Recipe {
  /** URL slug (e.g., "compress-images", "clean-csv"). */
  slug: string;

  /** Display name (e.g., "Compress Images"). */
  name: string;

  /** User-facing description. One sentence, plain language. */
  description: string;

  /** Category for grouping (e.g., "image", "spreadsheet", "file"). */
  category: string;

  /** File types this recipe accepts as input. */
  accept: AcceptSpec;

  /** Feature tags for JSON-LD and display (e.g., ["PNG", "JPEG", "Browser-based"]). */
  features: string[];

  /** Search engine optimization metadata. */
  seo: SEOSpec;

  /** The full definition ready for execution. */
  definition: Definition;
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

/**
 * Search engine metadata for the recipe's public page.
 */
export interface SEOSpec {
  /** Page title — "[Action] Online Free -- bnto". */
  title: string;

  /** H1 heading — exact target search query. */
  h1: string;
}
