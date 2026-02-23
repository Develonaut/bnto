"use client";

// ── Core singleton (imperative + React hooks merged) ──────────────────────
export { core } from "./reactCore";

// ── Provider ──────────────────────────────────────────────────────────────
export { BntoCoreProvider } from "./provider";

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
  User,
  UploadFileInput,
  PresignedUploadUrl,
  UploadSession,
  FileUploadProgress,
  UploadProgress,
} from "./types";

// ── Session types ─────────────────────────────────────────────────────────
export type { AuthStatus } from "./providers/SessionContext";

// ── Auth types ────────────────────────────────────────────────────────────
export type { AuthUser, AuthState } from "./hooks/useAuth";

// ---------------------------------------------------------------------------
// Backwards-compatible hook re-exports.
//
// These allow existing consumers to continue importing hooks directly:
//   import { useWorkflows } from "@bnto/core";
//
// New code should use the namespaced pattern:
//   import { core } from "@bnto/core";
//   core.workflows.useWorkflows();
// ---------------------------------------------------------------------------
export { useWorkflows } from "./hooks/useWorkflows";
export { useWorkflow } from "./hooks/useWorkflow";
export { useSaveWorkflow } from "./hooks/useSaveWorkflow";
export { useRemoveWorkflow } from "./hooks/useRemoveWorkflow";
export { useRunWorkflow } from "./hooks/useRunWorkflow";
export { useExecution } from "./hooks/useExecution";
export { useExecutions } from "./hooks/useExecutions";
export { useExecutionLogs } from "./hooks/useExecutionLogs";
export { useRunsRemaining } from "./hooks/useRunsRemaining";
export { useCurrentUser } from "./hooks/useCurrentUser";
export { useReady } from "./hooks/useReady";
export { useIsAuthenticated } from "./hooks/useIsAuthenticated";
export { useSessionStatus } from "./hooks/useSessionStatus";
export { useSignOut } from "./hooks/useSignOut";
export { useAnonymousSession } from "./hooks/useAnonymousSession";
export { useRunQuota } from "./hooks/useRunQuota";
export { useUploadFiles } from "./hooks/useUploadFiles";
