/**
 * Node schema registry — maps node type name to its schema definition
 * and optional field configs.
 *
 * Separated from index.ts so helper files can import NODE_SCHEMAS
 * without circular dependencies.
 */

import type { NodeTypeName } from "../nodeTypes";
import type { NodeParamFields, NodeSchema } from "./types";

import { editFieldsNodeSchema, editFieldsFields } from "./editFields";
import { fileRenameNodeSchema, fileRenameFields } from "./fileRename";
import { groupNodeSchema, groupFields } from "./group";
import { imageCompressNodeSchema, imageCompressFields } from "./imageCompress";
import { imageConvertNodeSchema, imageConvertFields } from "./imageConvert";
import { imageResizeNodeSchema, imageResizeFields } from "./imageResize";
import { imageStripExifNodeSchema, imageStripExifFields } from "./imageStripExif";
import { inputNodeSchema, inputFields } from "./input";
import { loopNodeSchema, loopFields } from "./loop";
import { outputNodeSchema, outputFields } from "./output";
import { parallelNodeSchema, parallelFields } from "./parallel";
import { spreadsheetCleanNodeSchema, spreadsheetCleanFields } from "./spreadsheetClean";
import { spreadsheetRenameNodeSchema, spreadsheetRenameFields } from "./spreadsheetRename";
import { transformNodeSchema, transformFields } from "./transform";

/**
 * Schema definitions for 14 of 16 registered node types.
 *
 * Maps node type name -> NodeSchema (Zod schema + engine metadata).
 * Types without engine processors (http-request, shell-command) have
 * no schema — they'll get schemas when processors are implemented.
 */
export const NODE_SCHEMAS: Partial<Record<NodeTypeName, NodeSchema>> = {
  "edit-fields": editFieldsNodeSchema,
  "file-rename": fileRenameNodeSchema,
  group: groupNodeSchema,
  "image-compress": imageCompressNodeSchema,
  "image-convert": imageConvertNodeSchema,
  "image-resize": imageResizeNodeSchema,
  "image-strip-exif": imageStripExifNodeSchema,
  input: inputNodeSchema,
  loop: loopNodeSchema,
  output: outputNodeSchema,
  parallel: parallelNodeSchema,
  "spreadsheet-clean": spreadsheetCleanNodeSchema,
  "spreadsheet-rename": spreadsheetRenameNodeSchema,
  transform: transformNodeSchema,
} as const;

/**
 * UI field configs for 14 of 16 registered node types.
 *
 * Maps node type name -> NodeParamFields (UI presentation metadata).
 * Parallel structure to NODE_SCHEMAS — look up fields alongside
 * the schema to get the full picture for rendering.
 */
export const NODE_PARAM_FIELDS: Partial<Record<NodeTypeName, NodeParamFields>> = {
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
} as const;
