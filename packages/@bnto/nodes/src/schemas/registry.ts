/**
 * Node schema registry — maps node type name to its schema definition
 * and optional field configs.
 *
 * Separated from index.ts so helper files can import NODE_SCHEMA_DEFS
 * without circular dependencies.
 */

import type { NodeTypeName } from "../nodeTypes";
import type { FieldConfigMap, NodeSchemaDefinition } from "./types";

import { editFieldsNodeSchema, editFieldsFields } from "./editFields";
import { fileRenameNodeSchema, fileRenameFields } from "./fileRename";
import { groupNodeSchema, groupFields } from "./group";
import { imageCompressNodeSchema, imageCompressFields } from "./imageCompress";
import { imageConvertNodeSchema, imageConvertFields } from "./imageConvert";
import { imageResizeNodeSchema, imageResizeFields } from "./imageResize";
import { inputNodeSchema, inputFields } from "./input";
import { loopNodeSchema, loopFields } from "./loop";
import { outputNodeSchema, outputFields } from "./output";
import { parallelNodeSchema, parallelFields } from "./parallel";
import { spreadsheetCleanNodeSchema, spreadsheetCleanFields } from "./spreadsheetClean";
import { spreadsheetRenameNodeSchema, spreadsheetRenameFields } from "./spreadsheetRename";
import { transformNodeSchema, transformFields } from "./transform";

/**
 * Schema definitions for 13 of 15 registered node types.
 *
 * Maps node type name -> NodeSchemaDefinition (Zod schema + engine metadata).
 * Types without engine processors (http-request, shell-command) have
 * no schema — they'll get schemas when processors are implemented.
 */
export const NODE_SCHEMA_DEFS: Partial<Record<NodeTypeName, NodeSchemaDefinition>> = {
  "edit-fields": editFieldsNodeSchema,
  "file-rename": fileRenameNodeSchema,
  group: groupNodeSchema,
  "image-compress": imageCompressNodeSchema,
  "image-convert": imageConvertNodeSchema,
  "image-resize": imageResizeNodeSchema,
  input: inputNodeSchema,
  loop: loopNodeSchema,
  output: outputNodeSchema,
  parallel: parallelNodeSchema,
  "spreadsheet-clean": spreadsheetCleanNodeSchema,
  "spreadsheet-rename": spreadsheetRenameNodeSchema,
  transform: transformNodeSchema,
} as const;

/**
 * UI field configs for 13 of 15 registered node types.
 *
 * Maps node type name -> FieldConfigMap (UI presentation metadata).
 * Parallel structure to NODE_SCHEMA_DEFS — look up fields alongside
 * the schema to get the full picture for rendering.
 */
export const NODE_FIELD_CONFIGS: Partial<Record<NodeTypeName, FieldConfigMap>> = {
  "edit-fields": editFieldsFields,
  "file-rename": fileRenameFields,
  group: groupFields,
  "image-compress": imageCompressFields,
  "image-convert": imageConvertFields,
  "image-resize": imageResizeFields,
  input: inputFields,
  loop: loopFields,
  output: outputFields,
  parallel: parallelFields,
  "spreadsheet-clean": spreadsheetCleanFields,
  "spreadsheet-rename": spreadsheetRenameFields,
  transform: transformFields,
} as const;
