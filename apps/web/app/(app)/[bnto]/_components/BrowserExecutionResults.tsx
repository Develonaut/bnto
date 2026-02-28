"use client";

import { CheckCircle2Icon, DownloadIcon } from "@/components/ui/icons";
import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { formatFileSize } from "@/src/utils/formatFileSize";

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
      <div className="flex min-w-0 flex-1 items-center gap-3">
        <div className="flex size-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary">
          <CheckCircle2Icon className="size-5" />
        </div>
        <div className="flex min-w-0 flex-1 flex-col">
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
        </div>
      </div>

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
