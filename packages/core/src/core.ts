"use client";

/**
 * Core singleton — imperative, framework-agnostic API.
 *
 * Wires services into clients. This is the base layer that reactCore.ts
 * enhances with React hooks for the final public `core` export.
 *
 * Dependency flow:
 *   core.ts -> clients -> services -> adapters -> @bnto/backend
 */

import { createWorkflowService } from "./services/workflowService";
import { createExecutionService } from "./services/executionService";
import { createUserService } from "./services/userService";
import { createUploadService } from "./services/uploadService";
import { createDownloadService } from "./services/downloadService";
import { createBrowserExecutionService } from "./services/browserExecutionService";
import { createWorkflowClient } from "./clients/workflowClient";
import { createExecutionClient } from "./clients/executionClient";
import { createUserClient } from "./clients/userClient";
import { createUploadClient } from "./clients/uploadClient";
import { createDownloadClient } from "./clients/downloadClient";
import { createAuthClient } from "./clients/authClient";
import { createBrowserClient } from "./clients/browserClient";

// ── Services (single-domain, internal) ────────────────────────────────────
const workflowService = createWorkflowService();
const executionService = createExecutionService();
const userService = createUserService();
const uploadService = createUploadService();
const downloadService = createDownloadService();
const browserExecutionService = createBrowserExecutionService();

// ── Clients (cross-domain, public API) ────────────────────────────────────
const workflowClient = createWorkflowClient(workflowService, executionService);
const executionClient = createExecutionClient(executionService);
const userClient = createUserClient(userService);
const uploadClient = createUploadClient(uploadService);
const downloadClient = createDownloadClient(downloadService);
const authClient = createAuthClient();
const browserClient = createBrowserClient(browserExecutionService);

// ── Core Singleton ────────────────────────────────────────────────────────
export const core = {
  workflows: workflowClient,
  executions: executionClient,
  user: userClient,
  uploads: uploadClient,
  downloads: downloadClient,
  auth: authClient,
  browser: browserClient,
} as const;
