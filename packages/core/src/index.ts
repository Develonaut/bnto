"use client";

// ── Core singleton (imperative + React hooks merged) ──────────────────────
export { core } from "./reactCore";

// ── Provider ──────────────────────────────────────────────────────────────
export { BntoCoreProvider } from "./BntoCoreProvider";

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
} from "./types";

// ── Auth types ────────────────────────────────────────────────────────────
export type { AuthUser, AuthState, AuthStatus } from "./types/auth";
