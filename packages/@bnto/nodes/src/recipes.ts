/**
 * All predefined recipes — the bnto catalog.
 *
 * Every recipe is a complete `Recipe` with I/O nodes. Building-block
 * recipes and composite recipes have the same shape.
 *
 * Individual recipe definitions live in `./recipes/` (one file per recipe).
 */

import type { Recipe } from "./recipe";

import {
  batchCompress,
  batchConvert,
  batchRename,
  batchResize,
  cleanCsv,
  columnRenamer,
  compressImages,
  convertImageFormat,
  csvCleaner,
  renameCsvColumns,
  renameFiles,
  resizeImages,
} from "./recipes/index";

/**
 * Public recipes — the 6 Tier 1 bntos that map to public URLs.
 *
 * Order determines display order in the UI grid on the home page.
 * These are the only recipes shown to users in the gallery.
 */
export const RECIPES: readonly Recipe[] = [
  compressImages,
  resizeImages,
  convertImageFormat,
  renameFiles,
  cleanCsv,
  renameCsvColumns,
];

/**
 * All recipes in the catalog — public + building-block.
 *
 * Building-block recipes are used in the editor and dev tools
 * but are not shown on the home page or public gallery.
 */
export const ALL_RECIPES: readonly Recipe[] = [
  ...RECIPES,
  batchCompress,
  batchResize,
  batchConvert,
  batchRename,
  csvCleaner,
  columnRenamer,
];

/** Returns the recipe matching a URL slug, or undefined if not found. */
export function getRecipeBySlug(slug: string): Recipe | undefined {
  return ALL_RECIPES.find((r) => r.slug === slug);
}

// Re-export all recipes for direct access
export {
  batchCompress,
  batchConvert,
  batchRename,
  batchResize,
  cleanCsv,
  columnRenamer,
  compressImages,
  convertImageFormat,
  csvCleaner,
  renameCsvColumns,
  renameFiles,
  resizeImages,
};
