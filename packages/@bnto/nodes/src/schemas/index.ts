/**
 * Node parameter schemas — registry and helpers.
 *
 * All schemas, field configs, and enum constants are auto-generated
 * from the engine catalog. No hand-written overlays needed.
 */

// Registry
export { NODE_SCHEMAS, NODE_PARAM_FIELDS } from "./registry";

// Helper functions
export { getNodeSchema } from "./getNodeSchema";
export { getNodeParamFields } from "./getNodeParamFields";
export { getRequiredParams } from "./getRequiredParams";
export { getConditionallyRequired } from "./getConditionallyRequired";
export { getVisibleParams } from "./getVisibleParams";

// Static field info (replaces runtime Zod introspection)
export { getParamFieldInfo, NODE_PARAM_FIELD_INFO } from "../generated/paramFieldInfo";

// Types
export type {
  NodeSchema,
  NodeParam,
  ParamCondition,
  NodeParamField,
  NodeParamFields,
  NodeParamFieldInfo,
  NodeParamControl,
} from "./types";

// Enum constants (generated from catalog)
export {
  INPUT_MODES,
  OUTPUT_MODES,
  LOOP_MODES,
  GROUP_MODES,
  ERROR_STRATEGIES,
  IMAGE_FORMATS,
} from "../generated/enumConstants";

// Engine-backed Zod schemas and inferred types (auto-generated, all types)
export * from "../generated/schemas";
