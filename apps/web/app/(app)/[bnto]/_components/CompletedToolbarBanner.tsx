"use client";

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

interface CompletedToolbarBannerProps {
  count: number;
  saved: number;
}

/** Success banner shown in the toolbar after all files are processed. */
export function CompletedToolbarBanner({ count, saved }: CompletedToolbarBannerProps) {
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
