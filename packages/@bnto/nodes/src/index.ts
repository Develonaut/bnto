/**
 * @bnto/nodes — Engine-agnostic node definitions, types, and metadata.
 *
 * This package is consumed by every execution target:
 * - Rust WASM engine (browser — primary)
 * - Web app config UI / editor
 * - Desktop (Tauri, planned)
 *
 * The Rust engine's self-describing catalog (`engine/catalog.snapshot.json`)
 * is the source of truth for processor metadata. This package provides
 * the TypeScript types, Zod schemas, and UI metadata that the engine
 * catalog validates against via `catalogValidation.test.ts`.
 *
 * Zero runtime dependencies except Zod (for schema validation).
 */

// Definition types
export type {
  Definition,
  Position,
  Metadata,
  Port,
  Edge,
  FieldsConfig,
  IterationMode,
  PipelineSettings,
  InputCardinality,
} from "./definition";

// Execution types
export type { ExecutionContext, NodeResult } from "./execution";

// Node type registry (generated from engine catalog)
export { NODE_TYPES, NODE_TYPE_NAMES, NODE_TYPE_INFO } from "./generated/catalog";
export type { NodeTypeName, NodeCategory, NodeTypeInfo } from "./generated/catalog";
export { isNodeType } from "./isNodeType";
export { getNodeTypeInfo } from "./getNodeTypeInfo";
export { getNodeIcon } from "./getNodeIcon";
export { getNodeSublabel } from "./getNodeSublabel";
export { isIoNodeType } from "./isIoNodeType";
export { isContainerNodeType } from "./isContainerNodeType";

// Categories
export { CATEGORIES, getCategoryInfo } from "./categories";
export type { CategoryInfo } from "./categories";

// Format versioning
export {
  CURRENT_FORMAT_VERSION,
  SUPPORTED_FORMAT_VERSIONS,
  isSupportedVersion,
  isCompatibleVersion,
} from "./formatVersion";

// Definition validation
export { validateDefinition, validateEdges } from "./validate";
export type { ValidationError } from "./validate";

// Node parameter validation (Zod-based)
export { validateNodeParams } from "./validateNodeParams";

// Parameter schemas — Zod-based registry + helpers
export {
  NODE_SCHEMAS,
  NODE_PARAM_FIELDS,
  getNodeSchema,
  getNodeParamFields,
  getRequiredParams,
  getConditionallyRequired,
  getVisibleParams,
  INPUT_MODES,
  LOOP_MODES,
  OUTPUT_MODES,
  IMAGE_FORMATS,
  GROUP_MODES,
  ERROR_STRATEGIES,
} from "./schemas";
export type {
  NodeSchema,
  NodeParam,
  ParamCondition,
  NodeParamFieldInfo,
  NodeParamControl,
  NodeParamField,
  NodeParamFields,
} from "./schemas";
export { getParamFieldInfo, NODE_PARAM_FIELD_INFO } from "./schemas";

// I/O node helpers
export { getInputNode } from "./getInputNode";
export { getOutputNode } from "./getOutputNode";
export { deriveCategory } from "./deriveCategory";

// Definition CRUD operations
export { createBlankDefinition } from "./createBlankDefinition";
export { addNode } from "./addNode";
export { removeNode } from "./removeNode";
export { updateNodeParams } from "./updateNodeParams";
export { moveNode } from "./moveNode";

// Definition result type (mutation return shape)
export { isValid } from "./definitionResult";
export type { DefinitionResult } from "./definitionResult";

// Engine catalog (generated from engine/catalog.snapshot.json)
export {
  PROCESSORS,
  PROCESSOR_MAP,
  getProcessorDefaults,
  getParamConstraints,
  getProcessorAccepts,
  ITERATION_MODES,
} from "./generated/catalog";
export type {
  ProcessorDef,
  ProcessorParam,
  ParamType,
  IterationModeValue,
} from "./generated/catalog";

// Definition JSON Schema (generated from engine — validates .bnto.json files)
export { DEFINITION_JSON_SCHEMA } from "./generated/definitionSchema";

// Generated recipes (from engine built-in recipe catalog)
export { GENERATED_RECIPES } from "./generated/recipes";
export type { GeneratedRecipe } from "./generated/recipes";
