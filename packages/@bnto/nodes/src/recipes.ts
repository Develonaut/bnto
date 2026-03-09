/**
 * All predefined recipes — the bnto catalog.
 *
 * Every recipe is a complete `Recipe` with I/O nodes.
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
 * All predefined recipes — the 6 Tier 1 bntos that map to public URLs.
 *
 * Order determines display order in the UI grid on the home page.
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

// Re-export all recipes for direct access
export {
  cleanCsv,
  compressImages,
  convertImageFormat,
  renameCsvColumns,
  renameFiles,
  resizeImages,
};
