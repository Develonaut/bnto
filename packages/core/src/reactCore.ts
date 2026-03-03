"use client";

/**
 * React-enhanced core singleton.
 *
 * Merges imperative clients from core.ts with React hooks for the final
 * public API. Consumers import `core` from this module (via index.ts).
 *
 * 5 domains: recipes, executions, user, auth, telemetry.
 *
 * Usage:
 *   import { core } from "@bnto/core";
 *   const { isAuthenticated, user } = core.auth.useAuth();
 */

import { core as baseCore } from "./core";

// Recipe hooks
import { useRecipes } from "./hooks/useRecipes";
import { useRecipe } from "./hooks/useRecipe";
import { useSaveRecipe } from "./hooks/useSaveRecipe";
import { useRemoveRecipe } from "./hooks/useRemoveRecipe";
import { useRunRecipe } from "./hooks/useRunRecipe";

// Execution hooks
import { useExecution } from "./hooks/useExecution";
import { useExecutions } from "./hooks/useExecutions";
import { useExecutionHistory } from "./hooks/useExecutionHistory";
import { useExecutionLogs } from "./hooks/useExecutionLogs";
import { useRunPredefined } from "./hooks/useRunPredefined";
import { useExecutionState } from "./hooks/useExecutionState";

// User hooks
import { useCurrentUser } from "./hooks/useCurrentUser";

// User hooks (analytics)
import { useUsageAnalytics } from "./hooks/useUsageAnalytics";
import { useSlugAggregates } from "./hooks/useSlugAggregates";

// Auth hooks
import { useReady } from "./hooks/useReady";
import { useIsAuthenticated } from "./hooks/useIsAuthenticated";
import { useSessionStatus } from "./hooks/useSessionStatus";
import { useAuth } from "./hooks/useAuth";
import { useSignOut } from "./hooks/useSignOut";
import { useSignIn } from "@bnto/auth";
import { useSignUp } from "./hooks/useSignUp";

// Upload/Download hooks
import { useUploadFiles } from "./hooks/useUploadFiles";
import { useDownloadFiles } from "./hooks/useDownloadFiles";

export const core = {
  recipes: {
    ...baseCore.recipes,
    useRecipes,
    useRecipe,
    useSaveRecipe,
    useRemoveRecipe,
    useRunRecipe,
  },

  executions: {
    ...baseCore.executions,
    useExecution,
    useExecutions,
    useExecutionHistory,
    useExecutionLogs,
    useRunPredefined,
    useExecutionState,
  },

  user: {
    ...baseCore.user,
    useCurrentUser,
    useUsageAnalytics,
    useSlugAggregates,
  },

  auth: {
    useReady,
    useIsAuthenticated,
    useSessionStatus,
    useAuth,
    useSignIn,
    useSignUp,
    useSignOut,
  },

  telemetry: {
    ...baseCore.telemetry,
  },

  // ── Internal (not public API, kept for consumer migration) ─────────
  /** @internal Cloud uploads — will be absorbed into executions for M4. */
  uploads: {
    useUploadFiles,
    generateUrls: baseCore.uploads.generateUrls,
    uploadFiles: baseCore.uploads.uploadFiles,
  },
  /** @internal Cloud downloads — will be absorbed into executions for M4. */
  downloads: {
    useDownloadFiles,
    getDownloadUrls: baseCore.downloads.getDownloadUrls,
    downloadFile: baseCore.downloads.downloadFile,
  },
} as const;
