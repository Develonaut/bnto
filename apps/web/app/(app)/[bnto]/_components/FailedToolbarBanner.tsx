"use client";

import {
  StatusBanner,
  StatusBannerIcon,
  StatusBannerLabel,
  StatusBannerProgress,
  StatusBannerRow,
  XCircleIcon,
} from "@bnto/ui";

interface FailedToolbarBannerProps {
  error: string;
}

/** Error banner shown in the toolbar when execution fails. */
export function FailedToolbarBanner({ error }: FailedToolbarBannerProps) {
  return (
    <StatusBanner variant="error">
      <StatusBannerRow>
        <StatusBannerIcon>
          <XCircleIcon />
        </StatusBannerIcon>
        <StatusBannerLabel>{error}</StatusBannerLabel>
      </StatusBannerRow>
      <StatusBannerProgress value={100} variant="error" />
    </StatusBanner>
  );
}
