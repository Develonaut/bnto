/**
 * Wails adapter stub (Phase 3).
 *
 * When Wails desktop support is implemented, this module will provide
 * the same adapter interface as the Convex adapters, routing through
 * Wails Go bindings instead of Convex cloud.
 */

const NOT_IMPLEMENTED = "Wails adapter not implemented (Phase 3)";

function notImplemented(): never {
  throw new Error(NOT_IMPLEMENTED);
}

// Workflow adapters
export function getWorkflowsQuery(): never { notImplemented(); }
export function getWorkflowQuery(): never { notImplemented(); }
export function saveWorkflow(): never { notImplemented(); }
export function removeWorkflow(): never { notImplemented(); }

// Execution adapters
export function getExecutionQuery(): never { notImplemented(); }
export function getExecutionsQuery(): never { notImplemented(); }
export function getExecutionLogsQuery(): never { notImplemented(); }
export function startExecution(): never { notImplemented(); }

// User adapters
export function getCurrentUserQuery(): never { notImplemented(); }
export function getRunsRemainingQuery(): never { notImplemented(); }
