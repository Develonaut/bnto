"use client";

import { DownloadIcon, FileArchiveIcon, CheckCircle2Icon } from "@/components/ui/icons";
import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { Button } from "@/components/ui/Button";
import { formatFileSize } from "@/src/utils/formatFileSize";

interface BrowserExecutionResultsProps {
  execution: BrowserExecution;
  onDownload: (result: BrowserFileResult) => void;
  onDownloadAll: () => void;
}

/** Displays browser execution output files with download controls. */
export function BrowserExecutionResults({
  execution,
  onDownload,
  onDownloadAll,
}: BrowserExecutionResultsProps) {
  const { results } = execution;
  if (results.length === 0) return null;

  const durationMs =
    execution.completedAt && execution.startedAt
      ? execution.completedAt - execution.startedAt
      : null;

  return (
    <div
      className="space-y-3 rounded-lg border border-border bg-card p-4"
      data-testid="browser-execution-results"
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <CheckCircle2Icon className="size-5 shrink-0 text-green-600 dark:text-green-400" />
          <p className="text-sm font-medium text-foreground">
            {results.length} {results.length === 1 ? "file" : "files"} ready
          </p>
        </div>
        {durationMs !== null && (
          <p className="text-xs text-muted-foreground">
            Completed in {formatDuration(durationMs)}
          </p>
        )}
      </div>

      <ul className="space-y-2">
        {results.map((result, index) => (
          <BrowserFileRow
            key={`${result.filename}-${index}`}
            result={result}
            onDownload={onDownload}
          />
        ))}
      </ul>

      {results.length > 1 && (
        <Button
          variant="outline"
          className="w-full"
          onClick={onDownloadAll}
          data-testid="download-all-button"
        >
          <FileArchiveIcon className="size-4" />
          Download All as ZIP ({results.length} files)
        </Button>
      )}

      {results.length === 1 && (
        <Button
          className="w-full"
          onClick={() => onDownload(results[0])}
          data-testid="download-button"
        >
          <DownloadIcon className="size-4" />
          Download
        </Button>
      )}
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
    <li
      className="flex items-center justify-between gap-3 rounded-lg border border-border bg-background px-4 py-3"
      data-testid="output-file"
    >
      <div className="min-w-0 flex-1">
        <p className="truncate text-sm font-medium text-foreground">
          {result.filename}
        </p>
        <p className="text-xs text-muted-foreground">
          {formatFileSize(result.blob.size)}
          {originalSize != null && ratio != null && (
            <span>
              {" "}
              &middot; {Math.round((1 - ratio) * 100)}% smaller
            </span>
          )}
        </p>
      </div>

      <Button
        variant="outline"
        size="icon-sm"
        onClick={() => onDownload(result)}
        aria-label={`Download ${result.filename}`}
      >
        <DownloadIcon className="size-4" />
      </Button>
    </li>
  );
}

function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`;
  const seconds = (ms / 1000).toFixed(1);
  return `${seconds}s`;
}
