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
  generateThumbnails,
  optimizeImagesForWeb,
  renameCsvColumns,
  renameFiles,
  resizeImages,
} from "./recipes/index";

/**
 * All predefined recipes that map to public URLs.
 *
 * Order determines display order in the UI grid on the home page.
 * Tier 1: single-op recipes. Tier 2: multi-node compositions.
 */
export const RECIPES: readonly Recipe[] = [
  compressImages,
  resizeImages,
  convertImageFormat,
  renameFiles,
  cleanCsv,
  renameCsvColumns,
  optimizeImagesForWeb,
  generateThumbnails,
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
  generateThumbnails,
  optimizeImagesForWeb,
  renameCsvColumns,
  renameFiles,
  resizeImages,
};
