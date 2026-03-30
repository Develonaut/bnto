"use client";

/**
 * React-enhanced core singleton.
 *
 * Merges imperative clients from core.ts with React hooks for the final
 * public API. Consumers import `core` from this module (via index.ts).
 *
 * 6 domains: executions, user, auth, telemetry, registry, flags.
 *
 * Usage:
 *   import { core } from "@bnto/core";
 *   const { isAuthenticated, user } = core.auth.useAuth();
 */

import { core as baseCore } from "./core";

// Execution hooks
import { useExecution } from "./hooks/useExecution";
import { useRunPredefined } from "./hooks/useRunPredefined";
import { useExecutionState } from "./hooks/useExecutionState";

// User hooks
import { useCurrentUser } from "./hooks/useCurrentUser";

// Auth hooks
import { useReady } from "./hooks/useReady";
import { useIsAuthenticated } from "./hooks/useIsAuthenticated";
import { useSessionStatus } from "./hooks/useSessionStatus";
import { useAuth } from "./hooks/useAuth";
import { useSignOut } from "./hooks/useSignOut";
import { useSignIn } from "@bnto/auth";
import { useSignUp } from "./hooks/useSignUp";

// Registry hooks
import { useRegistryRecipes } from "./hooks/useRegistryRecipes";
import { useRegistryNodeTypes } from "./hooks/useRegistryNodeTypes";
import { useRegistryCategories } from "./hooks/useRegistryCategories";
import { useRegistryProcessors } from "./hooks/useRegistryProcessors";

// Flags hooks
import { useFlag } from "./hooks/useFlag";
import { useVariant } from "./hooks/useVariant";
import { useFlagResult } from "./hooks/useFlagResult";

// Upload/Download hooks
import { useUploadFiles } from "./hooks/useUploadFiles";
import { useDownloadFiles } from "./hooks/useDownloadFiles";

export const core = {
  executions: {
    ...baseCore.executions,
    useExecution,
    useRunPredefined,
    useExecutionState,
  },

  user: {
    ...baseCore.user,
    useCurrentUser,
  },

  auth: {
    ...baseCore.auth,
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

  registry: {
    ...baseCore.registry,
    useRecipes: useRegistryRecipes,
    useNodeTypes: useRegistryNodeTypes,
    useCategories: useRegistryCategories,
    useProcessors: useRegistryProcessors,
  },

  flags: {
    ...baseCore.flags,
    useFlag,
    useVariant,
    useFlagResult,
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
