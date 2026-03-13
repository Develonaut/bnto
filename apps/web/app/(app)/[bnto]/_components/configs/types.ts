/**
 * Per-bnto configuration types.
 *
 * Each bnto slug maps to a config shape that drives the tool's
 * context-specific controls. These configs are passed to the
 * execution flow when the user clicks "Run".
 */

export interface CompressImagesConfig {
  compression: number;
}

export interface ResizeImagesConfig {
  width: number;
  maintainAspectRatio: boolean;
}

export interface ConvertFormatConfig {
  format: "webp" | "jpeg" | "png";
  quality: number;
}

export interface RenameFilesConfig {
  pattern: string;
}

export interface CleanCsvConfig {
  trimWhitespace: boolean;
  removeEmptyRows: boolean;
  removeDuplicates: boolean;
}

export interface OptimizeImagesForWebConfig {
  width: number;
  format: "webp" | "jpeg" | "png";
  compression: number;
}

export interface GenerateThumbnailsConfig {
  width: number;
  format: "webp" | "jpeg" | "png";
  prefix: string;
}

/**
 * Column mapping is blocked until array-level transforms ship.
 * Config is empty for now — UI shows a placeholder.
 */
export type RenameCsvColumnsConfig = Record<string, never>;

/** Union of all bnto config types, keyed by slug. */
export type BntoConfigMap = {
  "compress-images": CompressImagesConfig;
  "resize-images": ResizeImagesConfig;
  "convert-image-format": ConvertFormatConfig;
  "rename-files": RenameFilesConfig;
  "clean-csv": CleanCsvConfig;
  "rename-csv-columns": RenameCsvColumnsConfig;
  "optimize-images-for-web": OptimizeImagesForWebConfig;
  "generate-thumbnails": GenerateThumbnailsConfig;
};

export type BntoSlug = keyof BntoConfigMap;

/** Default configs for each bnto — matches fixture defaults. */
export const DEFAULT_CONFIGS: BntoConfigMap = {
  "compress-images": { compression: 20 },
  "resize-images": { width: 800, maintainAspectRatio: true },
  "convert-image-format": { format: "webp", quality: 80 },
  "rename-files": { pattern: "renamed-{{name}}" },
  "clean-csv": {
    trimWhitespace: true,
    removeEmptyRows: true,
    removeDuplicates: false,
  },
  "rename-csv-columns": {},
  "optimize-images-for-web": { width: 800, format: "webp", compression: 20 },
  "generate-thumbnails": { width: 150, format: "webp", prefix: "thumb_" },
};
