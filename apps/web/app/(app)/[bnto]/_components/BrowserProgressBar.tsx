"use client";

import { Row, Stack } from "@bnto/ui";
import type { BrowserExecution } from "@bnto/core";

type FileProgress = NonNullable<BrowserExecution["fileProgress"]>;

interface BrowserProgressBarProps {
  fileProgress: FileProgress;
}

/** Per-file progress bar with message and percentage label. */
export function BrowserProgressBar({ fileProgress }: BrowserProgressBarProps) {
  return (
    <Stack className="gap-1.5">
      <Row justify="between" className="text-xs text-muted-foreground">
        <span>{fileProgress.message}</span>
        <span>{fileProgress.overallPercent}%</span>
      </Row>
      <div className="h-1.5 overflow-hidden rounded-full bg-muted">
        <div
          data-testid="progress-bar"
          className="h-full rounded-full bg-primary motion-safe:transition-[width] motion-safe:duration-fast"
          style={{ width: `${fileProgress.overallPercent}%` }}
        />
      </div>
    </Stack>
  );
}
