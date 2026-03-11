/**
 * History client — thin passthrough to historyService.
 */

import type { HistoryService, HistoryClient } from "../editorTypes";

function createHistoryClient(historyService: HistoryService): HistoryClient {
  return { ...historyService };
}

export { createHistoryClient };
