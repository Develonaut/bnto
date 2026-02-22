"use client";

/**
 * Core singleton — imperative, framework-agnostic API.
 *
 * Wires services into clients. This is the base layer that reactCore.ts
 * enhances with React hooks for the final public `core` export.
 *
 * Dependency flow:
 *   core.ts → clients → services → adapters → @bnto/backend
 */

import { createWorkflowService } from "./services/workflowService";
import { createExecutionService } from "./services/executionService";
import { createUserService } from "./services/userService";
import { createWorkflowClient } from "./clients/workflowClient";
import { createExecutionClient } from "./clients/executionClient";
import { createUserClient } from "./clients/userClient";
import { createAuthClient } from "./clients/authClient";

// ── Services (single-domain, internal) ────────────────────────────────────
const workflowService = createWorkflowService();
const executionService = createExecutionService();
const userService = createUserService();

// ── Clients (cross-domain, public API) ────────────────────────────────────
const workflowClient = createWorkflowClient(workflowService, executionService);
const executionClient = createExecutionClient(executionService);
const userClient = createUserClient(userService);
const authClient = createAuthClient();

// ── Core Singleton ────────────────────────────────────────────────────────
export const core = {
  workflows: workflowClient,
  executions: executionClient,
  user: userClient,
  auth: authClient,
} as const;
