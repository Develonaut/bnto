"use client";

const NOT_IMPLEMENTED = "Wails adapter not implemented (Phase 3)";

function notImplemented(): never {
  throw new Error(NOT_IMPLEMENTED);
}

// ---------------------------------------------------------------------------
// Query option factories — same shape as convex.ts
// ---------------------------------------------------------------------------

export function workflowsQueryOptions(): never {
  notImplemented();
}

export function workflowQueryOptions(): never {
  notImplemented();
}

export function executionQueryOptions(): never {
  notImplemented();
}

export function executionsQueryOptions(): never {
  notImplemented();
}

export function executionLogsQueryOptions(): never {
  notImplemented();
}

export function runsRemainingQueryOptions(): never {
  notImplemented();
}

export function currentUserQueryOptions(): never {
  notImplemented();
}

// ---------------------------------------------------------------------------
// Mutation hook factories — same shape as convex.ts
// ---------------------------------------------------------------------------

export function useSaveWorkflowMutation(): never {
  notImplemented();
}

export function useRemoveWorkflowMutation(): never {
  notImplemented();
}

export function useStartExecutionMutation(): never {
  notImplemented();
}
