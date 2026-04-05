/**
 * All predefined recipes — the bnto catalog.
 *
 * Engine owns recipe definitions (nodes, edges, parameters).
 * This file derives `Recipe` objects from engine-generated data,
 * adding web-only overlays (features for JSON-LD, UUIDs for persistence).
 */

import { GENERATED_RECIPES } from "@bnto/nodes";
import type { Recipe } from "./recipe";
import { deriveAcceptSpec } from "./deriveAcceptSpec";

/**
 * Stable UUIDs for predefined recipes.
 * These were assigned when recipes were first created and must not change.
 */
const RECIPE_IDS: Record<string, string> = {
  "compress-images": "04ad520d-3afe-470b-8225-6cae2b14c402",
  "resize-images": "ea8bec03-b732-4bc6-aeec-1097f75c0b87",
  "convert-image-format": "9cf4fa85-9145-49f7-b838-dae759261ba2",
  "rename-files": "404cea0b-ea2c-4d81-bd89-47f5eb3b8389",
  "clean-csv": "5beec7db-f553-4e99-90f3-07be1d419db8",
  "rename-csv-columns": "5be92d39-fa8e-49ce-a0f3-c55d01a48c7b",
  "csv-to-json": "a7c3e1f0-8d42-4b9a-b5e6-3f1a2c4d5e6f",
  "merge-csv": "b4d5e6f7-8a9b-0c1d-2e3f-4a5b6c7d8e9f",
  "optimize-images-for-web": "e8c3a3ad-1e7e-40f6-ab60-00f405514f6f",
  "generate-thumbnails": "4cf63f58-327a-4e6a-a03c-caa9679c7152",
  "compress-and-rename": "b3a1c7d2-8f4e-4a6b-9c5d-1e2f3a4b5c6d",
  "standardize-csv": "a2b3c4d5-6e7f-8a9b-0c1d-2e3f4a5b6c7d",
  "strip-exif": "b8d4f2a1-9e53-4c0b-a6f7-4g2b3d5e7f8a",
  "watermark-images": "b7d2a3f1-8e4c-4d6b-9a1f-2c5e7b8d3f4a",
  "download-video": "d7e2c891-4f3a-4b1e-9c8d-2a5f6e7b3c01",
};

/**
 * Web-only feature tags for JSON-LD and display.
 * These are not in the engine — they're marketing/SEO metadata.
 */
const WEB_FEATURES: Record<string, string[]> = {
  "compress-images": ["PNG", "JPEG", "WebP", "No upload", "Browser-based"],
  "resize-images": ["PNG", "JPEG", "WebP", "Custom dimensions", "Browser-based"],
  "convert-image-format": ["PNG", "JPEG", "WebP", "GIF", "Browser-based"],
  "rename-files": ["Batch rename", "Pattern matching", "Browser-based"],
  "clean-csv": ["CSV", "Remove duplicates", "Trim whitespace", "Browser-based"],
  "rename-csv-columns": ["CSV", "Column rename", "Bulk edit", "Browser-based"],
  "csv-to-json": ["CSV", "JSON", "Configurable delimiter", "Browser-based"],
  "merge-csv": ["CSV", "Merge", "Header reconciliation", "Deduplication", "Browser-based"],
  "optimize-images-for-web": ["Resize", "WebP", "Compress", "Multi-step", "Browser-based"],
  "generate-thumbnails": ["Thumbnails", "Resize", "WebP", "Multi-step", "Browser-based"],
  "compress-and-rename": ["Compress", "Rename", "Suffix", "Multi-step", "Browser-based"],
  "standardize-csv": ["CSV", "Clean", "Rename columns", "Multi-step", "Browser-based"],
  "strip-exif": ["PNG", "JPEG", "WebP", "No upload", "Browser-based"],
  "watermark-images": [
    "PNG",
    "JPEG",
    "WebP",
    "Configurable position",
    "Adjustable opacity",
    "Browser-based",
  ],
  "download-video": ["YouTube", "m3u8/HLS", "MP4/WebM/MKV", "Audio extraction"],
};

/**
 * Display order for recipes on the home page grid.
 * Slugs listed first appear first. Unlisted slugs go at the end.
 */
const DISPLAY_ORDER: string[] = [
  "compress-images",
  "resize-images",
  "convert-image-format",
  "rename-files",
  "clean-csv",
  "rename-csv-columns",
  "csv-to-json",
  "merge-csv",
  "optimize-images-for-web",
  "generate-thumbnails",
  "compress-and-rename",
  "standardize-csv",
  "strip-exif",
  "watermark-images",
  "download-video",
];

/**
 * All predefined recipes that map to public URLs.
 *
 * Derived from engine-generated recipes with web-only overlays
 * (UUIDs, features, accept specs). Order follows DISPLAY_ORDER.
 */
export const RECIPES: readonly Recipe[] = DISPLAY_ORDER.map((slug) => {
  const generated = GENERATED_RECIPES.find((r) => r.slug === slug);
  if (!generated) throw new Error(`Recipe "${slug}" listed in DISPLAY_ORDER but not in engine`);

  return {
    id: RECIPE_IDS[slug] ?? slug,
    slug: generated.slug,
    name: generated.name,
    description: generated.description,
    category: generated.category,
    definition: generated.definition,
    accept: deriveAcceptSpec(generated.definition) ?? {
      mimeTypes: [],
      extensions: [],
      label: "Files",
    },
    features: WEB_FEATURES[slug] ?? [],
  };
}).filter(Boolean) as Recipe[];
