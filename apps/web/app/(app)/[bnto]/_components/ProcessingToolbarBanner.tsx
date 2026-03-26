"use client";

import {
  LoaderIcon,
  StatusBanner,
  StatusBannerIcon,
  StatusBannerLabel,
  StatusBannerProgress,
  StatusBannerRow,
} from "@bnto/ui";
import type { BrowserExecution } from "@bnto/core";

interface ProcessingToolbarBannerProps {
  fileProgress: BrowserExecution["fileProgress"];
}

/** In-progress banner shown in the toolbar while processing files. */
export function ProcessingToolbarBanner({ fileProgress }: ProcessingToolbarBannerProps) {
  const percent = fileProgress?.overallPercent ?? 0;
  const label = fileProgress
    ? `Processing file ${fileProgress.fileIndex + 1} of ${fileProgress.totalFiles}...`
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
