"use client";

import type { ReactNode } from "react";
import type { BrowserExecution } from "@bnto/core";
import { computeTotalSaved } from "@bnto/core";
import { CompletedToolbarBanner } from "./CompletedToolbarBanner";
import { FailedToolbarBanner } from "./FailedToolbarBanner";
import { IdleToolbarBanner } from "./IdleToolbarBanner";
import { ProcessingToolbarBanner } from "./ProcessingToolbarBanner";

interface ToolbarProgressProps {
  execution: BrowserExecution;
}

function ProgressWrapper({
  status,
  children,
  ...rest
}: Record<string, unknown> & { status: string; children: ReactNode }) {
  return (
    <div data-testid="toolbar-progress" data-status={status} {...rest}>
      {children}
    </div>
  );
}

/** Resolve the banner component for the current execution status. */
function resolveBanner(execution: BrowserExecution) {
  if (execution.status === "failed") {
    return {
      status: "failed",
      banner: <FailedToolbarBanner error={execution.error ?? "Execution failed"} />,
    };
  }
  if (execution.status === "completed") {
    const saved = computeTotalSaved(execution.results);
    const count = execution.results.length;
    return {
      status: "completed",
      banner: <CompletedToolbarBanner count={count} saved={saved} />,
      attrs: { "data-total-saved": saved, "data-files-count": count },
    };
  }
  if (execution.status === "processing") {
    return {
      status: "processing",
      banner: <ProcessingToolbarBanner fileProgress={execution.fileProgress} />,
      attrs: {
        "data-file-index": execution.fileProgress?.fileIndex,
        "data-total-files": execution.fileProgress?.totalFiles,
        "data-overall-percent": execution.fileProgress?.overallPercent ?? 0,
      },
    };
  }
  return { status: "idle", banner: <IdleToolbarBanner /> };
}

/**
 * Persistent progress/status banner for the recipe toolbar.
 *
 * Always renders a StatusBanner — reserves consistent toolbar height across phases.
 */
export function ToolbarProgress({ execution }: ToolbarProgressProps) {
  const { status, banner, attrs } = resolveBanner(execution);
  return (
    <ProgressWrapper status={status} {...attrs}>
      {banner}
    </ProgressWrapper>
  );
}
