"use client";

import {
  StatusBanner,
  StatusBannerIcon,
  StatusBannerLabel,
  StatusBannerProgress,
  StatusBannerRow,
  XCircleIcon,
} from "@bnto/ui";

export function FailedBanner({ error }: { error: string }) {
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
