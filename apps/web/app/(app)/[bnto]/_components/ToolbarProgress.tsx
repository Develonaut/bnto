"use client";

import type { BrowserExecution } from "@bnto/core";
import { computeTotalSaved } from "@bnto/core";
import {
  CheckCircle2Icon,
  LoaderIcon,
  StatusBanner,
  StatusBannerIcon,
  StatusBannerLabel,
  StatusBannerProgress,
  StatusBannerRow,
  StatusBannerSpacer,
  formatFileSize,
} from "@bnto/ui";

interface ToolbarProgressProps {
  execution: BrowserExecution;
}

/**
 * Persistent progress/status banner for the recipe toolbar.
 *
 * One StatusBanner stays mounted — props change across phases, no layout shift.
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
        <StatusBanner variant="success">
          <StatusBannerRow>
            <StatusBannerIcon>
              <CheckCircle2Icon />
            </StatusBannerIcon>
            <StatusBannerLabel>
              {count} {count === 1 ? "file" : "files"} processed
            </StatusBannerLabel>
            <StatusBannerSpacer />
            {saved > 0 && (
              <StatusBannerLabel muted mono>
                {formatFileSize(saved)} saved
              </StatusBannerLabel>
            )}
          </StatusBannerRow>
          <StatusBannerProgress value={100} variant="success" />
        </StatusBanner>
      </div>
    );
  }

  const { fileProgress } = execution;
  const percent = fileProgress?.overallPercent ?? 0;
  const label = fileProgress
    ? `Processing file ${fileProgress.fileIndex + 1} of ${fileProgress.totalFiles}...`
    : "Initializing...";

  return (
    <div
      data-testid="toolbar-progress"
      data-status="processing"
      data-file-index={fileProgress?.fileIndex}
      data-total-files={fileProgress?.totalFiles}
      data-overall-percent={percent}
    >
      <StatusBanner>
        <StatusBannerRow>
          <StatusBannerIcon>
            <LoaderIcon className="motion-safe:animate-spin" />
          </StatusBannerIcon>
          <StatusBannerLabel muted>{label}</StatusBannerLabel>
        </StatusBannerRow>
        <StatusBannerProgress value={percent} />
      </StatusBanner>
    </div>
  );
}
