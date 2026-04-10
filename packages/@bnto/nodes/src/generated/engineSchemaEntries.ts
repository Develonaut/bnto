/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeSchema } from "../schemas/types";
import type { NodeParamFields } from "../schemas/types";

import { fileRenameNodeSchema } from "./schemas/fileRename";
import { imageCompressNodeSchema } from "./schemas/imageCompress";
import { imageConvertNodeSchema } from "./schemas/imageConvert";
import { imageOverlayNodeSchema } from "./schemas/imageOverlay";
import { imageResizeNodeSchema } from "./schemas/imageResize";
import { imageStripExifNodeSchema } from "./schemas/imageStripExif";
import { spreadsheetCleanNodeSchema } from "./schemas/spreadsheetClean";
import { spreadsheetConvertNodeSchema } from "./schemas/spreadsheetConvert";
import { spreadsheetMergeNodeSchema } from "./schemas/spreadsheetMerge";
import { spreadsheetRenameNodeSchema } from "./schemas/spreadsheetRename";
import { vectorRasterizeNodeSchema } from "./schemas/vectorRasterize";
import { videoDownloadNodeSchema } from "./schemas/videoDownload";

/**
 * Engine-backed schema entries - auto-assembled from generated per-type files.
 * Merged with hand-written schemas in schemas/registry.ts.
 */
export const ENGINE_NODE_SCHEMAS: Record<string, NodeSchema> = {
  "file-rename": fileRenameNodeSchema,
  "image-compress": imageCompressNodeSchema,
  "image-convert": imageConvertNodeSchema,
  "image-overlay": imageOverlayNodeSchema,
  "image-resize": imageResizeNodeSchema,
  "image-strip-exif": imageStripExifNodeSchema,
  "spreadsheet-clean": spreadsheetCleanNodeSchema,
  "spreadsheet-convert": spreadsheetConvertNodeSchema,
  "spreadsheet-merge": spreadsheetMergeNodeSchema,
  "spreadsheet-rename": spreadsheetRenameNodeSchema,
  "vector-rasterize": vectorRasterizeNodeSchema,
  "video-download": videoDownloadNodeSchema,
};

/**
 * Default (empty) field configs for engine-backed types.
 * Hand-written overrides in schemas/{type}.ts take precedence.
 */
export const ENGINE_NODE_PARAM_FIELDS: Record<string, NodeParamFields> = {
  "file-rename": {},
  "image-compress": {},
  "image-convert": {},
  "image-overlay": {},
  "image-resize": {},
  "image-strip-exif": {},
  "spreadsheet-clean": {},
  "spreadsheet-convert": {},
  "spreadsheet-merge": {},
  "spreadsheet-rename": {},
  "vector-rasterize": {},
  "video-download": {},
};
