"use client";

import type { BrowserExecution } from "@bnto/core";
import { computeTotalSaved } from "@bnto/core";
import {
  CheckCircle2Icon,
  StatusBanner,
  StatusBannerIcon,
  StatusBannerLabel,
  StatusBannerProgress,
  StatusBannerRow,
  StatusBannerSpacer,
  formatFileSize,
} from "@bnto/ui";

export function CompletedBanner({ execution }: { execution: BrowserExecution }) {
  const count = execution.results.length;
  const saved = computeTotalSaved(execution.results);
  return (
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
  );
}
