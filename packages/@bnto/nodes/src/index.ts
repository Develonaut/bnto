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

// Recipe definition types
export type { Definition, Position, Metadata, Port, Edge, FieldsConfig } from "./definition";

// Execution types
export type { ExecutionContext, NodeResult } from "./execution";

// Recipe types
export type { Recipe, AcceptSpec, SEOSpec } from "./recipe";

// Node type registry
export { NODE_TYPES, NODE_TYPE_NAMES, NODE_TYPE_INFO } from "./nodeTypes";
export type { NodeTypeName, NodeCategory, NodeTypeInfo } from "./nodeTypes";
export { isNodeType } from "./isNodeType";
export { getNodeTypeInfo } from "./getNodeTypeInfo";
export { getNodeIcon } from "./getNodeIcon";
export { getNodeSublabel } from "./getNodeSublabel";
export { isIoNodeType } from "./isIoNodeType";
export { isContainerNodeType } from "./isContainerNodeType";

// Categories
export { CATEGORIES, getCategoryInfo } from "./categories";
export type { CategoryInfo } from "./categories";

// Predefined recipes (6 Tier 1 bntos)
export {
  RECIPES,
  getRecipeBySlug,
  cleanCsv,
  compressImages,
  convertImageFormat,
  renameCsvColumns,
  renameFiles,
  resizeImages,
} from "./recipes";

// Reusable sub-recipe building blocks
export {
  batchCompress,
  batchConvert,
  batchRename,
  batchResize,
  columnRenamer,
  csvCleaner,
} from "./recipes";

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
  NODE_SCHEMA_DEFS,
  getNodeSchema,
  getRequiredParams,
  getConditionallyRequired,
  getVisibleParams,
  HTTP_METHODS,
  FILE_OPERATIONS,
  INPUT_MODES,
  LOOP_MODES,
  OUTPUT_MODES,
  IMAGE_OPERATIONS,
  IMAGE_FORMATS,
  SPREADSHEET_OPERATIONS,
  SPREADSHEET_FORMATS,
  GROUP_MODES,
  ERROR_STRATEGIES,
} from "./schemas";
export type {
  NodeSchemaDefinition,
  NodeParamMeta,
  ParamCondition,
  FieldTypeInfo,
  FieldControl,
} from "./schemas";
export { inferFieldType } from "./schemas";

// I/O node helpers (Sprint 4C — self-describing recipes)
export { getInputNode } from "./getInputNode";
export { getOutputNode } from "./getOutputNode";
export { deriveAcceptSpec } from "./deriveAcceptSpec";

// Definition CRUD operations (Sprint 4 Wave 1 — editor foundation)
export { createBlankDefinition } from "./createBlankDefinition";
export { addNode } from "./addNode";
export { removeNode } from "./removeNode";
export { updateNodeParams } from "./updateNodeParams";
export { moveNode } from "./moveNode";
export { definitionToRecipe } from "./definitionToRecipe";
export type { RecipeMetadata } from "./definitionToRecipe";

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
} from "./generated/catalog";
export type { ProcessorDef, ProcessorParam, ParamType } from "./generated/catalog";
