"use client";

import { useCallback } from "react";
import { Button, Text, DownloadIcon, CheckCircle2Icon, XCircleIcon, LoaderIcon } from "@bnto/ui";
import type { BrowserFileResult } from "@bnto/core";
import type { ExecutionPhase } from "../../hooks/useEditorExecution";

interface ResultsTabProps {
  phase: ExecutionPhase;
  results: BrowserFileResult[];
  errors: string[];
  onDownloadFile: (file: BrowserFileResult) => void;
  onDownloadAll: () => void;
}

/**
 * ResultsTab — shows execution output files and errors.
 *
 * Idle: prompt to run. Running: spinner. Completed: file list with
 * download buttons. Failed: error messages.
 */
function ResultsTab({ phase, results, errors, onDownloadFile, onDownloadAll }: ResultsTabProps) {
  if (phase === "idle") {
    return (
      <div className="flex h-full items-center justify-center p-4">
        <Text size="xs" color="muted">
          Run a recipe to see results.
        </Text>
      </div>
    );
  }

  if (phase === "running") {
    return (
      <div className="flex h-full items-center justify-center gap-2 p-4">
        <LoaderIcon className="size-4 motion-safe:animate-spin text-muted-foreground" />
        <Text size="xs" color="muted">
          Running...
        </Text>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col overflow-y-auto">
      {errors.length > 0 && (
        <div className="flex flex-col gap-1 border-b border-border p-3">
          {errors.map((error, i) => (
            <div key={i} className="flex items-start gap-2">
              <XCircleIcon className="mt-0.5 size-3.5 shrink-0 text-destructive" />
              <Text size="xs" className="text-destructive">
                {error}
              </Text>
            </div>
          ))}
        </div>
      )}
      {results.length > 0 && (
        <>
          <div className="flex flex-col gap-0.5 p-2">
            {results.map((file, i) => (
              <ResultRow key={i} file={file} onDownload={onDownloadFile} />
            ))}
          </div>
          {results.length > 1 && (
            <div className="border-t border-border p-2">
              <Button size="sm" variant="outline" className="w-full" onClick={onDownloadAll}>
                <DownloadIcon className="size-3.5" />
                Download All ({results.length})
              </Button>
            </div>
          )}
        </>
      )}
      {results.length === 0 && errors.length === 0 && (
        <div className="flex h-full items-center justify-center p-4">
          <Text size="xs" color="muted">
            No output files.
          </Text>
        </div>
      )}
    </div>
  );
}

function ResultRow({
  file,
  onDownload,
}: {
  file: BrowserFileResult;
  onDownload: (file: BrowserFileResult) => void;
}) {
  const handleClick = useCallback(() => onDownload(file), [file, onDownload]);
  const sizeKb = (file.blob.size / 1024).toFixed(1);

  return (
    <button
      type="button"
      onClick={handleClick}
      className="flex items-center gap-2 rounded-md px-2 py-1.5 text-left hover:bg-muted/50"
    >
      <CheckCircle2Icon className="size-3.5 shrink-0 text-primary" />
      <div className="min-w-0 flex-1">
        <Text size="xs" className="truncate font-medium">
          {file.filename}
        </Text>
        <Text size="xs" color="muted">
          {sizeKb} KB
        </Text>
      </div>
      <DownloadIcon className="size-3.5 shrink-0 text-muted-foreground" />
    </button>
  );
}

export { ResultsTab };
