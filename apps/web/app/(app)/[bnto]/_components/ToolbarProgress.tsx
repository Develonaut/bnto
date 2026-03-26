"use client";

import type { BrowserExecution } from "@bnto/core";
import { computeTotalSaved } from "@bnto/core";
import { CompletedToolbarBanner } from "./CompletedToolbarBanner";
import { ProcessingToolbarBanner } from "./ProcessingToolbarBanner";

interface ToolbarProgressProps {
  execution: BrowserExecution;
}

/**
 * Persistent progress/status banner for the recipe toolbar.
 *
 * One StatusBanner stays mounted -- props change across phases, no layout shift.
 */
export function ToolbarProgress({ execution }: ToolbarProgressProps) {
  if (execution.status === "completed") {
    const saved = computeTotalSaved(execution.results);
    const count = execution.results.length;

    return (
      <div
        data-testid="toolbar-progress"
        data-status="completed"
        data-total-saved={saved}
        data-files-count={count}
      >
        <CompletedToolbarBanner count={count} saved={saved} />
      </div>
    );
  }

  return (
    <div
      data-testid="toolbar-progress"
      data-status="processing"
      data-file-index={execution.fileProgress?.fileIndex}
      data-total-files={execution.fileProgress?.totalFiles}
      data-overall-percent={execution.fileProgress?.overallPercent ?? 0}
    >
      <ProcessingToolbarBanner fileProgress={execution.fileProgress} />
    </div>
  );
}
