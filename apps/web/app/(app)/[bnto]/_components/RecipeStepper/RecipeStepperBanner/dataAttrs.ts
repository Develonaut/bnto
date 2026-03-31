import type { BrowserExecution } from "@bnto/core";
import { computeTotalSaved } from "@bnto/core";

/** Data attributes for E2E test selectors on the progress wrapper. */
export function dataAttrs(execution: BrowserExecution): Record<string, unknown> {
  if (execution.status === "completed") {
    const saved = computeTotalSaved(execution.results);
    return { "data-total-saved": saved, "data-files-count": execution.results.length };
  }
  if (execution.status === "processing") {
    return {
      "data-file-index": execution.fileProgress?.fileIndex,
      "data-total-files": execution.fileProgress?.totalFiles,
      "data-overall-percent": execution.fileProgress?.overallPercent ?? 0,
    };
  }
  return {};
}
