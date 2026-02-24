/**
 * All predefined Tier 1 recipes — the bnto catalog.
 *
 * Each recipe maps to a public URL at `/{slug}` and drives static
 * generation, metadata, and the file drop zone.
 *
 * Individual recipe definitions live in `./recipes/` (one file per recipe).
 */

import type { Recipe } from "./recipe";

import {
  cleanCsv,
  compressImages,
  convertImageFormat,
  renameCsvColumns,
  renameFiles,
  resizeImages,
} from "./recipes/index";

/**
 * All predefined recipes in the catalog.
 *
 * Order determines display order in the UI grid.
 */
export const RECIPES: readonly Recipe[] = [
  compressImages,
  resizeImages,
  convertImageFormat,
  renameFiles,
  cleanCsv,
  renameCsvColumns,
];

/** Returns the recipe matching a URL slug, or undefined if not found. */
export function getRecipeBySlug(slug: string): Recipe | undefined {
  return RECIPES.find((r) => r.slug === slug);
}

// Re-export individual recipes for direct access
export {
  cleanCsv,
  compressImages,
  convertImageFormat,
  renameCsvColumns,
  renameFiles,
  resizeImages,
};
