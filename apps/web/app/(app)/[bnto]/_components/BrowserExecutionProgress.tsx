"use client";

import { ClockIcon, LoaderIcon, Row, Stack } from "@bnto/ui";
import type { BrowserExecution } from "@bnto/core";
import { useElapsedTime, formatElapsed } from "../_hooks/useElapsedTime";

interface BrowserExecutionProgressProps {
  execution: BrowserExecution;
}

/**
 * Progress display for browser-side WASM execution.
 *
 * Shows per-file progress with a progress bar, elapsed time,
 * and status messages from the Web Worker.
 */
export function BrowserExecutionProgress({ execution }: BrowserExecutionProgressProps) {
  const elapsed = useElapsedTime(execution.startedAt, execution.status === "processing");
  const { fileProgress } = execution;

  return (
    <Stack
      className="gap-3 rounded-lg border border-border bg-card p-4"
      data-testid="browser-execution-progress"
      data-status={execution.status}
      data-file-index={fileProgress?.fileIndex}
      data-total-files={fileProgress?.totalFiles}
      data-overall-percent={fileProgress?.overallPercent}
    >
      <Row justify="between">
        <Row gap="xs">
          <LoaderIcon className="size-5 shrink-0 text-primary motion-safe:animate-spin" />
          <p className="text-sm font-medium text-foreground">
            {fileProgress
              ? `Processing file ${fileProgress.fileIndex + 1} of ${fileProgress.totalFiles}\u2026`
              : "Initializing engine\u2026"}
          </p>
        </Row>
        <Row className="gap-1.5 text-xs text-muted-foreground">
          <ClockIcon className="size-3.5" />
          <span>{formatElapsed(elapsed)}</span>
        </Row>
      </Row>

      {fileProgress && (
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
      )}
    </Stack>
  );
}
