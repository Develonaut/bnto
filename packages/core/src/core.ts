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
import { createWasmExecutionService } from "./services/wasmExecutionService";
import { createWorkflowClient } from "./clients/workflowClient";
import { createExecutionClient } from "./clients/executionClient";
import { createUserClient } from "./clients/userClient";
import { createUploadClient } from "./clients/uploadClient";
import { createDownloadClient } from "./clients/downloadClient";
import { createAuthClient } from "./clients/authClient";
import { createWasmClient } from "./clients/wasmClient";
import { createRecipeClient } from "./clients/recipeClient";

// ── Services (single-domain, internal) ────────────────────────────────────
const workflowService = createWorkflowService();
const executionService = createExecutionService();
const userService = createUserService();
const uploadService = createUploadService();
const downloadService = createDownloadService();
const wasmExecutionService = createWasmExecutionService();

// ── Clients (cross-domain, public API) ────────────────────────────────────
const workflowClient = createWorkflowClient(workflowService, executionService);
const executionClient = createExecutionClient(executionService);
const userClient = createUserClient(userService);
const uploadClient = createUploadClient(uploadService);
const downloadClient = createDownloadClient(downloadService);
const authClient = createAuthClient();
const wasmClient = createWasmClient(wasmExecutionService);
const recipeClient = createRecipeClient();

// ── Core Singleton ────────────────────────────────────────────────────────
export const core = {
  workflows: workflowClient,
  executions: executionClient,
  user: userClient,
  uploads: uploadClient,
  downloads: downloadClient,
  auth: authClient,
  wasm: wasmClient,
  recipe: recipeClient,
} as const;
