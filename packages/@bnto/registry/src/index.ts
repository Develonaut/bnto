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
  PipelineSettings,
  IterationMode,
  ExecutionContext,
  NodeResult,
  NodeTypeName,
  NodeCategory,
  NodeTypeInfo,
  CategoryInfo,
  CategoryName,
  ProcessorDef,
  ProcessorParam,
  ParamType,
} from "./types";
export type { Recipe, AcceptSpec } from "./recipe";

// ── Curation functions ──────────────────────────────────────────────────
export { getAllRecipes } from "./getAllRecipes";
export { getRecipeBySlug } from "./getRecipeBySlug";
export { getRecipesByCategory } from "./getRecipesByCategory";
export { getAllNodeTypes } from "./getAllNodeTypes";
export { getBrowserNodeTypes } from "./getBrowserNodeTypes";
export { getBrowserRecipes } from "./getBrowserRecipes";
export { getAllCategories } from "./categories";
export { getAllProcessors } from "./processors";

// ── Definition CRUD ─────────────────────────────────────────────────────
export {
  createBlankDefinition,
  addNode,
  removeNode,
  updateNodeParams,
  moveNode,
  isValid,
} from "./definition";
export type { DefinitionResult } from "./definition";
export { definitionToRecipe } from "./definitionToRecipe";
export type { RecipeMetadata } from "./definitionToRecipe";

// ── Validation ──────────────────────────────────────────────────────────
export { validateDefinition, validateEdges, validateNodeParams } from "./validation";
export type { ValidationError } from "./validation";
export { validateRecipe } from "./validateRecipe";

// ── Classification ──────────────────────────────────────────────────────
export { isNodeType, isIoNodeType, isContainerNodeType } from "./classification";
export { isRecipeBrowserCapable } from "./isRecipeBrowserCapable";

// ── Node metadata ──────────────────────────────────────────────────────
export { getNodeTypeInfo, getNodeIcon, getNodeSublabel } from "./nodeMetadata";

// ── Category helpers ───────────────────────────────────────────────────
export { getCategoryInfo, deriveCategory } from "./categoryHelpers";

// ── I/O node helpers ───────────────────────────────────────────────────
export { getInputNode, getOutputNode } from "./ioNodes";

// ── Version checks ─────────────────────────────────────────────────────
export { isSupportedVersion, isCompatibleVersion } from "./versionCheck";
export { deriveAcceptSpec } from "./deriveAcceptSpec";

// ── Schema introspection ────────────────────────────────────────────────
export {
  NODE_SCHEMAS,
  NODE_PARAM_FIELDS,
  getNodeSchema,
  getNodeParamFields,
  getRequiredParams,
  getConditionallyRequired,
  getVisibleParams,
  getParamFieldInfo,
  NODE_PARAM_FIELD_INFO,
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
  ITERATION_MODES,
} from "./catalog";

// ── Predefined recipes ───────────────────────────────────────────────────
export { RECIPES } from "./recipesCatalog";
