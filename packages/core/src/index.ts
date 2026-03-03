"use client";

// ── Core singleton (imperative + React hooks merged) ──────────────────────
export { core } from "./reactCore";

// ── Providers ─────────────────────────────────────────────────────────────
export { BntoCoreProvider } from "./BntoCoreProvider";
export { TelemetryProvider } from "./providers/TelemetryProvider";

// ── Types (transport-agnostic) ────────────────────────────────────────────
export type {
  Recipe,
  RecipeListItem,
  SaveRecipeInput,
  RecipeDefinition,
  Position,
  Metadata,
  Port,
  Edge,
  FieldsConfig,
  RecipeSummary,
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
  BrowserRunResult,
  LocalHistoryEntry,
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
export type { ExecutionInstanceState } from "./stores/executionInstanceStore";

// ── Service types (for instance consumers) ────────────────────────────────
export type { ExecutionInstance } from "./services/executionInstance";
export type { ExecutionState } from "./hooks/useExecutionState";

// ── Store factories (for app-layer orchestration) ─────────────────────────
export { createRecipeFlowStore } from "./stores/recipeFlowStore";
