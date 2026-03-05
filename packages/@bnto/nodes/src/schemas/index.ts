/**
 * Node parameter schemas — registry and helpers.
 *
 * Assembles per-type Zod schemas into a single registry keyed by node type name.
 * Re-exports helpers from individual files for one-export-per-file compliance.
 */

// Registry
export { NODE_SCHEMA_DEFS } from "./registry";

// Helper functions
export { getNodeSchema } from "./getNodeSchema";
export { getRequiredParams } from "./getRequiredParams";
export { getConditionallyRequired } from "./getConditionallyRequired";
export { getVisibleParams } from "./getVisibleParams";
export { inferFieldType } from "./inferFieldType";
export type { FieldTypeInfo, FieldControl } from "./inferFieldType";

// Types
export type { NodeSchemaDefinition, NodeParamMeta, ParamCondition } from "./types";

// Per-type enum constants
export { HTTP_METHODS } from "./httpRequest";
export { FILE_OPERATIONS } from "./fileSystem";
export { INPUT_MODES } from "./input";
export { LOOP_MODES } from "./loop";
export { OUTPUT_MODES } from "./output";
export { IMAGE_OPERATIONS, IMAGE_FORMATS } from "./image";
export { SPREADSHEET_OPERATIONS, SPREADSHEET_FORMATS } from "./spreadsheet";
export { GROUP_MODES } from "./group";
export { ERROR_STRATEGIES } from "./parallel";

// Per-type Zod schemas and inferred types
export { imageParamsSchema, imageNodeSchema } from "./image";
export type { ImageParams } from "./image";
export { inputParamsSchema, inputNodeSchema } from "./input";
export type { InputParams } from "./input";
export { outputParamsSchema, outputNodeSchema } from "./output";
export type { OutputParams } from "./output";
export { fileSystemParamsSchema, fileSystemNodeSchema } from "./fileSystem";
export type { FileSystemParams } from "./fileSystem";
export { httpRequestParamsSchema, httpRequestNodeSchema } from "./httpRequest";
export type { HttpRequestParams } from "./httpRequest";
export { spreadsheetParamsSchema, spreadsheetNodeSchema } from "./spreadsheet";
export type { SpreadsheetParams } from "./spreadsheet";
export { transformParamsSchema, transformNodeSchema } from "./transform";
export type { TransformParams } from "./transform";
export { editFieldsParamsSchema, editFieldsNodeSchema } from "./editFields";
export type { EditFieldsParams } from "./editFields";
export { loopParamsSchema, loopNodeSchema } from "./loop";
export type { LoopParams } from "./loop";
export { groupParamsSchema, groupNodeSchema } from "./group";
export type { GroupParams } from "./group";
export { parallelParamsSchema, parallelNodeSchema } from "./parallel";
export type { ParallelParams } from "./parallel";
export { shellCommandParamsSchema, shellCommandNodeSchema } from "./shellCommand";
export type { ShellCommandParams } from "./shellCommand";
