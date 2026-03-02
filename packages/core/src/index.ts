"use client";

// ── Core singleton (imperative + React hooks merged) ──────────────────────
export { core } from "./reactCore";

// ── Providers ─────────────────────────────────────────────────────────────
export { BntoCoreProvider } from "./BntoCoreProvider";
export { TelemetryProvider } from "./providers/TelemetryProvider";

// ── Types (transport-agnostic) ────────────────────────────────────────────
export type {
  Workflow,
  WorkflowListItem,
  SaveWorkflowInput,
  WorkflowDefinition,
  Position,
  Metadata,
  Port,
  Edge,
  FieldsConfig,
  WorkflowSummary,
  ValidationResult,
  RunResponse,
  Execution,
  ExecutionLog,
  NodeProgress,
  RunResult,
  OutputFile,
  StartExecutionInput,
  StartPredefinedInput,
  User,
  UsageAnalytics,
  SlugAggregate,
  ServerQuota,
  UploadFileInput,
  PresignedUploadUrl,
  UploadSession,
  FileUploadProgress,
  UploadProgress,
  OutputFileUrl,
  DownloadUrlsResult,
  BrowserEngine,
  BrowserFileResult,
  BrowserExecution,
  BrowserFileProgress,
  WasmRunResult,
  RunPhase,
  RecipeFlow,
  TelemetryConfig,
  TelemetryProperties,
  TelemetryUserTraits,
} from "./types";

// ── Auth types ────────────────────────────────────────────────────────────
export type { AuthUser, AuthState, AuthStatus } from "./types/auth";

// ── Store types (for consumer selectors) ──────────────────────────────────
export type { RecipeFlowState } from "./stores/recipeFlowStore";
export type { WasmExecutionState } from "./stores/wasmExecutionStore";

// ── Service types (for instance consumers) ────────────────────────────────
export type { WasmExecutionInstance } from "./services/wasmExecutionService";
