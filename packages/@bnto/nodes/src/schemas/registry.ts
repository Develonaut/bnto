/**
 * Node schema registry -- the assembled mapping of node type name to schema.
 *
 * Separated from index.ts so helper files can import NODE_SCHEMAS
 * without circular dependencies.
 */

import type { NodeTypeName } from "../nodeTypes";
import type { NodeSchema } from "./types";

import { editFieldsSchema } from "./editFields";
import { fileSystemSchema } from "./fileSystem";
import { groupSchema } from "./group";
import { httpRequestSchema } from "./httpRequest";
import { imageSchema } from "./image";
import { inputSchema } from "./input";
import { loopSchema } from "./loop";
import { outputSchema } from "./output";
import { parallelSchema } from "./parallel";
import { shellCommandSchema } from "./shellCommand";
import { spreadsheetSchema } from "./spreadsheet";
import { transformSchema } from "./transform";

/**
 * Parameter schemas for all 12 registered node types.
 *
 * Maps node type name -> schema. Drives the config panel UI --
 * each schema describes what parameters the node accepts.
 */
export const NODE_SCHEMAS: Record<NodeTypeName, NodeSchema> = {
  "edit-fields": editFieldsSchema,
  "file-system": fileSystemSchema,
  group: groupSchema,
  "http-request": httpRequestSchema,
  image: imageSchema,
  input: inputSchema,
  loop: loopSchema,
  output: outputSchema,
  parallel: parallelSchema,
  "shell-command": shellCommandSchema,
  spreadsheet: spreadsheetSchema,
  transform: transformSchema,
} as const;
