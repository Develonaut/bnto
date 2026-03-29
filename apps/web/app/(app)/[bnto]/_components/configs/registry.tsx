"use client";

import type { ComponentType, LazyExoticComponent } from "react";
import { lazy } from "react";
import type { BntoSlug } from "./types";

/**
 * Lazy-loaded config component registry.
 *
 * Maps each bnto slug to its lazy-loaded config component.
 * Each config is code-split — only the one needed for the
 * current recipe is downloaded.
 *
 * To add a new config: create the component file, then add
 * an entry here. RecipeConfigSection picks it up automatically.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any -- registry values are rendered with per-slug typed props
export const CONFIG_REGISTRY: Partial<Record<BntoSlug, LazyExoticComponent<ComponentType<any>>>> = {
  "compress-images": lazy(() =>
    import("./CompressImagesConfig").then((m) => ({ default: m.CompressImagesConfig })),
  ),
  "resize-images": lazy(() =>
    import("./ResizeImagesConfig").then((m) => ({ default: m.ResizeImagesConfig })),
  ),
  "convert-image-format": lazy(() =>
    import("./ConvertFormatConfig").then((m) => ({ default: m.ConvertFormatConfig })),
  ),
  "rename-files": lazy(() =>
    import("./RenameFilesConfig").then((m) => ({ default: m.RenameFilesConfig })),
  ),
  "clean-csv": lazy(() => import("./CleanCsvConfig").then((m) => ({ default: m.CleanCsvConfig }))),
  "rename-csv-columns": lazy(() =>
    import("./RenameCsvColumnsConfig").then((m) => ({ default: m.RenameCsvColumnsConfig })),
  ),
  "optimize-images-for-web": lazy(() =>
    import("./OptimizeImagesForWebConfig").then((m) => ({
      default: m.OptimizeImagesForWebConfig,
    })),
  ),
  "generate-thumbnails": lazy(() =>
    import("./GenerateThumbnailsConfig").then((m) => ({
      default: m.GenerateThumbnailsConfig,
    })),
  ),
  "watermark-images": lazy(() =>
    import("./WatermarkImagesConfig").then((m) => ({
      default: m.WatermarkImagesConfig,
    })),
  ),
};
