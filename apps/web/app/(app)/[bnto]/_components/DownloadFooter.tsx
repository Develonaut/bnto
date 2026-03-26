"use client";

import { Button, DownloadIcon } from "@bnto/ui";
import type { OutputFileUrl } from "@bnto/core";
import { DownloadAllButton } from "./DownloadAllButton";

interface DownloadFooterProps {
  fileCount: number;
  urls: OutputFileUrl[];
  isReady: boolean;
  isLoading: boolean;
  onDownloadSingle: () => void;
  onDownloadAll: () => void;
}

/** Download controls -- "Download All" for multi-file, single button for one file. */
export function DownloadFooter({
  fileCount,
  urls,
  isReady,
  isLoading,
  onDownloadSingle,
  onDownloadAll,
}: DownloadFooterProps) {
  if (fileCount > 1) {
    return (
      <DownloadAllButton
        fileCount={fileCount}
        isReady={isReady}
        isLoading={isLoading}
        onDownloadAll={onDownloadAll}
      />
    );
  }
  if (fileCount === 1 && isReady && urls.length > 0) {
    return (
      <Button className="w-full" onClick={onDownloadSingle} data-testid="download-button">
        <DownloadIcon className="size-4" />
        Download
      </Button>
    );
  }
  return null;
}
