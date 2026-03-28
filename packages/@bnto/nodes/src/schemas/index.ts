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

// Engine-backed Zod schemas and inferred types (auto-generated, per-processor)
export * from "../generated/schemas";

// Non-engine schema types (hand-written, not per-operation)
export { inputParamsSchema, inputNodeSchema } from "./input";
export type { InputParams } from "./input";
export { outputParamsSchema, outputNodeSchema } from "./output";
export type { OutputParams } from "./output";
export { transformParamsSchema, transformNodeSchema } from "./transform";
export { editFieldsParamsSchema, editFieldsNodeSchema } from "./editFields";
export { loopParamsSchema, loopNodeSchema } from "./loop";
export { groupParamsSchema, groupNodeSchema } from "./group";
export { parallelParamsSchema, parallelNodeSchema } from "./parallel";
