"use client";

import { Stack } from "@bnto/ui";
import type { OutputFileUrl } from "@bnto/core";
import { OutputFileList } from "./OutputFileList";
import { DownloadFooter } from "./DownloadFooter";
import { ExecutionResultsHeader } from "./ExecutionResultsHeader";

interface ExecutionResultsBodyProps {
  outputFiles: { key: string; name: string; sizeBytes: number }[];
  execution: { startedAt?: number; completedAt?: number } | undefined;
  urls: OutputFileUrl[];
  isReady: boolean;
  isLoading: boolean;
  onDownloadFile: (url: OutputFileUrl) => () => void;
  onDownloadSingle: () => void;
  onDownloadAll: () => void;
}

/** Render body for execution results — file list, download controls. */
export function ExecutionResultsBody({
  outputFiles,
  execution,
  urls,
  isReady,
  isLoading,
  onDownloadFile,
  onDownloadSingle,
  onDownloadAll,
}: ExecutionResultsBodyProps) {
  return (
    <Stack
      className="gap-3 rounded-lg border border-border bg-card p-4"
      data-testid="execution-results"
    >
      <ExecutionResultsHeader fileCount={outputFiles.length} execution={execution} />
      <OutputFileList files={outputFiles} urls={urls} onDownloadFile={onDownloadFile} />
      <DownloadFooter
        fileCount={outputFiles.length}
        urls={urls}
        isReady={isReady}
        isLoading={isLoading}
        onDownloadSingle={onDownloadSingle}
        onDownloadAll={onDownloadAll}
      />
    </Stack>
  );
}
