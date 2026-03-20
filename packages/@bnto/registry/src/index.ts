/**
 * @bnto/registry — Curation and discovery layer for recipes and node types.
 *
 * Public facade for the entire node system. Re-exports everything from
 * @bnto/nodes that consumers need. @bnto/nodes is an internal package —
 * only @bnto/registry imports from it directly.
 *
 * No React, no Zustand — purely stateless functions and types.
 */

// ── Types ───────────────────────────────────────────────────────────────
export type {
  RegistryData,
  Definition,
  Position,
  Metadata,
  Port,
  Edge,
  FieldsConfig,
  ExecutionContext,
  NodeResult,
  Recipe,
  AcceptSpec,
  NodeTypeName,
  NodeCategory,
  NodeTypeInfo,
  CategoryInfo,
  ProcessorDef,
  ProcessorParam,
  ParamType,
} from "./types";

// ── Curation functions ──────────────────────────────────────────────────
export { getAllRecipes, getRecipeBySlug, getRecipesByCategory } from "./recipes";
export { getAllNodeTypes, getBrowserNodeTypes } from "./nodeTypes";
export { getAllCategories } from "./categories";
export { getAllProcessors } from "./processors";

// ── Definition CRUD ─────────────────────────────────────────────────────
export {
  createBlankDefinition,
  addNode,
  removeNode,
  updateNodeParams,
  moveNode,
  definitionToRecipe,
  isValid,
} from "./definition";
export type { DefinitionResult, RecipeMetadata } from "./definition";

// ── Validation ──────────────────────────────────────────────────────────
export {
  validateDefinition,
  validateEdges,
  validateRecipe,
  validateNodeParams,
} from "./validation";
export type { ValidationError } from "./validation";

// ── Classification & helpers ────────────────────────────────────────────
export {
  isNodeType,
  getNodeTypeInfo,
  getNodeIcon,
  getNodeSublabel,
  isIoNodeType,
  isContainerNodeType,
  getCategoryInfo,
  getInputNode,
  getOutputNode,
  deriveAcceptSpec,
  deriveCategory,
  isSupportedVersion,
  isCompatibleVersion,
} from "./helpers";

// ── Schema introspection ────────────────────────────────────────────────
export {
  NODE_SCHEMAS,
  NODE_PARAM_FIELDS,
  getNodeSchema,
  getNodeParamFields,
  getRequiredParams,
  getConditionallyRequired,
  getVisibleParams,
  inferFieldType,
  getProcessorDefaults,
  getParamConstraints,
  getProcessorAccepts,
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

// ── Catalog constants ───────────────────────────────────────────────────
export {
  NODE_TYPES,
  NODE_TYPE_NAMES,
  NODE_TYPE_INFO,
  CATEGORIES,
  PROCESSORS,
  PROCESSOR_MAP,
  DEFINITION_JSON_SCHEMA,
  CURRENT_FORMAT_VERSION,
  SUPPORTED_FORMAT_VERSIONS,
  INPUT_MODES,
  LOOP_MODES,
  OUTPUT_MODES,
  IMAGE_FORMATS,
  GROUP_MODES,
  ERROR_STRATEGIES,
} from "./catalog";

// ── Predefined recipes (individual named exports) ───────────────────────
export {
  RECIPES,
  cleanCsv,
  compressImages,
  convertImageFormat,
  renameCsvColumns,
  renameFiles,
  resizeImages,
} from "@bnto/nodes";
