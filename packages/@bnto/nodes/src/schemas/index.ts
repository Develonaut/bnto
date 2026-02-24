/**
 * Node parameter schemas — registry and helpers.
 *
 * Assembles per-type schemas into a single registry keyed by node type name.
 * Provides helpers for querying parameter requirements.
 */

import type { NodeTypeName } from "../nodeTypes";
import type { NodeSchema, ParameterSchema } from "./types";

import { editFieldsSchema } from "./editFields";
import { fileSystemSchema } from "./fileSystem";
import { groupSchema } from "./group";
import { httpRequestSchema } from "./httpRequest";
import { imageSchema } from "./image";
import { loopSchema } from "./loop";
import { parallelSchema } from "./parallel";
import { shellCommandSchema } from "./shellCommand";
import { spreadsheetSchema } from "./spreadsheet";
import { transformSchema } from "./transform";

/**
 * Parameter schemas for all 10 registered node types.
 *
 * Maps node type name → schema. Drives the config panel UI —
 * each schema describes what parameters the node accepts.
 */
export const NODE_SCHEMAS: Record<NodeTypeName, NodeSchema> = {
  "edit-fields": editFieldsSchema,
  "file-system": fileSystemSchema,
  group: groupSchema,
  "http-request": httpRequestSchema,
  image: imageSchema,
  loop: loopSchema,
  parallel: parallelSchema,
  "shell-command": shellCommandSchema,
  spreadsheet: spreadsheetSchema,
  transform: transformSchema,
} as const;

/** Returns the schema for a node type, or undefined if not registered. */
export function getNodeSchema(typeName: string): NodeSchema | undefined {
  return NODE_SCHEMAS[typeName as NodeTypeName];
}

/** Returns only the required parameters for a node type. */
export function getRequiredParams(
  typeName: string,
): readonly ParameterSchema[] {
  const schema = getNodeSchema(typeName);
  if (!schema) return [];
  return schema.parameters.filter((p) => p.required);
}

/**
 * Returns parameters that are conditionally required when a
 * specific parameter has a specific value.
 *
 * Example: `getConditionallyRequired("loop", "mode", "forEach")`
 * returns `[{ name: "items", ... }]`.
 */
export function getConditionallyRequired(
  typeName: string,
  paramName: string,
  paramValue: string,
): readonly ParameterSchema[] {
  const schema = getNodeSchema(typeName);
  if (!schema) return [];
  return schema.parameters.filter(
    (p) =>
      p.requiredWhen?.param === paramName &&
      p.requiredWhen.equals === paramValue,
  );
}

/**
 * Returns parameters that are visible when a specific parameter
 * has a specific value.
 *
 * Example: `getVisibleParams("image", "operation", "resize")`
 * returns width, height, and maintainAspect parameters.
 */
export function getVisibleParams(
  typeName: string,
  paramName: string,
  paramValue: string,
): readonly ParameterSchema[] {
  const schema = getNodeSchema(typeName);
  if (!schema) return [];
  return schema.parameters.filter(
    (p) =>
      !p.visibleWhen ||
      (p.visibleWhen.param === paramName &&
        p.visibleWhen.equals === paramValue),
  );
}

// Re-export types and per-type constants
export type { NodeSchema, ParameterSchema, ParameterType } from "./types";
export { HTTP_METHODS } from "./httpRequest";
export { FILE_OPERATIONS } from "./fileSystem";
export { LOOP_MODES } from "./loop";
export { IMAGE_OPERATIONS, IMAGE_FORMATS } from "./image";
export { SPREADSHEET_OPERATIONS, SPREADSHEET_FORMATS } from "./spreadsheet";
export { GROUP_MODES } from "./group";
export { ERROR_STRATEGIES } from "./parallel";
