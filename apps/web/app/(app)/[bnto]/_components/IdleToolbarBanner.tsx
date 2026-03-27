"use client";

import { StatusBanner, StatusBannerLabel, StatusBannerProgress, StatusBannerRow } from "@bnto/ui";

/** Neutral banner shown before execution starts. Reserves toolbar height. */
export function IdleToolbarBanner() {
  return (
    <StatusBanner>
      <StatusBannerRow>
        <StatusBannerLabel muted>Ready to run</StatusBannerLabel>
      </StatusBannerRow>
      <StatusBannerProgress value={0} />
    </StatusBanner>
  );
}
