"use client";

// Hooks — transport-agnostic data access for UI components
export {
  useWorkflows,
  useWorkflow,
  useSaveWorkflow,
  useRemoveWorkflow,
  useRunWorkflow,
  useExecution,
  useExecutions,
  useExecutionLogs,
  useRunsRemaining,
  useCurrentUser,
} from "./hooks";

// Session hooks — auth state and provider readiness
export {
  useReady,
  useIsAuthenticated,
  useSessionStatus,
} from "./hooks";

// Auth action hooks
export { useSignOut } from "./hooks";

// Anonymous session
export { useAnonymousSession } from "./hooks";

// Types — workflow definitions, API responses, Convex document types
export type {
  BntoAPI,
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
  NodeProgress,
  RunResult,
  WorkflowId,
  ExecutionId,
  UserId,
  WorkflowDoc,
  ExecutionDoc,
  ExecutionLogDoc,
  UserDoc,
  WorkflowListItem,
} from "./types";

// Session types
export type { AuthStatus } from "./providers/SessionContext";

// Provider — re-export for convenience (also available via @bnto/core/provider)
export { BntoCoreProvider } from "./provider";
