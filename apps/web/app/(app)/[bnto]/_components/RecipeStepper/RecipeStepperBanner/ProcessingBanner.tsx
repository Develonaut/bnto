"use client";

import type { BrowserExecution } from "@bnto/core";
import {
  LoaderIcon,
  StatusBanner,
  StatusBannerIcon,
  StatusBannerLabel,
  StatusBannerProgress,
  StatusBannerRow,
} from "@bnto/ui";

export function ProcessingBanner({ execution }: { execution: BrowserExecution }) {
  const percent = execution.fileProgress?.overallPercent ?? 0;
  const label = execution.fileProgress
    ? `Processing file ${execution.fileProgress.fileIndex + 1} of ${execution.fileProgress.totalFiles}...`
    : "Initializing...";
  return (
    <StatusBanner>
      <StatusBannerRow>
        <StatusBannerIcon>
          <LoaderIcon className="motion-safe:animate-spin" />
        </StatusBannerIcon>
        <StatusBannerLabel muted>{label}</StatusBannerLabel>
      </StatusBannerRow>
      <StatusBannerProgress value={percent} />
    </StatusBanner>
  );
}
