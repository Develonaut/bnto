"use client";

import { CheckCircle2Icon, Row } from "@bnto/ui";

interface ExecutionResultsHeaderProps {
  fileCount: number;
  execution: { startedAt?: number; completedAt?: number } | undefined;
}

/** Header row showing file count and optional execution duration. */
export function ExecutionResultsHeader({ fileCount, execution }: ExecutionResultsHeaderProps) {
  const duration =
    execution?.completedAt && execution?.startedAt
      ? Math.round((execution.completedAt - execution.startedAt) / 1000)
      : null;

  return (
    <Row justify="between">
      <Row className="gap-2">
        <CheckCircle2Icon className="size-5 shrink-0 text-success" />
        <p className="text-sm font-medium text-foreground">
          {fileCount} {fileCount === 1 ? "file" : "files"} ready
        </p>
      </Row>
      {duration !== null && (
        <p className="text-xs text-muted-foreground">Completed in {duration}s</p>
      )}
    </Row>
  );
}
