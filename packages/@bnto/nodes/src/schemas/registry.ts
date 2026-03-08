/**
 * Node schema registry — maps node type name to its schema definition.
 *
 * Separated from index.ts so helper files can import NODE_SCHEMA_DEFS
 * without circular dependencies.
 */

import type { NodeTypeName } from "../nodeTypes";
import type { NodeSchemaDefinition } from "./types";

import { editFieldsNodeSchema } from "./editFields";
import { fileSystemNodeSchema } from "./fileSystem";
import { groupNodeSchema } from "./group";
import { imageNodeSchema } from "./image";
import { inputNodeSchema } from "./input";
import { loopNodeSchema } from "./loop";
import { outputNodeSchema } from "./output";
import { parallelNodeSchema } from "./parallel";
import { spreadsheetNodeSchema } from "./spreadsheet";
import { transformNodeSchema } from "./transform";

/**
 * Schema definitions for 10 of 12 registered node types.
 *
 * Maps node type name -> NodeSchemaDefinition (Zod schema + UI metadata).
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
