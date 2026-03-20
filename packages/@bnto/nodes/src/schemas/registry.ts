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
import { fileSystemNodeSchema, fileSystemFields } from "./fileSystem";
import { groupNodeSchema, groupFields } from "./group";
import { imageNodeSchema, imageFields } from "./image";
import { inputNodeSchema, inputFields } from "./input";
import { loopNodeSchema, loopFields } from "./loop";
import { outputNodeSchema, outputFields } from "./output";
import { parallelNodeSchema, parallelFields } from "./parallel";
import { spreadsheetNodeSchema, spreadsheetFields } from "./spreadsheet";
import { transformNodeSchema, transformFields } from "./transform";

/**
 * Schema definitions for 10 of 12 registered node types.
 *
 * Maps node type name -> NodeSchemaDefinition (Zod schema + engine metadata).
 * Types without engine processors (http-request, shell-command) have
 * no schema — they'll get schemas when processors are implemented.
 */
export const NODE_SCHEMA_DEFS: Partial<Record<NodeTypeName, NodeSchemaDefinition>> = {
  "edit-fields": editFieldsNodeSchema,
  "file-system": fileSystemNodeSchema,
  group: groupNodeSchema,
  image: imageNodeSchema,
  input: inputNodeSchema,
  loop: loopNodeSchema,
  output: outputNodeSchema,
  parallel: parallelNodeSchema,
  spreadsheet: spreadsheetNodeSchema,
  transform: transformNodeSchema,
} as const;

/**
 * UI field configs for 10 of 12 registered node types.
 *
 * Maps node type name -> FieldConfigMap (UI presentation metadata).
 * Parallel structure to NODE_SCHEMA_DEFS — look up fields alongside
 * the schema to get the full picture for rendering.
 */
export const NODE_FIELD_CONFIGS: Partial<Record<NodeTypeName, FieldConfigMap>> = {
  "edit-fields": editFieldsFields,
  "file-system": fileSystemFields,
  group: groupFields,
  image: imageFields,
  input: inputFields,
  loop: loopFields,
  output: outputFields,
  parallel: parallelFields,
  spreadsheet: spreadsheetFields,
  transform: transformFields,
} as const;
