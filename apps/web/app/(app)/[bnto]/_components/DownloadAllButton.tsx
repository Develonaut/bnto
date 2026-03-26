"use client";

import { Button, FileDownIcon, LoaderIcon } from "@bnto/ui";

interface DownloadAllButtonProps {
  fileCount: number;
  isReady: boolean;
  isLoading: boolean;
  onDownloadAll: () => void;
}

/**
 * "Download All" button for multi-file execution results.
 *
 * Shown when execution produces more than one output file.
 */
export function DownloadAllButton({
  fileCount,
  isReady,
  isLoading,
  onDownloadAll,
}: DownloadAllButtonProps) {
  return (
    <Button
      variant="outline"
      className="w-full"
      onClick={onDownloadAll}
      disabled={!isReady || isLoading}
      data-testid="download-all-button"
    >
      {isLoading ? (
        <LoaderIcon className="size-4 motion-safe:animate-spin" />
      ) : (
        <FileDownIcon className="size-4" />
      )}
      Download All ({fileCount} files)
    </Button>
  );
}
