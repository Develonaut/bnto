"use client";

/**
 * React-enhanced core singleton.
 *
 * Merges imperative clients from core.ts with React hooks for the final
 * public API. Consumers import `core` from this module (via index.ts).
 *
 * Usage:
 *   import { core } from "@bnto/core";
 *   const { isAuthenticated, user } = core.auth.useAuth();
 *   core.auth.signOut();
 */

import { core as baseCore } from "./core";

// Workflow hooks
import { useWorkflows } from "./hooks/useWorkflows";
import { useWorkflow } from "./hooks/useWorkflow";
import { useSaveWorkflow } from "./hooks/useSaveWorkflow";
import { useRemoveWorkflow } from "./hooks/useRemoveWorkflow";
import { useRunWorkflow } from "./hooks/useRunWorkflow";

// Execution hooks
import { useExecution } from "./hooks/useExecution";
import { useExecutions } from "./hooks/useExecutions";
import { useExecutionLogs } from "./hooks/useExecutionLogs";

// User hooks
import { useCurrentUser } from "./hooks/useCurrentUser";
import { useRunsRemaining } from "./hooks/useRunsRemaining";

// Session hooks (context-based, no service/adapter)
import { useReady } from "./hooks/useReady";
import { useIsAuthenticated } from "./hooks/useIsAuthenticated";
import { useSessionStatus } from "./hooks/useSessionStatus";

// Auth hooks
import { useAuth } from "./hooks/useAuth";
import { useSignOut } from "./hooks/useSignOut";
import { useAnonymousSession } from "./hooks/useAnonymousSession";

// Auth re-exports from @bnto/auth (sign-in / sign-up methods)
import { useSignIn, useSignUp } from "@bnto/auth";

export const core = {
  workflows: {
    ...baseCore.workflows,
    useWorkflows,
    useWorkflow,
    useSaveWorkflow,
    useRemoveWorkflow,
    useRunWorkflow,
  },

  executions: {
    ...baseCore.executions,
    useExecution,
    useExecutions,
    useExecutionLogs,
  },

  user: {
    ...baseCore.user,
    useCurrentUser,
    useRunsRemaining,
  },

  session: {
    useReady,
    useIsAuthenticated,
    useSessionStatus,
  },

  auth: {
    ...baseCore.auth,
    useAuth,
    useSignIn,
    useSignUp,
    useSignOut,
    useAnonymousSession,
  },
} as const;
