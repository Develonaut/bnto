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
import { httpRequestNodeSchema } from "./httpRequest";
import { imageNodeSchema } from "./image";
import { inputNodeSchema } from "./input";
import { loopNodeSchema } from "./loop";
import { outputNodeSchema } from "./output";
import { parallelNodeSchema } from "./parallel";
import { shellCommandNodeSchema } from "./shellCommand";
import { spreadsheetNodeSchema } from "./spreadsheet";
import { transformNodeSchema } from "./transform";

/**
 * Schema definitions for all 12 registered node types.
 *
 * Maps node type name -> NodeSchemaDefinition (Zod schema + UI metadata).
 */
export const NODE_SCHEMA_DEFS: Record<NodeTypeName, NodeSchemaDefinition> = {
  "edit-fields": editFieldsNodeSchema,
  "file-system": fileSystemNodeSchema,
  group: groupNodeSchema,
  "http-request": httpRequestNodeSchema,
  image: imageNodeSchema,
  input: inputNodeSchema,
  loop: loopNodeSchema,
  output: outputNodeSchema,
  parallel: parallelNodeSchema,
  "shell-command": shellCommandNodeSchema,
  spreadsheet: spreadsheetNodeSchema,
  transform: transformNodeSchema,
} as const;
