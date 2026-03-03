"use client";

/**
 * Core singleton — imperative, framework-agnostic API.
 *
 * Wires services into clients. This is the base layer that reactCore.ts
 * enhances with React hooks for the final public `core` export.
 *
 * 5 domains: recipes, executions, user, auth, telemetry.
 *
 * Dependency flow:
 *   core.ts -> clients -> services -> adapters -> @bnto/backend
 */

import { createRecipeService } from "./services/recipeService";
import { createExecutionService } from "./services/executionService";
import { createUserService } from "./services/userService";
import { createAnalyticsService } from "./services/analyticsService";
import { createBrowserExecutionService } from "./services/browserExecutionService";
import { createRecipeClient } from "./clients/recipeClient";
import { createExecutionClient } from "./clients/executionClient";
import { createUserClient } from "./clients/userClient";
import { createAuthClient } from "./clients/authClient";
import { createTelemetryClient } from "./clients/telemetryClient";

// Services still created internally — uploads, downloads, analytics stay
// as services for when M4 cloud execution activates. They're just not
// exposed as top-level domains on the singleton.
import { createUploadService } from "./services/uploadService";
import { createDownloadService } from "./services/downloadService";

// ── Services (single-domain, internal) ────────────────────────────────────
const recipeService = createRecipeService();
const executionService = createExecutionService();
const userService = createUserService();
const analyticsService = createAnalyticsService();
const browserExecutionService = createBrowserExecutionService();

// Cloud execution infrastructure — used internally by hooks, not exposed
// as top-level domains on the public API (reactCore.ts).
const uploadService = createUploadService();
const downloadService = createDownloadService();

// ── Clients (cross-domain, public API) ────────────────────────────────────
const recipeClient = createRecipeClient(recipeService, executionService);
const executionClient = createExecutionClient(executionService, browserExecutionService);
const userClient = createUserClient(userService, analyticsService);
const authClient = createAuthClient();
const telemetryClient = createTelemetryClient();

// ── Core Singleton ────────────────────────────────────────────────────────
export const core = {
  recipes: recipeClient,
  executions: executionClient,
  user: userClient,
  auth: authClient,
  telemetry: telemetryClient,

  // ── Internal (used by hooks, not top-level public domains) ──────────
  /** @internal Cloud upload service — will be absorbed into executions for M4. */
  uploads: uploadService,
  /** @internal Cloud download service — will be absorbed into executions for M4. */
  downloads: downloadService,
} as const;
