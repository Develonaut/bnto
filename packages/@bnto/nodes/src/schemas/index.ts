/**
 * Node parameter schemas — registry and helpers.
 *
 * Assembles per-type Zod schemas into a single registry keyed by node type name.
 * Re-exports helpers from individual files for one-export-per-file compliance.
 */

// Registry
export { NODE_SCHEMAS, NODE_PARAM_FIELDS } from "./registry";

// Helper functions
export { getNodeSchema } from "./getNodeSchema";
export { getNodeParamFields } from "./getNodeParamFields";
export { getRequiredParams } from "./getRequiredParams";
export { getConditionallyRequired } from "./getConditionallyRequired";
export { getVisibleParams } from "./getVisibleParams";
export { inferFieldType } from "./inferFieldType";
export type { NodeParamFieldInfo, NodeParamControl } from "./inferFieldType";

// Types
export type {
  NodeSchema,
  NodeParam,
  ParamCondition,
  NodeParamField,
  NodeParamFields,
} from "./types";

// Per-type enum constants
export { INPUT_MODES } from "./input";
export { LOOP_MODES } from "./loop";
export { OUTPUT_MODES } from "./output";
export { IMAGE_FORMATS } from "./imageConvert";
export { GROUP_MODES } from "./group";
export { ERROR_STRATEGIES } from "./parallel";

// Per-type Zod schemas and inferred types (per-operation)
export { imageCompressParamsSchema, imageCompressNodeSchema } from "./imageCompress";
export type { ImageCompressParams } from "./imageCompress";
export { imageResizeParamsSchema, imageResizeNodeSchema } from "./imageResize";
export type { ImageResizeParams } from "./imageResize";
export { imageConvertParamsSchema, imageConvertNodeSchema } from "./imageConvert";
export type { ImageConvertParams } from "./imageConvert";
export { spreadsheetCleanParamsSchema, spreadsheetCleanNodeSchema } from "./spreadsheetClean";
export type { SpreadsheetCleanParams } from "./spreadsheetClean";
export { spreadsheetRenameParamsSchema, spreadsheetRenameNodeSchema } from "./spreadsheetRename";
export type { SpreadsheetRenameParams } from "./spreadsheetRename";
export { fileRenameParamsSchema, fileRenameNodeSchema } from "./fileRename";
export type { FileRenameParams } from "./fileRename";

// Non-engine schema types (hand-written, not per-operation)
export { inputParamsSchema, inputNodeSchema } from "./input";
export type { InputParams } from "./input";
export { outputParamsSchema, outputNodeSchema } from "./output";
export type { OutputParams } from "./output";
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
