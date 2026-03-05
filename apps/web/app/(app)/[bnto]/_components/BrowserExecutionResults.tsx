"use client";

import { Button, Card, CheckCircle2Icon, DownloadIcon, IconBadge, Row, Stack } from "@bnto/ui";
import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { formatFileSize } from "@bnto/ui";

interface BrowserExecutionResultsProps {
  execution: BrowserExecution;
  onDownload: (result: BrowserFileResult) => void;
}

/** Displays browser execution output files matching the Phase 2 file list style. */
export function BrowserExecutionResults({
  execution,
  onDownload,
}: BrowserExecutionResultsProps) {
  const { results } = execution;
  if (results.length === 0) return null;

  return (
    <div
      className={
        results.length === 1
          ? "grid grid-cols-1"
          : results.length === 2
            ? "grid grid-cols-1 gap-2 sm:grid-cols-2"
            : "grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3"
      }
      data-testid="browser-execution-results"
    >
      {results.map((result, index) => (
        <BrowserFileRow
          key={`${result.filename}-${index}`}
          result={result}
          onDownload={onDownload}
        />
      ))}
    </div>
  );
}

function BrowserFileRow({
  result,
  onDownload,
}: {
  result: BrowserFileResult;
  onDownload: (result: BrowserFileResult) => void;
}) {
  const originalSize = result.metadata.originalSize as number | undefined;
  const ratio = result.metadata.ratio as number | undefined;

  return (
    <Card
      className="flex items-center gap-3 rounded-lg px-4 py-3"
      elevation="sm"
      data-testid="output-file"
    >
      <Row className="min-w-0 flex-1 gap-3">
        <IconBadge variant="primary" size="lg">
          <CheckCircle2Icon className="size-5" />
        </IconBadge>
        <Stack className="min-w-0 flex-1 gap-0">
          <span className="truncate text-sm font-semibold">{result.filename}</span>
          <span className="truncate text-xs text-muted-foreground">
            {formatFileSize(result.blob.size)}
            {originalSize != null && ratio != null && (
              <span>
                {" "}
                &middot; {Math.round((1 - ratio) * 100)}% smaller
              </span>
            )}
          </span>
        </Stack>
      </Row>

      <Button
        variant="outline"
        size="icon"
        elevation="sm"
        onClick={() => onDownload(result)}
        aria-label={`Download ${result.filename}`}
      >
        <DownloadIcon className="size-4" />
      </Button>
    </Card>
  );
}
