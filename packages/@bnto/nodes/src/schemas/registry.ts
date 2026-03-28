/**
 * Node schema registry — maps node type name to its schema definition
 * and optional field configs.
 *
 * Engine-backed schemas are auto-assembled in `generated/engineSchemaEntries.ts`.
 * Hand-written schemas provide UI augmentations and non-engine node types.
 * This file merges both into the canonical registries.
 *
 * Separated from index.ts so helper files can import NODE_SCHEMAS
 * without circular dependencies.
 */

import type { NodeTypeName } from "../nodeTypes";
import type { NodeParamFields, NodeSchema } from "./types";

import { ENGINE_NODE_SCHEMAS, ENGINE_NODE_PARAM_FIELDS } from "../generated/engineSchemaEntries";

// Hand-written schemas (non-engine types + UI augmentations)
import { editFieldsNodeSchema, editFieldsFields } from "./editFields";
import { fileRenameFields } from "./fileRename";
import { groupNodeSchema, groupFields } from "./group";
import { imageCompressFields } from "./imageCompress";
import { imageConvertFields } from "./imageConvert";
import { imageResizeFields } from "./imageResize";
import { imageStripExifFields } from "./imageStripExif";
import { inputNodeSchema, inputFields } from "./input";
import { loopNodeSchema, loopFields } from "./loop";
import { outputNodeSchema, outputFields } from "./output";
import { parallelNodeSchema, parallelFields } from "./parallel";
import { spreadsheetCleanFields } from "./spreadsheetClean";
import { spreadsheetRenameFields } from "./spreadsheetRename";
import { transformNodeSchema, transformFields } from "./transform";

// Non-engine schemas (hand-written, no engine processor)
const HAND_WRITTEN_SCHEMAS: Partial<Record<string, NodeSchema>> = {
  "edit-fields": editFieldsNodeSchema,
  group: groupNodeSchema,
  input: inputNodeSchema,
  loop: loopNodeSchema,
  output: outputNodeSchema,
  parallel: parallelNodeSchema,
  transform: transformNodeSchema,
};

// UI field overrides (hand-written augmentations for engine-backed + non-engine types)
const HAND_WRITTEN_FIELDS: Partial<Record<string, NodeParamFields>> = {
  "edit-fields": editFieldsFields,
  "file-rename": fileRenameFields,
  group: groupFields,
  "image-compress": imageCompressFields,
  "image-convert": imageConvertFields,
  "image-resize": imageResizeFields,
  "image-strip-exif": imageStripExifFields,
  input: inputFields,
  loop: loopFields,
  output: outputFields,
  parallel: parallelFields,
  "spreadsheet-clean": spreadsheetCleanFields,
  "spreadsheet-rename": spreadsheetRenameFields,
  transform: transformFields,
};

/**
 * Schema definitions for all registered node types with schemas.
 *
 * Engine-backed schemas are auto-generated. Hand-written schemas cover
 * non-engine types (input, output, loop, group, etc.). Engine schemas
 * are listed first, hand-written override if they provide their own.
 */
export const NODE_SCHEMAS: Partial<Record<NodeTypeName, NodeSchema>> = {
  ...ENGINE_NODE_SCHEMAS,
  ...HAND_WRITTEN_SCHEMAS,
} as const;

/**
 * UI field configs for all registered node types with schemas.
 *
 * Engine defaults are empty objects. Hand-written overrides add
 * presets, suffixes, groups, and other UI presentation metadata.
 */
export const NODE_PARAM_FIELDS: Partial<Record<NodeTypeName, NodeParamFields>> = {
  ...ENGINE_NODE_PARAM_FIELDS,
  ...HAND_WRITTEN_FIELDS,
} as const;
