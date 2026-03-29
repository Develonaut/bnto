/**
 * All predefined recipes — the bnto catalog.
 *
 * Every recipe is a complete `Recipe` with I/O nodes.
 * Individual recipe definitions live in `./recipes/` (one file per recipe).
 */

import type { Recipe } from "./recipe";

import {
  cleanCsv,
  compressAndRename,
  compressImages,
  convertImageFormat,
  csvToJson,
  generateThumbnails,
  optimizeImagesForWeb,
  renameCsvColumns,
  renameFiles,
  resizeImages,
  standardizeCsv,
  stripExif,
  watermarkImages,
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
  csvToJson,
  optimizeImagesForWeb,
  generateThumbnails,
  compressAndRename,
  standardizeCsv,
  stripExif,
  watermarkImages,
];

// Re-export all recipes for direct access
export {
  cleanCsv,
  compressAndRename,
  compressImages,
  convertImageFormat,
  csvToJson,
  generateThumbnails,
  optimizeImagesForWeb,
  renameCsvColumns,
  renameFiles,
  resizeImages,
  standardizeCsv,
  stripExif,
  watermarkImages,
};
