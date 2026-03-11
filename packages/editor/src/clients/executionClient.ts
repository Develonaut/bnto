/**
 * Execution client — thin passthrough to executionService.
 */

import type { ExecutionService, ExecutionClient } from "../editorTypes";

function createExecutionClient(executionService: ExecutionService): ExecutionClient {
  return { ...executionService };
}

export { createExecutionClient };
