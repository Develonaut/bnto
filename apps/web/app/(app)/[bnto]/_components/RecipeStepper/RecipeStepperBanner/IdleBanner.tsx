"use client";

import { StatusBanner, StatusBannerLabel, StatusBannerProgress, StatusBannerRow } from "@bnto/ui";

export function IdleBanner() {
  return (
    <StatusBanner>
      <StatusBannerRow>
        <StatusBannerLabel muted>Ready to run</StatusBannerLabel>
      </StatusBannerRow>
      <StatusBannerProgress value={0} />
    </StatusBanner>
  );
}
