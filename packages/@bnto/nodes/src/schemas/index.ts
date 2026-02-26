/**
 * Node parameter schemas — registry and helpers.
 *
 * Assembles per-type schemas into a single registry keyed by node type name.
 * Re-exports helpers from individual files for one-export-per-file compliance.
 */

// Registry (assembled NODE_SCHEMAS constant)
export { NODE_SCHEMAS } from "./registry";

// Helper functions (extracted to individual files)
export { getNodeSchema } from "./getNodeSchema";
export { getRequiredParams } from "./getRequiredParams";
export { getConditionallyRequired } from "./getConditionallyRequired";
export { getVisibleParams } from "./getVisibleParams";

// Re-export types and per-type constants
export type { NodeSchema, ParameterSchema, ParameterType } from "./types";
export { HTTP_METHODS } from "./httpRequest";
export { FILE_OPERATIONS } from "./fileSystem";
export { LOOP_MODES } from "./loop";
export { IMAGE_OPERATIONS, IMAGE_FORMATS } from "./image";
export { SPREADSHEET_OPERATIONS, SPREADSHEET_FORMATS } from "./spreadsheet";
export { GROUP_MODES } from "./group";
export { ERROR_STRATEGIES } from "./parallel";
