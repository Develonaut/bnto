"use client";

// ── Core singleton (imperative + React hooks merged) ──────────────────────
export { core } from "./reactCore";

// ── Providers ─────────────────────────────────────────────────────────────
export { BntoCoreProvider } from "./BntoCoreProvider";
export { TelemetryProvider } from "./providers/TelemetryProvider";

// ── Types (transport-agnostic) ────────────────────────────────────────────
export type {
  Execution,
  ExecutionLog,
  NodeProgress,
  RunResult,
  OutputFile,
  StartExecutionInput,
  StartPredefinedInput,
  User,
  UploadFileInput,
  PresignedUploadUrl,
  UploadSession,
  FileUploadProgress,
  UploadProgress,
  OutputFileUrl,
  DownloadUrlsResult,
  BrowserFileResult,
  BrowserExecution,
  BrowserFileProgress,
  BrowserRunResult,
  ProcessorMetadata,
  LocalHistoryEntry,
  TelemetryConfig,
  TelemetryProperties,
  TelemetryUserTraits,
  FlagResult,
  FlagSubscriber,
} from "./types";

// ── Auth types ────────────────────────────────────────────────────────────
export type { AuthUser, AuthState, AuthStatus } from "./types/auth";

// ── Store types (for consumer selectors) ──────────────────────────────────
export type { ExecutionInstanceState } from "./stores/executionInstanceStore";

// ── Service types (for instance consumers) ────────────────────────────────
export type { ExecutionInstance } from "./services/executionInstance";
export type { ExecutionState } from "./hooks/useExecutionState";
export { useFileResultProps } from "./hooks/useFileResultProps";
export type { FileResultDisplayProps } from "./hooks/useFileResultProps";

// ── Utils (pure functions) ────────────────────────────────────────────────
export { computeTotalSaved } from "./utils/computeTotalSaved";
export { deriveFileResultProps } from "./utils/deriveFileResultProps";
export type { FileResultDisplay } from "./utils/deriveFileResultProps";

// ── Store factories (for app-layer orchestration) ─────────────────────────
export { createEnhancedStore } from "./stores/createEnhancedStore";

// ── Store utilities (for app-layer selectors) ────────────────────────────
export { useShallow } from "zustand/react/shallow";

// ── Pipeline types (execution-oriented definition for WASM) ───────────────
export type { PipelineDefinition, PipelineNode } from "./types/pipeline";
export { definitionToPipeline } from "./transforms/definitionToPipeline";
export { applyConfigToDefinition } from "./transforms/applyConfigToDefinition";

// ── Pipeline events (structured progress from Rust executor) ─────────────
export type {
  PipelineEvent,
  PipelineStartedEvent,
  NodeStartedEvent,
  FileProgressEvent,
  NodeCompletedEvent,
  NodeFailedEvent,
  PipelineCompletedEvent,
  PipelineFailedEvent,
} from "./types/pipelineEvents";

// ── Browser utilities (for app-layer use) ─────────────────────────────────
export { downloadBlob } from "./adapters/browser/downloadBlob";
export { deriveAcceptedTypes } from "./adapters/browser/deriveAcceptedTypes";
export { deriveOutputConfig } from "./adapters/browser/deriveOutputConfig";
export { createZipBlob } from "./adapters/browser/createZipBlob";

// ── Strings (re-exported from @bnto/i18n for consumers) ──────────────────
export { t, useT } from "@bnto/i18n";
export type { StringKey } from "@bnto/i18n";

// ── Node system (re-exported from @bnto/registry for consumers) ──────────

// Types
export type { RegistryData, Recipe } from "@bnto/registry";
export type {
  Definition,
  Position,
  Metadata,
  Port,
  Edge,
  FieldsConfig,
  PipelineSettings,
  IterationMode,
  NodeTypeName,
  NodeCategory,
  NodeTypeInfo,
  CategoryInfo,
  AcceptSpec,
  ProcessorDef,
  ProcessorParam,
  ParamType,
  NodeSchema,
  NodeParam,
  ParamCondition,
  NodeParamFieldInfo,
  NodeParamControl,
  NodeParamField,
  NodeParamFields,
  DefinitionResult,
  ValidationError,
} from "@bnto/registry";
export type { RecipeMetadata as NodeRecipeMetadata } from "@bnto/registry";

// Catalog constants
export {
  NODE_TYPE_INFO,
  NODE_TYPE_NAMES,
  NODE_SCHEMAS,
  NODE_PARAM_FIELDS,
  CURRENT_FORMAT_VERSION,
  CATEGORIES,
  PROCESSORS,
  RECIPES,
  ITERATION_MODES,
} from "@bnto/registry";

// Classification & helpers
export {
  isIoNodeType,
  isContainerNodeType,
  isNodeType,
  getNodeIcon,
  getNodeSublabel,
  getNodeTypeInfo,
} from "@bnto/registry";

// Definition CRUD
export {
  createBlankDefinition,
  addNode,
  removeNode,
  isValid,
  definitionToRecipe,
} from "@bnto/registry";

// Validation
export { validateDefinition } from "@bnto/registry";

// Schema introspection
export {
  inferFieldType,
  getVisibleParams,
  getRequiredParams,
  getConditionallyRequired,
  getNodeSchema,
  getNodeParamFields,
  getProcessorDefaults,
} from "@bnto/registry";

// Recipe lookup
export { getRecipeBySlug, getAllRecipes } from "@bnto/registry";
