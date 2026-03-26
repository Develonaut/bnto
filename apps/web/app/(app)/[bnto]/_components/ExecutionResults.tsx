"use client";

import { useCallback, useEffect } from "react";
import type { OutputFileUrl } from "@bnto/core";
import { core } from "@bnto/core";
import { ExecutionResultsBody } from "./ExecutionResultsBody";

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
  const { data: execution } = core.executions.useExecution(executionId);
  const { urls, fetchUrls, downloadFile, downloadAll, isLoading, isReady } =
    core.downloads.useDownloadFiles();

  const outputFiles = execution?.outputFiles ?? [];
  const isCompleted = execution?.status === "completed";
  const handleDownloadFile = useCallback(
    (url: OutputFileUrl) => () => downloadFile(url),
    [downloadFile],
  );
  const handleDownloadSingle = useCallback(() => downloadFile(urls[0]), [downloadFile, urls]);

  useEffect(() => {
    if (isCompleted && outputFiles.length > 0) fetchUrls(executionId);
  }, [isCompleted, outputFiles.length, executionId, fetchUrls]);

  if (!isCompleted || outputFiles.length === 0) return null;

  return (
    <ExecutionResultsBody
      outputFiles={outputFiles}
      execution={execution}
      urls={urls}
      isReady={isReady}
      isLoading={isLoading}
      onDownloadFile={handleDownloadFile}
      onDownloadSingle={handleDownloadSingle}
      onDownloadAll={downloadAll}
    />
  );
}
