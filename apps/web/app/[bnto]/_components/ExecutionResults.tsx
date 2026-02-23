"use client";

import { useEffect } from "react";
import { Download, FileDown, Loader2, CheckCircle2 } from "lucide-react";
import { useExecution, useDownloadFiles } from "@bnto/core";
import { Button } from "@/components/ui/button";
import { formatFileSize } from "@/src/utils/formatFileSize";

interface ExecutionResultsProps {
  executionId: string;
}

/**
 * Displays execution output files with download controls.
 *
 * Automatically fetches presigned download URLs when the execution completes.
 * Shows individual file downloads and a "Download All" button.
 */
export function ExecutionResults({ executionId }: ExecutionResultsProps) {
  const { data: execution } = useExecution(executionId);
  const { urls, fetchUrls, downloadFile, downloadAll, isLoading, isReady } =
    useDownloadFiles();

  const outputFiles = execution?.outputFiles ?? [];
  const hasOutputFiles = outputFiles.length > 0;
  const isCompleted = execution?.status === "completed";
  const duration = execution?.completedAt && execution?.startedAt
    ? Math.round((execution.completedAt - execution.startedAt) / 1000)
    : null;

  useEffect(() => {
    if (isCompleted && hasOutputFiles) {
      fetchUrls(executionId);
    }
  }, [isCompleted, hasOutputFiles, executionId, fetchUrls]);

  if (!isCompleted || !hasOutputFiles) return null;

  return (
    <div
      className="space-y-3 rounded-lg border border-border bg-card p-4"
      data-testid="execution-results"
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <CheckCircle2 className="size-5 shrink-0 text-green-600 dark:text-green-400" />
          <p className="text-sm font-medium text-foreground">
            {outputFiles.length}{" "}
            {outputFiles.length === 1 ? "file" : "files"} ready
          </p>
        </div>
        {duration !== null && (
          <p className="text-xs text-muted-foreground">
            Completed in {duration}s
          </p>
        )}
      </div>

      <ul className="space-y-2">
        {outputFiles.map((file) => {
          const downloadUrl = urls.find((u) => u.key === file.key);
          return (
            <li
              key={file.key}
              className="flex items-center justify-between gap-3 rounded-lg border border-border bg-background px-4 py-3"
            >
              <div className="min-w-0 flex-1">
                <p className="truncate text-sm font-medium text-foreground">
                  {file.name}
                </p>
                <p className="text-xs text-muted-foreground">
                  {formatFileSize(file.sizeBytes)}
                </p>
              </div>

              {downloadUrl ? (
                <Button
                  variant="outline"
                  size="icon-sm"
                  onClick={() => downloadFile(downloadUrl)}
                  aria-label={`Download ${file.name}`}
                >
                  <Download className="size-4" />
                </Button>
              ) : (
                <Loader2 className="size-4 shrink-0 text-muted-foreground motion-safe:animate-spin" />
              )}
            </li>
          );
        })}
      </ul>

      {outputFiles.length > 1 && (
        <Button
          variant="outline"
          className="w-full"
          onClick={downloadAll}
          disabled={!isReady || isLoading}
          data-testid="download-all-button"
        >
          {isLoading ? (
            <Loader2 className="size-4 motion-safe:animate-spin" />
          ) : (
            <FileDown className="size-4" />
          )}
          Download All ({outputFiles.length} files)
        </Button>
      )}

      {outputFiles.length === 1 && isReady && urls.length > 0 && (
        <Button
          className="w-full"
          onClick={() => downloadFile(urls[0])}
          data-testid="download-button"
        >
          <Download className="size-4" />
          Download
        </Button>
      )}
    </div>
  );
}
